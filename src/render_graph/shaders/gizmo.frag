#version 450

layout(location = 0) in vec4 v_Color;

#ifdef GIZMOMATERIAL_TEXTURE
layout(location = 1) in vec2 v_Uv;
#endif

#ifdef GIZMOMATERIAL_LIT
layout(location = 2) in vec3 v_Normal;
#endif

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 3, binding = 0) uniform GizmoMaterial_color {
    vec4 Color;
};

#ifdef GIZMOMATERIAL_TEXTURE
layout(set = 3, binding = 1) uniform texture2D GizmoMaterial_texture;
layout(set = 3, binding = 2) uniform sampler GizmoMaterial_texture_sampler;
#endif

float lambert(vec3 light_dir, vec3 normal) {
    // compute Lambertian diffuse term
    return max(0.0, dot(normal, light_dir));
}

void main() {
    vec4 o = v_Color * Color;

#ifdef GIZMOMATERIAL_TEXTURE
    o *= texture(
        sampler2D(GizmoMaterial_texture, GizmoMaterial_texture_sampler),
        v_Uv);
#endif

#ifdef GIZMOMATERIAL_LIT
    // // Camera View matrix ??
    // vec3 forward = normalize(ViewProj[2].xyz);
    // vec3 up = normalize(ViewProj[1].xyz);
    // vec3 right = normalize(ViewProj[0].xyz);

    // vec3 lighting = vec4(0.5, 0.5, 0.5); // Ambient light
    // lighting += lambert((-0.2 * forward + 0.4 * up + 0.4 * right), v_Normal) * vec3(0.8, 0.9, 1.0); // Rim light
    // lighting += lambert((0.2 * forward + 0.4 * up + 0.4 * right), v_Normal) * vec3(0.9, 0.9, 0.8); // Backlight
    // lighting /= 3.0;

    // o.xyz *= lighting;
#endif
    
    o_Target = o;
}
