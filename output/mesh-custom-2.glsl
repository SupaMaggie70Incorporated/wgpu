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

out NagaVertexOutput
{
    layout(location = 0) vec4 color;
} nagaVertices[3];

perprimitiveEXT out NagaPrimitiveOutput
{
    layout(location = 1) vec4 colorMask;
} nagaPrimitives[1];

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
    uint vert_count = min(mesh_output.vertex_count, 3u);
    uint prim_count = min(mesh_output.primitive_count, 1u);
    SetMeshOutputsEXT(vert_count, prim_count);
    uint i = gl_LocalInvocationIndex;
    for (; i < vert_count; i++)
    {
        gl_MeshVerticesEXT[i].gl_Position = mesh_output.vertices[i].position;
        nagaVertices[i].color = mesh_output.vertices[i].color;
    }
    i = gl_LocalInvocationIndex;
    for (; i < prim_count; i++)
    {
        gl_PrimitiveTriangleIndicesEXT[i] = mesh_output.primitives[i].indices;
        gl_MeshPrimitivesEXT[i].gl_CullPrimitiveEXT = mesh_output.primitives[i].cull;
        nagaPrimitives[i].colorMask = mesh_output.primitives[i].colorMask;
    }
}
    
