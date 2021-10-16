#![allow(dead_code)]

use std::{f32::consts::PI, fmt::Debug, ops::Range};

use bevy::{
    prelude::*,
    render::{pipeline::RenderPipeline, render_graph::base, shader},
};
use smallvec::SmallVec;

mod gen;
mod line;
mod material;
mod mesh_helper;
mod render_graph;

use line::Line;
pub use material::GizmoMaterial;

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone)]
pub enum GizmoShape {
    /// Similar to blender empty axis, his solid shape is a octahedron
    Empty {
        radius: f32,
    },
    /// **NOTE** Billboard is the only gizmos that doesn't have a wireframe version
    Billboard {
        texture: Option<Handle<Texture>>,
        size: f32,
    },
    Cube {
        size: Vec3,
    },
    Circle {
        radius: f32,
    },
    Sphere {
        radius: f32,
    },
    Hemisphere {
        radius: f32,
    },
    Cylinder {
        radius: f32,
        height: f32,
    },
    Capsule {
        radius: f32,
        /// Height of the cylindrical portion, the total height is given by `height + 2.0 * radius`
        height: f32,
        /// Capsule axis orientation
        axis: Axis,
    },
    Mesh {
        mesh: Handle<Mesh>,
    },
}

/// Persistent gizmo component;
///
/// **NOTE** Removing this component won't remove the gizmos, thus
/// use the `GizmoBundle` to spawn a new gizmo as a child of the entity you want
/// to put the gizmo on;
#[derive(Debug, Reflect, Component)]
#[reflect(Component)]
pub struct Gizmo {
    #[reflect(ignore)]
    pub shape: GizmoShape,
    pub wireframe: Color,
    /// **NOTE** Not every gizmo has a filled shape, so this might be ignored
    pub color: Color,
}

impl Default for Gizmo {
    fn default() -> Self {
        Self {
            shape: GizmoShape::Cube {
                size: Vec3::new(1.0, 1.0, 1.0),
            },
            wireframe: Color::WHITE,
            color: Color::rgba_linear(0.0, 0.0, 0.0, 0.0),
        }
    }
}

#[derive(Default, Bundle)]
pub struct GizmoBundle {
    pub global_transform: GlobalTransform,
    pub transform: Transform,
    pub children: Children,
    pub gizmo: Gizmo,
}

/// The gizmo may use multiple [`GizmosMeshBundles`] to render it self
#[derive(Bundle)]
pub(crate) struct GizmoMeshBundle {
    pub mesh: Handle<Mesh>,
    pub main_pass: base::MainPass, // TODO: GizmoPass
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub material: GizmoMaterial,
}

impl Default for GizmoMeshBundle {
    fn default() -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                render_graph::GIZMOS_PIPELINE_HANDLE.typed(),
            )]),
            mesh: Default::default(),
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            main_pass: Default::default(),
            draw: Default::default(),
            transform: Default::default(),
            global_transform: Default::default(),
            material: Default::default(),
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

/// Defines a single frame gizmo command
pub enum GizmoCommand {
    Shape {
        transform: Transform,
        shape: GizmoShape,
        duration: f32,
        color: Color,
        wireframe: Color,
    },
    // TODO: Mesh, rendered with a custom wireframe material
    LineList {
        // TODO: Having a long set of points will allocate memory every frame,
        // having multiple `LineList` also allocate memory, because each command
        // are kept as a SegQueue node that lives in the heap;
        //
        // Pick your poison until a better solution come around, just go with
        // the more convenient solution for your problem;
        points: SmallVec<[Vec3; 4]>,
        duration: f32,
        color: Color,
    },
}

pub struct Gizmos {
    /// Control which set of gizmos it will draw
    pub mask: u32,
    commands: crossbeam::queue::SegQueue<GizmoCommand>,
}

impl Default for Gizmos {
    fn default() -> Self {
        Gizmos {
            mask: u32::MAX,
            commands: Default::default(),
        }
    }
}

impl Gizmos {
    /// Begins a draw group, each group have its one mask that can
    /// switch on and off using any gizmos code, use [`Gizmos.mask`]
    /// to control it;
    #[inline]
    pub fn draw(&self, mask: u32, scope: impl FnOnce(GizmosContext)) -> &Self {
        if (mask & self.mask) != 0 {
            (scope)(GizmosContext::new(self));
        }
        self
    }
}

// TODO: Will be wholesome if we could select each gizmos like if they where a button

