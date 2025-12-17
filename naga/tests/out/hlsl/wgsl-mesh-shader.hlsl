struct TaskPayload {
    float4 colorMask;
    bool visible;
    int _end_pad_0;
    int _end_pad_1;
    int _end_pad_2;
};

struct VertexOutput {
    float4 position : SV_Position;
    float4 color : LOC0;
};

struct PrimitiveOutput {
    uint3 indices_;
    bool cull : SV_CullPrimitive;
    float4 colorMask : LOC1 : primitive;
};

struct PrimitiveInput {
    float4 colorMask : LOC1 : primitive;
};

struct MeshOutput {
    VertexOutput vertices_[3];
    PrimitiveOutput primitives_[1];
    uint vertex_count;
    uint primitive_count;
};

static TaskPayload* taskPayload;
groupshared TaskPayload _taskPayload;
groupshared float workgroupData;
groupshared MeshOutput mesh_output;

struct MeshVertexOutput_ms_main {
    float4 color : LOC0;
    float4 position : SV_Position;
};

struct MeshPrimitiveOutput_ms_main {
    float4 colorMask : LOC1 : primitive;
    bool cull : SV_CullPrimitive;
};

struct FragmentInput_fs_main {
    float4 color_1 : LOC0;
    float4 colorMask_1 : LOC1 : primitive;
    float4 position_1 : SV_Position;
};

[numthreads(1, 1, 1)]
void ts_main(uint __local_invocation_index : SV_GroupIndex)
{
    taskPayload = &_taskPayload;
    if (all(__local_invocation_index == 0)) {
        workgroupData = (float)0;
    }
    GroupMemoryBarrierWithGroupSync();
    workgroupData = 1.0;
    (*taskPayload).colorMask = float4(1.0, 1.0, 0.0, 1.0);
    (*taskPayload).visible = true;
    uint3 gridSize = uint3(1u, 1u, 1u);
    DispatchMesh(gridSize.x, gridSize.x, gridSize.x, _taskPayload);
}

[numthreads(1, 1, 1)]
[outputtopology("triangle")]
void ms_main(uint __local_invocation_index : SV_GroupIndex, out indices uint3 triangleIndices[1], out vertices MeshVertexOutput_ms_main vertices_[3], out primitives MeshPrimitiveOutput_ms_main primitives_[1], in payload TaskPayload _taskPayload)
{
    taskPayload = &_taskPayload;
    if (all(__local_invocation_index == 0)) {
        workgroupData = (float)0;
        mesh_output = (MeshOutput)0;
    }
    GroupMemoryBarrierWithGroupSync();
    mesh_output.vertex_count = 3u;
    mesh_output.primitive_count = 1u;
    workgroupData = 2.0;
    mesh_output.vertices_[0].position = float4(0.0, 1.0, 0.0, 1.0);
    float4 _e23 = (*taskPayload).colorMask;
    mesh_output.vertices_[0].color = (float4(0.0, 1.0, 0.0, 1.0) * _e23);
    mesh_output.vertices_[1].position = float4(-1.0, -1.0, 0.0, 1.0);
    float4 _e45 = (*taskPayload).colorMask;
    mesh_output.vertices_[1].color = (float4(0.0, 0.0, 1.0, 1.0) * _e45);
    mesh_output.vertices_[2].position = float4(1.0, -1.0, 0.0, 1.0);
    float4 _e67 = (*taskPayload).colorMask;
    mesh_output.vertices_[2].color = (float4(1.0, 0.0, 0.0, 1.0) * _e67);
    mesh_output.primitives_[0].indices_ = uint3(0u, 1u, 2u);
    bool _e88 = (*taskPayload).visible;
    mesh_output.primitives_[0].cull = !(_e88);
    mesh_output.primitives_[0].colorMask = float4(1.0, 0.0, 1.0, 1.0);
    for (int vertIndex = __local_invocation_index; vertIndex < mesh_output.vertex_count; vertIndex += 1) {
        vertices_[vertIndex].color = mesh_output.vertices_[vertIndex].color;
        vertices_[vertIndex].position = mesh_output.vertices_[vertIndex].position;
    }
    for (int primIndex = __local_invocation_index; primIndex < mesh_output.primitive_count; primIndex += 1) {
        triangleIndices[primIndex] = mesh_output.primitives_[primIndex].indices_;
        primitives_[primIndex].colorMask = mesh_output.primitives_[primIndex].colorMask;
        primitives_[primIndex].cull = mesh_output.primitives_[primIndex].cull;
    }
    return;
}

float4 fs_main(FragmentInput_fs_main fragmentinput_fs_main) : SV_Target0
{
    VertexOutput vertex = { fragmentinput_fs_main.position_1, fragmentinput_fs_main.color_1 };
    PrimitiveInput primitive = { fragmentinput_fs_main.colorMask_1 };
    return (vertex.color * primitive.colorMask);
}
