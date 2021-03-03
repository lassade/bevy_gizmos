#version 450

const int MAX_LIGHTS = 10;

struct Light {
    mat4 proj;
    vec4 pos;
    vec4 color;
};

layout(location = 0) in vec4 v_Color;

layout(location = 0) out vec4 o_Target;

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

// TODO: Remove this Lights struct will make the program segfault
// TODO: Use a couple of hard coded lights to draw the gizmos 
layout(set = 1, binding = 0) uniform Lights {
    vec3 AmbientColor;
    uvec4 NumLights;
    Light SceneLights[MAX_LIGHTS];
};

layout(set = 3, binding = 0) uniform GizmoMaterial_color {
    vec4 Color;
};

void main() {
    #if GIZMOMATERIAL_UNIT
    // TODO: Fake lighting with a unlit switch flag
    #endif
    
    o_Target = v_Color * Color;
}
