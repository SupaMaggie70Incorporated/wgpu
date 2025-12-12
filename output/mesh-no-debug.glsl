#version 450
#extension GL_EXT_mesh_shader : require
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;
layout(max_vertices = 3, max_primitives = 1, triangles) out;

struct _6
{
    vec4 _m0;
    bool _m1;
};

struct _7
{
    vec4 _m0;
    vec4 _m1;
};

struct _10
{
    uvec3 _m0;
    bool _m1;
    vec4 _m2;
};

struct _15
{
    _7 _m0[3];
    _10 _m1[1];
    uint _m2;
    uint _m3;
};

out _39
{
    layout(location = 0) vec4 _m0;
} _42[3];

perprimitiveEXT out _46
{
    layout(location = 1) vec4 _m0;
} _49[1];

taskPayloadSharedEXT _6 _16;
shared float _18;
shared _15 _20;

void main()
{
    if (all(equal(gl_LocalInvocationID, uvec3(0u))))
    {
        _18 = 0.0;
        _20 = _15(_7[](_7(vec4(0.0), vec4(0.0)), _7(vec4(0.0), vec4(0.0)), _7(vec4(0.0), vec4(0.0))), _10[](_10(uvec3(0u), false, vec4(0.0))), 0u, 0u);
    }
    barrier();
    _20._m2 = 3u;
    _20._m3 = 1u;
    _18 = 2.0;
    _20._m0[0u]._m0 = vec4(0.0, 1.0, 0.0, 1.0);
    _20._m0[0u]._m1 = vec4(0.0, 1.0, 0.0, 1.0) * _16._m0;
    _20._m0[1u]._m0 = vec4(-1.0, -1.0, 0.0, 1.0);
    _20._m0[1u]._m1 = vec4(0.0, 0.0, 1.0, 1.0) * _16._m0;
    _20._m0[2u]._m0 = vec4(1.0, -1.0, 0.0, 1.0);
    _20._m0[2u]._m1 = vec4(1.0, 0.0, 0.0, 1.0) * _16._m0;
    _20._m1[0u]._m0 = uvec3(0u, 1u, 2u);
    _20._m1[0u]._m1 = !_16._m1;
    _20._m1[0u]._m2 = vec4(1.0, 0.0, 1.0, 1.0);
    barrier();
    uint _113 = min(_20._m2, 3u);
    uint _116 = min(_20._m3, 1u);
    SetMeshOutputsEXT(_113, _116);
    uint _29 = gl_LocalInvocationIndex;
    for (; _29 < _113; _29++)
    {
        gl_MeshVerticesEXT[_29].gl_Position = _20._m0[_29]._m0;
        _42[_29]._m0 = _20._m0[_29]._m1;
    }
    _29 = gl_LocalInvocationIndex;
    for (; _29 < _116; _29++)
    {
        gl_PrimitiveTriangleIndicesEXT[_29] = _20._m1[_29]._m0;
        gl_MeshPrimitivesEXT[_29].gl_CullPrimitiveEXT = _20._m1[_29]._m1;
        _49[_29]._m0 = _20._m1[_29]._m2;
    }
}