pub struct GizmosContext<'a> {
    color: Color,
    wireframe: Color,
    stack: Vec<Transform>,
    command_buffer: &'a Gizmos,
}

impl<'a> GizmosContext<'a> {
    fn new(command_buffer: &'a Gizmos) -> Self {
        Self {
            color: Color::rgba_linear(0.0, 0.0, 0.0, 0.0),
            wireframe: Color::WHITE,
            stack: vec![],
            command_buffer,
        }
    }

    #[inline]
    pub fn push_matrix(&mut self, transform: Transform) -> &mut Self {
        self.stack.push(transform);
        self
    }

    #[inline]
    pub fn pop_matrix(&mut self) -> &mut Self {
        self.stack.pop();
        self
    }

    #[inline]
    pub fn with_color(&mut self, color: Color) -> &mut Self {
        self.color = color;
        self
    }

    #[inline]
    pub fn no_color(&mut self) -> &mut Self {
        self.color = Color::rgba_linear(0.0, 0.0, 0.0, 0.0);
        self
    }

    #[inline]
    pub fn with_wireframe(&mut self, color: Color) -> &mut Self {
        self.wireframe = color;
        self
    }

    #[inline]
    pub fn no_wireframe(&mut self) -> &mut Self {
        self.wireframe = Color::rgba_linear(0.0, 0.0, 0.0, 0.0);
        self
    }

    pub fn shape(&mut self, shape: GizmoShape, duration: f32) -> &mut Self {
        self.command(GizmoCommand::Shape {
            transform: self.stack.last().copied().unwrap_or_default(),
            shape,
            duration,
            wireframe: self.wireframe,
            color: self.color,
        })
    }

    pub fn line_list(
        &mut self,
        points: impl Into<SmallVec<[Vec3; 4]>>,
        duration: f32,
    ) -> &mut Self {
        let mut points = points.into();

        // Transform points before pushing the line list
        if let Some(transform) = self.stack.last() {
            points.iter_mut().for_each(|p| *p = transform.mul_vec3(*p));
        }

        self.command(GizmoCommand::LineList {
            points,
            duration,
            color: self.wireframe,
        })
    }

