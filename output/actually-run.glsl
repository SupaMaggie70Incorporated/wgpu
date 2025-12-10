#version 450
#extension GL_EXT_mesh_shader : require
#extension GL_EXT_null_initializer : require
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(max_vertices = 3, max_primitives = 1, triangles) out;

struct TaskPayload
{
    vec4 colorMask;
    bool visible;
};

struct VertexOutput
{
    vec4 position;
    vec4 color;
};

struct PrimitiveOutput
{
    uvec3 indices;
    bool cull;
    vec4 colorMask;
};

struct MeshOutput
{
    VertexOutput vertices[3];
    PrimitiveOutput primitives[1];
    uint vertex_count;
    uint primitive_count;
};

out _42
{
    layout(location = 0) vec4 _m0;
} _45[3];

perprimitiveEXT out _49
{
    layout(location = 1) vec4 _m0;
} _52[1];

taskPayloadSharedEXT TaskPayload taskPayload;
shared float workgroupData = { };
shared MeshOutput mesh_output = { };

void main()
{
    mesh_output.vertex_count = 3u;
    mesh_output.primitive_count = 1u;
    workgroupData = 2.0;
    mesh_output.vertices[0u].position = vec4(0.0, 1.0, 0.0, 1.0);
    mesh_output.vertices[0u].color = vec4(0.0, 1.0, 0.0, 1.0) * taskPayload.colorMask;
    mesh_output.vertices[1u].position = vec4(-1.0, -1.0, 0.0, 1.0);
    mesh_output.vertices[1u].color = vec4(0.0, 0.0, 1.0, 1.0) * taskPayload.colorMask;
    mesh_output.vertices[2u].position = vec4(1.0, -1.0, 0.0, 1.0);
    mesh_output.vertices[2u].color = vec4(1.0, 0.0, 0.0, 1.0) * taskPayload.colorMask;
    mesh_output.primitives[0u].indices = uvec3(0u, 1u, 2u);
    mesh_output.primitives[0u].cull = !taskPayload.visible;
    mesh_output.primitives[0u].colorMask = vec4(1.0, 0.0, 1.0, 1.0);
    barrier();
    uint _105 = min(mesh_output.vertex_count, 3u);
    uint _108 = min(mesh_output.primitive_count, 1u);
    SetMeshOutputsEXT(_105, _108);
    uint _32 = gl_LocalInvocationIndex;
    for (; _32 < _105; _32++)
    {
        gl_MeshVerticesEXT[_32].gl_Position = mesh_output.vertices[_32].position;
        _45[_32]._m0 = mesh_output.vertices[_32].color;
    }
    _32 = gl_LocalInvocationIndex;
    for (; _32 < _108; _32++)
    {
        gl_PrimitiveTriangleIndicesEXT[_32] = mesh_output.primitives[_32].indices;
        gl_MeshPrimitivesEXT[_32].gl_CullPrimitiveEXT = mesh_output.primitives[_32].cull;
        _52[_32]._m0 = mesh_output.primitives[_32].colorMask;
    }
}

