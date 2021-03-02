#![allow(dead_code)]

use std::ops::Range;

use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::{Indices, VertexAttributeValues},
        pipeline::{PipelineDescriptor, PrimitiveTopology, RenderPipeline},
        render_graph::{base, RenderGraph, RenderResourcesNode},
        renderer::RenderResources,
        shader::{Shader, ShaderStage, ShaderStages},
    },
};
use smallvec::{smallvec, SmallVec};

mod gen;

#[derive(Debug, Copy, Clone)]
pub enum Axis {
    X,
    Y,
    Z,
}

// TODO: Implement reflect
#[derive(Debug, Copy, Clone)]
pub enum GizmoShape {
    /// Similar to blender empty axis
    Empty {
        radius: f32,
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

/// Use this bundle to spawn persistent gizmos
#[derive(Default, Bundle)]
pub struct GizmoBundle {
    pub gizmo: Gizmo,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub children: Children,
}

// NOTE: generated using python `import secrets; secrets.token_hex(8)`
const GIZMOS_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x936896ad9d35720c_u64);

#[derive(Default, Debug, Reflect, RenderResources)]
#[reflect(Component)]
pub struct GizmoMaterial {
    pub color: Color,
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
    pub parent: Parent,
    pub material: GizmoMaterial,
}

impl GizmoMeshBundle {
    fn new(parent: Entity, transform: Transform, mesh: Handle<Mesh>, color: Color) -> Self {
        Self {
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                GIZMOS_PIPELINE_HANDLE.typed(),
                //bevy::pbr::render_graph::FORWARD_PIPELINE_HANDLE.typed(),
            )]),
            mesh,
            visible: Default::default(),
            main_pass: Default::default(),
            draw: Default::default(),
            transform,
            global_transform: Default::default(),
            parent: Parent(parent),
            material: GizmoMaterial { color },
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

/// Immediate mode gizmos utility
pub struct GizmosCommandBuffer {
    enabled: bool,
    commands: crossbeam::queue::SegQueue<GizmoCommand>,
}

impl Default for GizmosCommandBuffer {
    fn default() -> Self {
        GizmosCommandBuffer {
            enabled: true,
            commands: Default::default(),
        }
    }
}

impl GizmosCommandBuffer {
    #[inline]
    pub fn push(&self, gizmo: GizmoCommand) {
        if self.enabled {
            self.commands.push(gizmo);
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
struct GizmosResources {
    mesh_empty: Handle<Mesh>,
    mesh_cube: Handle<Mesh>,
    mesh_sphere: Handle<Mesh>,
    mesh_hemisphere: Handle<Mesh>,
    mesh_cylinder: Handle<Mesh>,
    mesh_capsule_cap: Handle<Mesh>, // Similar to hemisphere but with less redundant lines

    // Gizmos command buffer
    volatile: Vec<(f32, Range<usize>, Option<Entity>)>,
    lines_entity: Option<Entity>,
    lines_mesh_handle: Handle<Mesh>,
}

fn gizmos_setup(
    mut gizmos: ResMut<GizmosResources>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    gizmos.mesh_empty = meshes.add(gen::wire_empty());
    gizmos.mesh_cube = meshes.add(gen::wire_cube());
    gizmos.mesh_sphere = meshes.add(gen::wire_sphere());
    gizmos.mesh_hemisphere = meshes.add(gen::wire_hemisphere());
    gizmos.mesh_cylinder = meshes.add(gen::wire_cylinder());
    gizmos.mesh_capsule_cap = meshes.add(gen::wire_capsule_cap());

    gizmos.lines_mesh_handle = {
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);
        mesh.set_attribute("Vertex_Color", Vec::<[f32; 4]>::with_capacity(32));
        mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, Vec::<[f32; 3]>::with_capacity(32));
        mesh.set_indices(Some(Indices::U16(Vec::with_capacity(32))));
        meshes.add(mesh)
    };

    //gizmos.lines_entity

    // Pipeline setup

    let gizmo_pipeline = PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("gizmo.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("gizmo.frag"),
        ))),
    });

    // gizmo_pipeline.depth_stencil = None; // No depth

    pipelines.set_untracked(GIZMOS_PIPELINE_HANDLE, gizmo_pipeline);

    render_graph.add_system_node(
        "gizmo_color",
        RenderResourcesNode::<GizmoMaterial>::new(true),
    );
    render_graph
        .add_node_edge("gizmo_color", base::node::MAIN_PASS)
        .unwrap();
}

