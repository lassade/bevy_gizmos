use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::{
        pipeline::*,
        render_graph::{base, RenderGraph, RenderResourcesNode},
        shader::{Shader, ShaderStage, ShaderStages},
        texture::TextureFormat,
    },
};

mod screen_info_node;

use crate::GizmoMaterial;

// TODO: Overlay pipeline

// NOTE: generated using python `import secrets; secrets.token_hex(8)`
pub const GIZMOS_PIPELINE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(PipelineDescriptor::TYPE_UUID, 0x936896ad9d35720c_u64);

pub(crate) fn gizmos_pipeline_setup(
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Pipeline setup

    let gizmo_pipeline = PipelineDescriptor {
        name: None,
        primitive: PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: FrontFace::Ccw,
            cull_mode: CullMode::Back,
            polygon_mode: PolygonMode::Fill,
        },
        layout: None,
        depth_stencil: Some(DepthStencilState {
            format: TextureFormat::Depth32Float,
            depth_write_enabled: false,
            depth_compare: CompareFunction::Less,
            stencil: StencilState {
                front: StencilFaceState::IGNORE,
                back: StencilFaceState::IGNORE,
                read_mask: 0,
                write_mask: 0,
            },
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0.0,
                clamp: 0.0,
            },
            clamp_depth: false,
        }),
        color_target_states: vec![ColorTargetState {
            format: TextureFormat::default(),
            color_blend: BlendState {
                src_factor: BlendFactor::SrcAlpha,
                dst_factor: BlendFactor::OneMinusSrcAlpha,
                operation: BlendOperation::Add,
            },
            alpha_blend: BlendState {
                src_factor: BlendFactor::One,
                dst_factor: BlendFactor::One,
                operation: BlendOperation::Add,
            },
            write_mask: ColorWrite::ALL,
        }],
        multisample: MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        shader_stages: ShaderStages {
            vertex: shaders.add(Shader::from_glsl(
                ShaderStage::Vertex,
                include_str!("shaders/gizmo.vert"),
            )),
            fragment: Some(shaders.add(Shader::from_glsl(
                ShaderStage::Fragment,
                include_str!("shaders/gizmo.frag"),
            ))),
        },
    };

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

    render_graph.add_system_node(
        screen_info_node::SCREEN_INFO_NODE,
        screen_info_node::ScreenInfoNode::default(),
    );
    render_graph
        .add_node_edge(screen_info_node::SCREEN_INFO_NODE, base::node::MAIN_PASS)
        .unwrap();
}
