#version 450
#extension GL_EXT_mesh_shader : require

struct VertexOutput
{
    vec4 position;
    vec4 color;
};

struct PrimitiveInput
{
    vec4 colorMask;
};

layout(location = 0) in vec4 color;
layout(location = 1) perprimitiveEXT in vec4 colorMask;
layout(location = 0) out vec4 _18;

void main()
{
    _18 = VertexOutput(gl_FragCoord, color).color * PrimitiveInput(colorMask).colorMask;
}

