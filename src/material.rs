use bevy::{
    prelude::*,
    render::{renderer::RenderResources, shader::ShaderDefs},
};

#[derive(Debug, Clone, Reflect, RenderResources, ShaderDefs)]
#[reflect(Component)]
pub struct GizmoMaterial {
    pub color: Color,

    // ! FIXME: ShaderDefs, don't like defines that are on by default
    #[shader_def]
    #[render_resources(ignore)]
    pub unlit: bool,

    #[shader_def]
    #[reflect(ignore)]
    pub texture: Option<Handle<Texture>>,

    #[shader_def]
    #[render_resources(ignore)]
    pub billboard: bool,
    pub billboard_size: f32,
}

impl Default for GizmoMaterial {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            // lit: false,
            unlit: true,
            texture: None,
            billboard: false,
            billboard_size: 0.5,
        }
    }
}

impl From<Color> for GizmoMaterial {
    fn from(color: Color) -> Self {
        GizmoMaterial {
            color,
            ..Default::default()
        }
    }
}
