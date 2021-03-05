use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::PipelineDescriptor,
        render_graph::{base, RenderGraph, RenderResourcesNode},
        shader::{Shader, ShaderStage, ShaderStages},
    },
};

mod screen_info_node;

use crate::GizmoMaterial;

// NOTE: generated using python `import secrets; secrets.token_hex(8)`
pub const GIZMOS_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x936896ad9d35720c_u64);

pub(crate) fn gizmos_pipeline_setup(
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Pipeline setup

    let gizmo_pipeline = PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("shaders/gizmo.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("shaders/gizmo.frag"),
        ))),
    });

    // TODO: Support transparency

    // gizmo_pipeline.depth_stencil = None; // No depth

    pipelines.set_untracked(GIZMOS_PIPELINE_HANDLE, gizmo_pipeline);

    render_graph.add_system_node(
        "gizmo_material",
        RenderResourcesNode::<GizmoMaterial>::new(true),
    );
    render_graph
        .add_node_edge("gizmo_material", base::node::MAIN_PASS)
        .unwrap();

    render_graph.add_node(
        screen_info_node::SCREEN_INFO_NODE,
        screen_info_node::ScreenInfoNode::default(),
    );
    render_graph
        .add_node_edge(screen_info_node::SCREEN_INFO_NODE, base::node::MAIN_PASS)
        .unwrap();
}
