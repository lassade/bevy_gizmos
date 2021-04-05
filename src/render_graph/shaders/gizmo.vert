#version 450

layout(location = 0) in vec3 Vertex_Position;
layout(location = 1) in vec4 Vertex_Color;

layout(location = 0) out vec4 v_Color;

#ifdef GIZMOMATERIAL_TEXTURE
layout(location = 2) in vec2 Vertex_Uv;
layout(location = 1) out vec2 v_Uv;
#endif

#ifdef GIZMOMATERIAL_LIT
layout(location = 3) in vec3 Vertex_Normal;
layout(location = 2) out vec3 v_Normal;
#endif

layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};

layout(set = 1, binding = 0) uniform Transform {
    mat4 Model;
};

layout(set = 2, binding = 0) uniform ScreenInfo {
    vec2 ScreenSize;
    vec2 ScreenAspectRatio;
};

#ifdef GIZMOMATERIAL_BILLBOARD
layout(set = 3, binding = 1) uniform GizmoMaterial_billboard_size {
    float BillboadSize;
};
#endif

void main() {
    v_Color = Vertex_Color;

#ifdef GIZMOMATERIAL_TEXTURE
    v_Uv = Vertex_Uv;
#endif


#ifdef GIZMOMATERIAL_BILLBOARD
    gl_Position = ViewProj * vec4(Model[3].xyz, 1.0);
    gl_Position /= gl_Position.w;
    gl_Position.xy += Vertex_Position.xy * vec2(ScreenAspectRatio.x, 1.0) * BillboadSize * 0.2;

#ifdef GIZMOMATERIAL_LIT
    // TODO: Transform normals
    v_Normal = Vertex_Normal;
#endif
#else
    gl_Position = ViewProj * vec4((Model * vec4(Vertex_Position, 1.0)).xyz, 1.0);

#ifdef GIZMOMATERIAL_LIT
    v_Normal = mat3(Model) * Vertex_Normal;
#endif
#endif
}