fn gizmos_update_system(
    commands: &mut Commands,
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut gizmos: ResMut<GizmosResources>,
    mut gizmos_command_buffer: ResMut<GizmosCommandBuffer>,
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

    // let lines_mesh = meshes.get_mut(&gizmos.lines_mesh_handle).unwrap();

    // // TODO: Find a better good way getting all the stuff we need before hand (maybe using macros I duno)
    // if let (
    //     Some(VertexAttributeValues::Float3(verticies)),
    //     Some(VertexAttributeValues::Float4(colors)),
    //     Some(Indices::U32(indices)),
    // ) = (
    //     lines_mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR),
    //     lines_mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION),
    //     lines_mesh.indices_mut(),
    // ) {}

    // Clear previous gizmos
    for i in (0..gizmos.volatile.len()).rev() {
        let (time_left, range, entity) = &mut gizmos.volatile[i];

        if *time_left < 0.0 {
            if let Some(entity) = *entity {
                commands.despawn_recursive(entity);
            }

            // TODO: Clear line list mesh

            gizmos.volatile.remove(i);
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
                    .push((duration, Range { start: 0, end: 0 }, Some(entity)));
            }
            GizmoCommand::LineList {
                points,
                duration,
                color,
            } => {}
        }
    }
}

fn gizmo_instantiate(
    commands: &mut Commands,
    gizmos: &mut GizmosResources,
    parent: Entity,
    gizmo: &Gizmo,
) {
    match gizmo.shape {
        GizmoShape::Empty { radius } => {
            commands.spawn(GizmoMeshBundle::new(
                parent,
                Transform::from_scale(Vec3::splat(radius)),
                gizmos.mesh_empty.clone(),
                gizmo.color,
            ));
        }
        GizmoShape::Cube { size } => {
            commands.spawn(GizmoMeshBundle::new(
                parent,
                Transform::from_scale(size),
                gizmos.mesh_cube.clone(),
                gizmo.color,
            ));
        }
        GizmoShape::Sphere { radius } => {
            commands.spawn(GizmoMeshBundle::new(
                parent,
                Transform::from_scale(Vec3::splat(radius)),
                gizmos.mesh_sphere.clone(),
                gizmo.color,
            ));
        }
        GizmoShape::Hemisphere { radius } => {
            commands.spawn(GizmoMeshBundle::new(
                parent,
                Transform::from_scale(Vec3::splat(radius)),
                gizmos.mesh_hemisphere.clone(),
                gizmo.color,
            ));
        }
        GizmoShape::Cylinder { radius, height } => {
            commands.spawn(GizmoMeshBundle::new(
                parent,
                Transform::from_scale(Vec3::new(radius, height, radius)),
                gizmos.mesh_cylinder.clone(),
                gizmo.color,
            ));
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

            commands.spawn(GizmoMeshBundle::new(
                parent,
                Transform {
                    translation: top,
                    rotation,
                    scale: Vec3::splat(radius),
                },
                gizmos.mesh_capsule_cap.clone(),
                gizmo.color,
            ));
            commands.spawn(GizmoMeshBundle::new(
                parent,
                Transform {
                    translation: Vec3::zero(),
                    rotation,
                    scale: Vec3::new(radius, height, radius),
                },
                gizmos.mesh_cylinder.clone(),
                gizmo.color,
            ));
            commands.spawn(GizmoMeshBundle::new(
                parent,
                Transform {
                    translation: bottom,
                    rotation,
                    scale: Vec3::splat(-radius),
                },
                gizmos.mesh_capsule_cap.clone(),
                gizmo.color,
            ));
        }
    };
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GizmosCommandBuffer::default())
            .add_resource(GizmosResources::default())
            .add_startup_system(gizmos_setup.system())
            //.add_stage_after(stage::POST_UPDATE, "gizmos")
            .add_system_to_stage(stage::POST_UPDATE, gizmos_update_system.system());
    }
}
