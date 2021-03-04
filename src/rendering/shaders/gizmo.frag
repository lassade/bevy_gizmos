#version 450

layout(location = 0) in vec4 v_Color;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

layout(set = 3, binding = 0) uniform GizmoMaterial_color {
    vec4 Color;
};

void main() {
    #ifndef GIZMOMATERIAL_UNIT
    // TODO: Fake lighting with a unlit switch flag
    #endif
    
    o_Target = v_Color * Color;
}
