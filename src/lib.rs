#![allow(dead_code)]

use std::{fmt::Debug, ops::Range};

use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        pipeline::{PrimitiveTopology, RenderPipeline},
        render_graph::base,
        shader,
    },
};
use smallvec::SmallVec;

mod gen;
mod mesh_helper;
pub mod rendering;

pub use rendering::GizmoMaterial;

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

// TODO: Implement reflect
#[derive(Debug, Clone)]
pub enum GizmoShape {
    /// Similar to blender empty axis
    Empty {
        radius: f32,
    },
    Billboard {
        texture: Option<Handle<Texture>>,
        size: f32,
    },
    Cube {
        size: Vec3,
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
    // TODO: Use the new Wireframe component from bevy master
    // Mesh {
    //     mesh: Handle<Mesh>,
    // },
}

/// Persistent gizmo component
#[derive(Debug, Reflect)]
#[reflect(Component)]
pub struct Gizmo {
    #[reflect(ignore)]
    pub shape: GizmoShape,
    pub color: Color,
}

impl Default for Gizmo {
    fn default() -> Self {
        Self {
            shape: GizmoShape::Cube {
                size: Vec3::new(1.0, 1.0, 1.0),
            },
            color: Color::WHITE,
        }
    }
}

#[derive(Default, Bundle)]
pub struct GizmoBundle {
    pub gizmo: Gizmo,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub children: Children,
}

#[derive(Bundle)]
struct GizmoMeshBundle {
    pub mesh: Handle<Mesh>,
    pub main_pass: base::MainPass, // TODO: GizmoPass
    pub draw: Draw,
    pub visible: Visible,
    pub render_pipelines: RenderPipelines,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub material: GizmoMaterial,
}

impl GizmoMeshBundle {
    fn new(transform: Transform, mesh: Handle<Mesh>, material: impl Into<GizmoMaterial>) -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                rendering::GIZMOS_PIPELINE_HANDLE.typed(),
            )]),
            mesh,
            visible: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform,
            global_transform: Default::default(),
            material: material.into(),
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

pub struct GizmosCommandBuffer {
    /// Control which set of gizmos it will draw
    pub mask: u32,
    commands: crossbeam::queue::SegQueue<GizmoCommand>,
}

impl Default for GizmosCommandBuffer {
    fn default() -> Self {
        GizmosCommandBuffer {
            mask: u32::MAX,
            commands: Default::default(),
        }
    }
}

impl GizmosCommandBuffer {
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
    stack: Vec<Transform>,
    command_buffer: &'a GizmosCommandBuffer,
}

