#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec4 Vertex_Color;

layout(location = 0) out vec4 v_Color;

#if GIZMOMATERIAL_TEXTURED
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 1) out vec2 v_Uv;
#endif

layout(set = 0, binding = 0) uniform Camera {
    mat4 ViewProj;
};

// TODO: Sceen info

layout(set = 2, binding = 0) uniform Transform {
    mat4 Model;
};

void main() {
    #if GIZMOMATERIAL_BILLBOARD
    // TODO: Billboard using a billboard switch flag
    #endif

    v_Color = Vertex_Color;
    gl_Position = ViewProj * vec4((Model * vec4(Vertex_Position, 1.0)).xyz, 1.0);
}
