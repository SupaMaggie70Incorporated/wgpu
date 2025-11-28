#version 450
#extension GL_EXT_mesh_shader : require
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(max_vertices = 3, max_primitives = 1, triangles) out;

struct TaskPayload {
  vec4 colorMask;
  bool visible;
};

struct VertexOutput {
  vec4 position;
  vec4 color;
};

struct PrimitiveOutput {
  uvec3 indices;
  bool cull;
  vec4 colorMask;
};

struct MeshOutput {
  VertexOutput vertices[3];
  PrimitiveOutput primitives[1];
  uint vertex_count;
  uint primitive_count;
};

out _40 { layout(location = 0) vec4 _m0; }
_43[3];

perprimitiveEXT out _47 { layout(location = 1) vec4 _m0; }
_50[1];

taskPayloadSharedEXT TaskPayload taskPayload;
shared MeshOutput mesh_output;

void main() {
  mesh_output.primitives[0u].cull = !taskPayload.visible;

  SetMeshOutputsEXT(3, 1);

  bool cull = mesh_output.primitives[gl_LocalInvocationIndex].cull;
  gl_MeshPrimitivesEXT[0].gl_CullPrimitiveEXT = cull;

  gl_MeshVerticesEXT[0].gl_Position = vec4(0.0, 1.0, 0.0, 1.0);
  gl_MeshVerticesEXT[1].gl_Position = vec4(-1.0, -1.0, 0.0, 1.0);
  gl_MeshVerticesEXT[2].gl_Position = vec4(1.0, -1.0, 0.0, 1.0);

  _43[0]._m0 = vec4(0.0, 1.0, 0.0, 1.0) * taskPayload.colorMask;
  _43[1]._m0 = vec4(0.0, 0.0, 1.0, 1.0) * taskPayload.colorMask;
  _43[2]._m0 = vec4(1.0, 0.0, 0.0, 1.0) * taskPayload.colorMask;

  gl_PrimitiveTriangleIndicesEXT[0] = uvec3(0u, 1u, 2u);
  _50[0]._m0 = vec4(1.0, 0.0, 1.0, 1.0);
  gl_MeshPrimitivesEXT[0].gl_CullPrimitiveEXT = false;
}