impl<'a> GizmosContext<'a> {
    fn new(command_buffer: &'a GizmosCommandBuffer) -> Self {
        Self {
            color: Color::WHITE,
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

    pub fn shape(&mut self, shape: GizmoShape, duration: f32) -> &mut Self {
        self.command(GizmoCommand::Shape {
            transform: self.stack.last().copied().unwrap_or_default(),
            shape,
            duration,
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
            color: self.color,
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

enum GizmoVolatile {
    Line(Range<usize>, Range<usize>),
    Shape(Entity),
}

#[derive(Default)]
struct GizmosResources {
    mesh_empty: Handle<Mesh>,
    mesh_billboard: Handle<Mesh>,
    mesh_cube: Handle<Mesh>,
    mesh_sphere: Handle<Mesh>,
    mesh_hemisphere: Handle<Mesh>,
    mesh_cylinder: Handle<Mesh>,
    mesh_capsule_cap: Handle<Mesh>, // Similar to hemisphere but with less redundant lines

    // Gizmos command buffer
    volatile: Vec<(f32, GizmoVolatile)>,
    lines_entity: Option<Entity>,
    lines_mesh_handle: Handle<Mesh>,
}

fn gizmos_setup(
    commands: &mut Commands,
    mut gizmos: ResMut<GizmosResources>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    gizmos.mesh_empty = meshes.add(gen::wire_empty());
    gizmos.mesh_billboard = meshes.add(gen::billboard());
    gizmos.mesh_cube = meshes.add(gen::wire_cube());
    gizmos.mesh_sphere = meshes.add(gen::wire_sphere());
    gizmos.mesh_hemisphere = meshes.add(gen::wire_hemisphere());
    gizmos.mesh_cylinder = meshes.add(gen::wire_cylinder());
    gizmos.mesh_capsule_cap = meshes.add(gen::wire_capsule_cap());

    // TODO: Partition between multiple meshes when have culling in place
    // Shared line mesh
    gizmos.lines_mesh_handle = {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.set_attribute(Mesh::ATTRIBUTE_COLOR, Vec::<[f32; 4]>::with_capacity(32));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::with_capacity(32));
        mesh.set_indices(Some(Indices::U16(Vec::with_capacity(32))));
        meshes.add(mesh)
    };

    gizmos.lines_entity = commands
        .spawn(GizmoMeshBundle::new(
            Transform::default(),
            gizmos.lines_mesh_handle.clone(),
            Color::WHITE,
        ))
        .current_entity();
}

fn gizmos_update_system(
    commands: &mut Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gizmos: ResMut<GizmosResources>,
    gizmos_command_buffer: ResMut<GizmosCommandBuffer>,
    gizmos_query: Query<(Entity, &Gizmo, &Children), (Changed<Gizmo>,)>,
) {
    // TODO: Disable gizmos

    let gizmos = &mut *gizmos;

    // Manage gizmos components by adding entities to render them
    for (entity, gizmo, children) in &mut gizmos_query.iter() {
        // Remove children
        children.iter().copied().for_each(|entity| {
            commands.despawn(entity);
        });

        gizmo_instantiate(commands, gizmos, entity, gizmo);
    }

    // TODO: Recycle entities to improve performance

    let mut lines_mesh_edit: Option<mesh_helper::MeshEditXC> = None;

    // Clear previous gizmos
    for i in (0..gizmos.volatile.len()).rev() {
        let (time_left, _) = &mut gizmos.volatile[i];

        if *time_left < 0.0 {
            let (_, item) = gizmos.volatile.remove(i);
            match item {
                GizmoVolatile::Line(vert, index) => {
                    // Remove lines
                    let edit = lines_mesh_edit.get_or_insert_with(|| {
                        // Lazily fetch a mutable mesh reference to avoid triggering an update every frame
                        let lines_mesh = meshes.get_mut(&gizmos.lines_mesh_handle).unwrap();
                        // SAFETY: Will be only be fetched once for
                        let lines_mesh = unsafe { &mut *(lines_mesh as *mut _) };
                        mesh_helper::MeshEditXC::from(lines_mesh)
                    });

                    // TODO: Find a more efficient way of doing this, maybe a separated lines_mesh for only single frame lines
                    // ? NOTE: This algorithm will reduce the amount of memory that needs to br sended over to the GPU
                    // ? and also reduce memory fragmentation, although having to move quite a bit of data around
                    // Remove vertex attributes
                    edit.vertices.drain(vert.start..vert.end);
                    edit.colors.drain(vert.start..vert.end);

                    // Remove indexes
                    for i in index.end..edit.indices.len() {
                        debug_assert!(edit.indices[i] >= index.end as u32);
                        edit.indices[i - index.start] = edit.indices[i] - index.start as u32;
                    }
                    edit.indices
                        .resize_with(edit.indices.len() - index.start, || unreachable!());
                }
                GizmoVolatile::Shape(entity) => {
                    // Delete the gizmo
                    commands.despawn_recursive(entity);
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
            } => {
                // Spawn gizmo
                let entity = commands
                    .spawn((transform, GlobalTransform::default(), Children::default()))
                    .current_entity()
                    .unwrap();
                gizmo_instantiate(commands, gizmos, entity, &Gizmo { shape, color });
                // Keep track
                gizmos
                    .volatile
                    .push((duration, GizmoVolatile::Shape(entity)));
            }
            GizmoCommand::LineList {
                points,
                duration,
                color,
            } => {
                // Add new lines
                let edit = lines_mesh_edit.get_or_insert_with(|| {
                    // Lazily fetch a mutable mesh reference to avoid triggering an update every frame
                    let lines_mesh = meshes.get_mut(&gizmos.lines_mesh_handle).unwrap();
                    // SAFETY: Will be only be fetched once for
                    let lines_mesh = unsafe { &mut *(lines_mesh as *mut _) };
                    mesh_helper::MeshEditXC::from(lines_mesh)
                });

                let v = edit.vertices.len();

                // TODO: I really don't trust these iterators todo the right thing
                edit.vertices
                    .extend(points.iter().map(|v| <[f32; 3]>::from(*v)));

                edit.colors
                    .resize(edit.colors.len() + points.len(), <[f32; 4]>::from(color));

                let i = edit.indices.len();
                for j in 0..(points.len() - 1) {
                    edit.indices.push(j as u32);
                    edit.indices.push(j as u32 + 1);
                }

                // Keep track
                gizmos.volatile.push((
                    duration,
                    GizmoVolatile::Line(
                        Range {
                            start: v,
                            end: edit.vertices.len(),
                        },
                        Range {
                            start: i,
                            end: edit.indices.len(),
                        },
                    ),
                ));
            }
        }
    }
}

fn gizmo_instantiate(
    commands: &mut Commands,
    gizmos: &mut GizmosResources,
    parent: Entity,
    gizmo: &Gizmo,
) {
    match gizmo.shape.clone() {
        GizmoShape::Empty { radius } => {
            commands
                .spawn(GizmoMeshBundle::new(
                    Transform::from_scale(Vec3::splat(radius)),
                    gizmos.mesh_empty.clone(),
                    gizmo.color,
                ))
                .with(Parent(parent));
        }
        GizmoShape::Billboard { texture, size } => {
            commands
                .spawn(GizmoMeshBundle::new(
                    Transform::default(),
                    gizmos.mesh_billboard.clone(),
                    GizmoMaterial {
                        color: gizmo.color,
                        texture,
                        billboard: true,
                        billboard_size: size,
                        ..Default::default()
                    },
                ))
                .with(Parent(parent));
        }
        GizmoShape::Cube { size } => {
            commands
                .spawn(GizmoMeshBundle::new(
                    Transform::from_scale(size),
                    gizmos.mesh_cube.clone(),
                    gizmo.color,
                ))
                .with(Parent(parent));
        }
        GizmoShape::Sphere { radius } => {
            commands
                .spawn(GizmoMeshBundle::new(
                    Transform::from_scale(Vec3::splat(radius)),
                    gizmos.mesh_sphere.clone(),
                    gizmo.color,
                ))
                .with(Parent(parent));
        }
        GizmoShape::Hemisphere { radius } => {
            commands
                .spawn(GizmoMeshBundle::new(
                    Transform::from_scale(Vec3::splat(radius)),
                    gizmos.mesh_hemisphere.clone(),
                    gizmo.color,
                ))
                .with(Parent(parent));
        }
        GizmoShape::Cylinder { radius, height } => {
            commands
                .spawn(GizmoMeshBundle::new(
                    Transform::from_scale(Vec3::new(radius, height, radius)),
                    gizmos.mesh_cylinder.clone(),
                    gizmo.color,
                ))
                .with(Parent(parent));
        }
        GizmoShape::Capsule {
            radius,
            height,
            axis,
        } => {
            let mut top = Vec3::zero();
            let mut bottom = Vec3::zero();

            let offset = height * 0.5;
            let rotation = match axis {
                Axis::X => {
                    top[0] = -offset;
                    bottom[0] = offset;
                    Quat::from_rotation_z(std::f32::consts::PI * 0.5)
                }
                Axis::Y => {
                    top[1] = offset;
                    bottom[1] = -offset;
                    Quat::default()
                }
                Axis::Z => {
                    top[2] = offset;
                    bottom[2] = -offset;
                    Quat::from_rotation_x(std::f32::consts::PI * 0.5)
                }
            };

            commands
                .spawn(GizmoMeshBundle::new(
                    Transform {
                        translation: top,
                        rotation,
                        scale: Vec3::splat(radius),
                    },
                    gizmos.mesh_capsule_cap.clone(),
                    gizmo.color,
                ))
                .with(Parent(parent));
            commands
                .spawn(GizmoMeshBundle::new(
                    Transform {
                        translation: Vec3::zero(),
                        rotation,
                        scale: Vec3::new(radius, height, radius),
                    },
                    gizmos.mesh_cylinder.clone(),
                    gizmo.color,
                ))
                .with(Parent(parent));
            commands
                .spawn(GizmoMeshBundle::new(
                    Transform {
                        translation: bottom,
                        rotation,
                        scale: Vec3::splat(-radius),
                    },
                    gizmos.mesh_capsule_cap.clone(),
                    gizmo.color,
                ))
                .with(Parent(parent));
        }
    };
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.register_type::<GizmoMaterial>().add_system_to_stage(
            CoreStage::PostUpdate,
            shader::shader_defs_system::<GizmoMaterial>.system(),
        );

        app.insert_resource(GizmosCommandBuffer::default())
            .insert_resource(GizmosResources::default())
            .add_startup_system(gizmos_setup.system())
            .add_startup_system(rendering::gizmos_pipeline_setup.system())
            //.add_stage_after(stage::POST_UPDATE, "gizmos")
            .add_system_to_stage(CoreStage::PostUpdate, gizmos_update_system.system());
    }
}
