#version 450

layout(location = 0) in vec4 v_Color;

#ifdef GIZMOMATERIAL_TEXTURE
layout(location = 1) in vec2 v_Uv;
#endif

// #ifndef GIZMOMATERIAL_UNLIT
// layout(location = 2) in vec3 v_Normal;
// #endif

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 3, binding = 0) uniform GizmoMaterial_color {
    vec4 Color;
};

#ifdef GIZMOMATERIAL_TEXTURE
layout(set = 3, binding = 1) uniform texture2D GizmoMaterial_texture;
layout(set = 3, binding = 2) uniform sampler GizmoMaterial_texture_sampler;
#endif

void main() {
    vec4 o = v_Color * Color;

#ifdef GIZMOMATERIAL_TEXTURE
    o *= texture(
        sampler2D(StandardMaterial_albedo_texture, StandardMaterial_albedo_texture_sampler),
        v_Uv);
#endif

#ifndef GIZMOMATERIAL_UNLIT
    o = vec4(0, 0, 0, 1);

    // TODO: Fake lighting with a unlit switch flag
#endif
    
    o_Target = o;
}
