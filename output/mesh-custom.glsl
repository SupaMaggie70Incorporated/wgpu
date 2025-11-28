#version 450
#extension GL_EXT_mesh_shader : require
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(max_vertices = 3, max_primitives = 1, triangles) out;

struct TaskPayload {
  vec4 colorMask;
  bool visible;
};

out _40 { layout(location = 0) vec4 _m0; }
_43[3];

perprimitiveEXT out _47 { layout(location = 1) vec4 _m0; }
_50[1];

taskPayloadSharedEXT TaskPayload taskPayload;

shared bool array[3];

void main() {
  array[0] = !taskPayload.visible;
  bool cull = array[gl_LocalInvocationIndex];
  gl_MeshPrimitivesEXT[0].gl_CullPrimitiveEXT = cull;

  SetMeshOutputsEXT(3, 1);

  gl_MeshVerticesEXT[0].gl_Position = vec4(0.0, 1.0, 0.0, 1.0);
  gl_MeshVerticesEXT[1].gl_Position = vec4(-1.0, -1.0, 0.0, 1.0);
  gl_MeshVerticesEXT[2].gl_Position = vec4(1.0, -1.0, 0.0, 1.0);

  _43[0]._m0 = vec4(0.0, 1.0, 0.0, 1.0) * taskPayload.colorMask;
  _43[1]._m0 = vec4(0.0, 0.0, 1.0, 1.0) * taskPayload.colorMask;
  _43[2]._m0 = vec4(1.0, 0.0, 0.0, 1.0) * taskPayload.colorMask;

  gl_PrimitiveTriangleIndicesEXT[0] = uvec3(0u, 1u, 2u);
  _50[0]._m0 = vec4(1.0, 0.0, 1.0, 1.0);
}