    /// **NOTE** Pushes a raw command, ignoring the current transform matrix
    #[inline]
    pub fn command(&mut self, gizmo: GizmoCommand) -> &mut Self {
        self.command_buffer.commands.push(gizmo);
        self
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct GizmosMeshes {
    /// Flag this meshes as wireframe
    wireframe: bool,
    mesh_empty: Handle<Mesh>,
    mesh_billboard: Handle<Mesh>,
    mesh_cube: Handle<Mesh>,
    mesh_sphere: Handle<Mesh>,
    mesh_hemisphere: Handle<Mesh>,
    mesh_cylinder: Handle<Mesh>,
    mesh_capsule_body: Handle<Mesh>,
    mesh_capsule_cap: Handle<Mesh>, // Similar to hemisphere but with less redundant lines
}

#[derive(Default)]
struct GizmosResources {
    meshes: GizmosMeshes,
    meshes_wireframe: GizmosMeshes,
    // TODO: Hashmap of instantiated gizmos to later delete them?
    // instances: HashMap<Entity, SmallVec<[Entity; 4]>>,

    // Gizmos command buffer
    /// Volatile gizmos shapes
    shapes_volatile_tracker: Vec<(f32, Entity)>,
    /// Not quite immediate mode but they will disappear eventually,
    /// it's particular hard to manage these lines because they share a single
    /// mesh
    lines_volatile: Line,
    lines_volatile_tracker: Vec<(f32, Range<usize>, Range<usize>)>,
    /// This set of lines will only be active once per frame which
    /// make their management way cheaper;
    lines_immediate: Line,
}

fn gizmos_setup(
    mut commands: Commands,
    mut gizmos: ResMut<GizmosResources>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let meshes = &mut *meshes;

    gizmos.meshes_wireframe.wireframe = true;
    gizmos.meshes_wireframe.mesh_empty = meshes.add(gen::wire_empty());
    //gizmos.meshes_wireframe.mesh_billboard = ...; // Empty
    gizmos.meshes_wireframe.mesh_cube = meshes.add(gen::wire_cube());
    gizmos.meshes_wireframe.mesh_sphere = meshes.add(gen::wire_sphere());
    gizmos.meshes_wireframe.mesh_hemisphere = meshes.add(gen::wire_hemisphere());
    gizmos.meshes_wireframe.mesh_cylinder = meshes.add(gen::wire_cylinder());
    gizmos.meshes_wireframe.mesh_capsule_body = gizmos.meshes_wireframe.mesh_cylinder.clone();
    gizmos.meshes_wireframe.mesh_capsule_cap = meshes.add(gen::wire_capsule_cap());

    gizmos.meshes.wireframe = false;
    gizmos.meshes.mesh_empty = meshes.add(gen::empty());
    gizmos.meshes.mesh_billboard = meshes.add(gen::billboard());
    gizmos.meshes.mesh_cube = meshes.add(gen::cube());
    gizmos.meshes.mesh_sphere = meshes.add(gen::sphere());
    gizmos.meshes.mesh_hemisphere = meshes.add(gen::hemisphere());
    // gizmos.meshes.mesh_cylinder = meshes.add(gen::cylinder());
    gizmos.meshes.mesh_capsule_body = meshes.add(gen::capsule_body());
    gizmos.meshes.mesh_capsule_cap = gizmos.meshes.mesh_hemisphere.clone();

    // Shared line mesh
    gizmos.lines_volatile = Line::new(&mut commands, meshes);
    gizmos.lines_immediate = Line::new(&mut commands, meshes);
}

fn gizmos_update_system(
    mut commands: Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gizmos: ResMut<GizmosResources>,
    gizmos_command_buffer: ResMut<Gizmos>,
    gizmos_query: Query<(Entity, &Gizmo, &Children), (Changed<Gizmo>,)>,
    // gizmos_removed_query: Query<Entity, Without<Gizmo>>,
) {
    // TODO: Deal with removed components
    // // Despawns removed gizmos
    // for entity in gizmos_removed_query.removed::<remove() {
    //     if let Some(instances) = gizmos.instances.remove(entity) {
    //         for entity in instances {
    //             commands.despawn(entity);
    //         }
    //     }
    // }

    let gizmos = &mut *gizmos;
    let meshes = &mut *meshes;

    // Manage gizmos components by adding entities to render them
    for (entity, gizmo, children) in &mut gizmos_query.iter() {
        // Remove children
        children.iter().copied().for_each(|entity| {
            commands.entity(entity).despawn();
        });

        if gizmo.wireframe.a() > f32::EPSILON {
            gizmo_instantiate(
                &mut commands,
                &gizmos.meshes_wireframe,
                entity,
                gizmo.shape.clone(),
                gizmo.wireframe,
            );
        }

        if gizmo.color.a() > f32::EPSILON {
            gizmo_instantiate(
                &mut commands,
                &gizmos.meshes,
                entity,
                gizmo.shape.clone(),
                gizmo.color,
            );
        }
    }

    // Manage previous volatile gizmos

    // Manage gizmos shapes
    // TODO: Recycle entities to improve performance
    for i in (0..gizmos.shapes_volatile_tracker.len()).rev() {
        let (time_left, _) = &mut gizmos.shapes_volatile_tracker[i];

        if *time_left < 0.0 {
            let (_, entity) = gizmos.shapes_volatile_tracker.remove(i);
            commands.entity(entity).despawn_recursive();
        } else {
            *time_left -= time.delta_seconds();
        }
    }

    let mut lines_immediate_edit: mesh_helper::MeshEditXC = {
        // SAFETY: This mesh is fetched only here,
        // further more the `meshes` won't mutate only his meshes
        let meshes = unsafe { &mut *(meshes as *mut _) };
        gizmos.lines_immediate.edit(meshes)
    };

    // Clear right away the immediate mode lines since they are just one frame
    lines_immediate_edit.vertices.clear();
    lines_immediate_edit.colors.clear();
    lines_immediate_edit.indices.clear();

    let mut lines_volatile_edit: Option<mesh_helper::MeshEditXC> = None;

    // Manage volatile lines
    for i in (0..gizmos.lines_volatile_tracker.len()).rev() {
        let (time_left, _, _) = &mut gizmos.lines_volatile_tracker[i];

        if *time_left < 0.0 {
            let (_, v_range, i_range) = gizmos.lines_volatile_tracker.remove(i);

            // Remove lines
            let edit = lines_volatile_edit.get_or_insert_with(|| {
                // SAFETY: This mesh is fetched once,
                // further more the `meshes` won't mutate only his meshes
                let meshes = unsafe { &mut *(meshes as *mut _) };
                gizmos.lines_volatile.edit(meshes)
            });

            // ? NOTE: This algorithm will reduce the amount of memory that needs to be sended over to the GPU
            // ? and also reduce memory fragmentation, although having to move data around quite a bit
            // Remove vertex attributes
            edit.vertices.drain(v_range.start..v_range.end);
            edit.colors.drain(v_range.start..v_range.end);

            let i_offset = i_range.end - i_range.start;
            let v_offset = v_range.end - v_range.start;

            // Move indexes over the removed role
            for i in i_range.end..edit.indices.len() {
                let index = edit.indices[i];
                edit.indices[i - i_offset] = if index >= v_range.end as u32 {
                    index - v_offset as u32
                } else {
                    index
                }
            }
            // Make sure to keep all the indices pointing to the right vertices
            for i in 0..i_range.start {
                if edit.indices[i] >= v_range.end as u32 {
                    edit.indices[i] -= v_offset as u32;
                }
            }

            // Trim the left over
            edit.indices
                .resize_with(edit.indices.len() - i_offset, || unreachable!());

            // Offset each other volatile line to keep track the moved parts
            for (_, v, i) in gizmos.lines_volatile_tracker.iter_mut() {
                if v.start >= v_range.start {
                    v.start -= v_offset;
                    v.end -= v_offset;
                }
                if i.start >= i_range.start {
                    i.start -= i_offset;
                    i.end -= i_offset;
                }
            }
        } else {
            *time_left -= time.delta_seconds();
        }
    }

    while let Some(command) = gizmos_command_buffer.commands.pop() {
        match command {
            GizmoCommand::Shape {
                transform,
                shape,
                duration,
                color,
                wireframe,
            } => {
                // Spawn gizmo
                let entity = commands
                    .spawn()
                    .insert_bundle((transform, GlobalTransform::default(), Children::default()))
                    .id();

                let gizmo = Gizmo {
                    shape,
                    color,
                    wireframe,
                };

                if gizmo.wireframe.a() > f32::EPSILON {
                    gizmo_instantiate(
                        &mut commands,
                        &gizmos.meshes_wireframe,
                        entity,
                        gizmo.shape.clone(),
                        gizmo.wireframe,
                    );
                }

                if gizmo.color.a() > f32::EPSILON {
                    gizmo_instantiate(
                        &mut commands,
                        &gizmos.meshes,
                        entity,
                        gizmo.shape.clone(),
                        gizmo.color,
                    );
                }

                // Keep track
                gizmos.shapes_volatile_tracker.push((duration, entity));
            }
            GizmoCommand::LineList {
                points,
                duration,
                color,
            } => {
                // True if more than a single frame
                let volatile = duration > f32::EPSILON;

                // Add new lines
                let edit = if volatile {
                    lines_volatile_edit.get_or_insert_with(|| {
                        // SAFETY: This mesh is fetched once, further more the `meshes` won't mutate only his meshes
                        let meshes = unsafe { &mut *(meshes as *mut _) };
                        gizmos.lines_volatile.edit(meshes)
                    })
                } else {
                    &mut lines_immediate_edit
                };

                let v = edit.vertices.len();
                let inserted_points = points.len();

                // SAFETY: `Vec3` can be trivially interpreted as `[f32; 3]`, and transmute guarantees both
                // types have the same size so the `SmallVec` buffer will always have the right amount of points
                unsafe {
                    edit.vertices
                        .extend(std::mem::transmute::<_, SmallVec<[[f32; 3]; 4]>>(points));
                }

                edit.colors
                    .resize(edit.colors.len() + inserted_points, <[f32; 4]>::from(color));

                let i = edit.indices.len();
                for index in 0..(inserted_points - 1) {
                    edit.indices.push((v + index) as u32);
                    edit.indices.push((v + index) as u32 + 1);
                }

                if volatile {
                    // Keep track, but only if volatile
                    gizmos.lines_volatile_tracker.push((
                        duration,
                        Range {
                            start: v,
                            end: edit.vertices.len(),
                        },
                        Range {
                            start: i,
                            end: edit.indices.len(),
                        },
                    ));
                }
            }
        }
    }
}

/// Instantiates a gizmo mesh
fn gizmo_instantiate(
    commands: &mut Commands,
    gizmos: &GizmosMeshes,
    parent: Entity,
    gizmo_shape: GizmoShape,
    gizmo_color: Color,
) {
    let mut material = GizmoMaterial::from(gizmo_color);
    //material.lit = !gizmos.wireframe;

    match gizmo_shape {
        GizmoShape::Empty { radius } => {
            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform::from_scale(Vec3::splat(radius)),
                    mesh: gizmos.mesh_empty.clone(),
                    material,
                    ..Default::default()
                })
                .insert(Parent(parent));
        }
        GizmoShape::Billboard { texture, size } => {
            material.texture = texture;
            //material.lit = false;
            material.billboard = true;
            material.billboard_size = size;

            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform::default(),
                    mesh: gizmos.mesh_billboard.clone(),
                    material,
                    ..Default::default()
                })
                .insert(Parent(parent));
        }
        GizmoShape::Cube { size } => {
            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform::from_scale(size),
                    mesh: gizmos.mesh_cube.clone(),
                    material,
                    ..Default::default()
                })
                .insert(Parent(parent));
        }
        GizmoShape::Circle { radius } => {
            let _ = radius;
            todo!()
        }
        GizmoShape::Sphere { radius } => {
            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform::from_scale(Vec3::splat(radius)),
                    mesh: gizmos.mesh_sphere.clone(),
                    material,
                    ..Default::default()
                })
                .insert(Parent(parent));
        }
        GizmoShape::Hemisphere { radius } => {
            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform::from_scale(Vec3::splat(radius)),
                    mesh: gizmos.mesh_hemisphere.clone(),
                    material,
                    ..Default::default()
                })
                .insert(Parent(parent));
        }
        GizmoShape::Cylinder { radius, height } => {
            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform::from_scale(Vec3::new(radius, height, radius)),
                    mesh: gizmos.mesh_cylinder.clone(),
                    material,
                    ..Default::default()
                })
                .insert(Parent(parent));
        }
        GizmoShape::Capsule {
            radius,
            height,
            axis,
        } => {
            let mut top = Vec3::ZERO;
            let mut bottom = Vec3::ZERO;

            let offset = height * 0.5;
            let (rotation, rotation_mirrored) = match axis {
                Axis::X => {
                    top[0] = -offset;
                    bottom[0] = offset;
                    (
                        Quat::from_rotation_z(PI * 0.5),
                        Quat::from_euler(bevy::math::EulerRot::YXZ, PI, 0.0, PI * 0.5),
                    )
                }
                Axis::Y => {
                    top[1] = offset;
                    bottom[1] = -offset;
                    (Quat::default(), Quat::from_rotation_x(PI))
                }
                Axis::Z => {
                    top[2] = offset;
                    bottom[2] = -offset;
                    (
                        Quat::from_rotation_x(PI * 0.5),
                        Quat::from_euler(bevy::math::EulerRot::YXZ, 0.0, PI * 0.5, PI),
                    )
                }
            };

            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform {
                        translation: top,
                        rotation,
                        scale: Vec3::splat(radius),
                    },
                    mesh: gizmos.mesh_capsule_cap.clone(),
                    material: material.clone(),
                    ..Default::default()
                })
                .insert(Parent(parent));
            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform {
                        translation: Vec3::ZERO,
                        rotation,
                        scale: Vec3::new(radius, height, radius),
                    },
                    mesh: gizmos.mesh_capsule_body.clone(),
                    material: material.clone(),
                    ..Default::default()
                })
                .insert(Parent(parent));
            commands
                .spawn()
                .insert_bundle(GizmoMeshBundle {
                    transform: Transform {
                        translation: bottom,
                        rotation: rotation_mirrored,
                        scale: Vec3::splat(radius),
                    },
                    mesh: gizmos.mesh_capsule_cap.clone(),
                    material,
                    ..Default::default()
                })
                .insert(Parent(parent));
        }
        GizmoShape::Mesh { mesh } => {
            // Wireframe component ?!?
            let _ = mesh;
            todo!()
        }
    };
}

///////////////////////////////////////////////////////////////////////////////

// ? NOTE: Gizmos needs his own stage because it relays on commands to push out
// ? many of the pre-build gizmos
#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum GizmoStage {
    Update,
}

#[derive(Default)]
pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            GizmoStage::Update,
            SystemStage::parallel(),
        );

        app.register_type::<GizmoMaterial>().add_system_to_stage(
            GizmoStage::Update,
            shader::shader_defs_system::<GizmoMaterial>.system(),
        );

        app.insert_resource(Gizmos::default())
            .insert_resource(GizmosResources::default())
            .add_startup_system(gizmos_setup.system())
            .add_startup_system(render_graph::gizmos_pipeline_setup.system())
            .add_system_to_stage(GizmoStage::Update, gizmos_update_system.system());
    }
}
