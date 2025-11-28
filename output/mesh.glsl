#version 450
#extension GL_EXT_mesh_shader : require
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

out _40
{
    layout(location = 0) vec4 _m0;
} _43[3];

perprimitiveEXT out _47
{
    layout(location = 1) vec4 _m0;
} _50[1];

taskPayloadSharedEXT TaskPayload taskPayload;
shared float workgroupData;
shared MeshOutput mesh_output;

void main()
{
    if (all(equal(gl_LocalInvocationID, uvec3(0u))))
    {
        workgroupData = 0.0;
        mesh_output = MeshOutput(VertexOutput[](VertexOutput(vec4(0.0), vec4(0.0)), VertexOutput(vec4(0.0), vec4(0.0)), VertexOutput(vec4(0.0), vec4(0.0))), PrimitiveOutput[](PrimitiveOutput(uvec3(0u), false, vec4(0.0))), 0u, 0u);
    }
    barrier();
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
    uint _114 = min(mesh_output.vertex_count, 3u);
    uint _117 = min(mesh_output.primitive_count, 1u);
    SetMeshOutputsEXT(_114, _117);
    uint _30 = gl_LocalInvocationIndex;
    for (; _30 < _114; _30++)
    {
        gl_MeshVerticesEXT[_30].gl_Position = mesh_output.vertices[_30].position;
        gl_MeshVerticesEXT[_30].gl_Position.y = -mesh_output.vertices[_30].position.y;
        _43[_30]._m0 = mesh_output.vertices[_30].color;
    }
    _30 = gl_LocalInvocationIndex;
    for (; _30 < _117; _30++)
    {
        gl_PrimitiveTriangleIndicesEXT[0u] = mesh_output.primitives[0u].indices;
        gl_MeshPrimitivesEXT[0u].gl_CullPrimitiveEXT = mesh_output.primitives[0u].cull;
        _50[0u]._m0 = mesh_output.primitives[0u].colorMask;
    }
}

