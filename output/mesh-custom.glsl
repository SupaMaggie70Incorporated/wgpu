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

layout(location = 0) out _40
{
    vec4 _m0;
} _43[3];

layout(location = 1) perprimitiveEXT out _47
{
    vec4 _m0;
} _50[1];

taskPayloadSharedEXT TaskPayload taskPayload;
shared float workgroupData;
shared MeshOutput mesh_output;

void main()
{
  SetMeshOutputsEXT(3, 1);

const vec4[3] positions = {vec4(0., 1.0, 0., 1.0), vec4(-1.0, -1.0, 0., 1.0),
                           vec4(1.0, -1.0, 0., 1.0)};
const vec4[3] colors = {vec4(0., 1., 0., 1.), vec4(0., 0., 1., 1.),
                        vec4(1., 0., 0., 1.)};

  gl_MeshVerticesEXT[0].gl_Position = vec4(0., 1.0, 0., 1.0);
  gl_MeshVerticesEXT[1].gl_Position = vec4(-1.0, -1.0, 0., 1.0);
  gl_MeshVerticesEXT[2].gl_Position = vec4(1.0, -1.0, 0., 1.0);

  _43[0]._m0 = vec4(0., 1., 0., 1.) * taskPayload.colorMask;
  _43[1]._m0 = vec4(0., 0., 1., 1.) * taskPayload.colorMask;
  _43[2]._m0 = vec4(1., 0., 0., 1.) * taskPayload.colorMask;

  gl_PrimitiveTriangleIndicesEXT[gl_LocalInvocationIndex] = uvec3(0, 1, 2);
  _50[0]._m0 = vec4(1.0, 0.0, 1.0, 1.0);
  gl_MeshPrimitivesEXT[0].gl_CullPrimitiveEXT = !taskPayload.visible;
}

