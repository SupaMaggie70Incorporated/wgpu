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

MESH TODO TaskPayload taskPayload;
groupshared float workgroupData;
groupshared MeshOutput mesh_output;

struct FragmentInput_fs_main {
    float4 color : LOC0;
    float4 colorMask : LOC1 : primitive;
    float4 position : SV_Position;
};

[numthreads(1, 1, 1)]
uint3 ts_main(uint3 __local_invocation_id : SV_GroupThreadID)
{
    if (all(__local_invocation_id == uint3(0u, 0u, 0u))) {
        workgroupData = (float)0;
    }
    GroupMemoryBarrierWithGroupSync();
    workgroupData = 1.0;
    taskPayload.colorMask = float4(1.0, 1.0, 0.0, 1.0);
    taskPayload.visible = true;
    return uint3(1u, 1u, 1u);
}

[numthreads(1, 1, 1)]
[outputtopology("triangle")]
void ms_main(uint3 __local_invocation_id : SV_GroupThreadID)
{
    if (all(__local_invocation_id == uint3(0u, 0u, 0u))) {
        workgroupData = (float)0;
        mesh_output = (MeshOutput)0;
    }
    GroupMemoryBarrierWithGroupSync();
    mesh_output.vertex_count = 3u;
    mesh_output.primitive_count = 1u;
    workgroupData = 2.0;
    mesh_output.vertices_[0].position = float4(0.0, 1.0, 0.0, 1.0);
    float4 _e23 = taskPayload.colorMask;
    mesh_output.vertices_[0].color = (float4(0.0, 1.0, 0.0, 1.0) * _e23);
    mesh_output.vertices_[1].position = float4(-1.0, -1.0, 0.0, 1.0);
    float4 _e45 = taskPayload.colorMask;
    mesh_output.vertices_[1].color = (float4(0.0, 0.0, 1.0, 1.0) * _e45);
    mesh_output.vertices_[2].position = float4(1.0, -1.0, 0.0, 1.0);
    float4 _e67 = taskPayload.colorMask;
    mesh_output.vertices_[2].color = (float4(1.0, 0.0, 0.0, 1.0) * _e67);
    mesh_output.primitives_[0].indices_ = uint3(0u, 1u, 2u);
    bool _e88 = taskPayload.visible;
    mesh_output.primitives_[0].cull = !(_e88);
    mesh_output.primitives_[0].colorMask = float4(1.0, 0.0, 1.0, 1.0);
    return;
}

float4 fs_main(FragmentInput_fs_main fragmentinput_fs_main) : SV_Target0
{
    VertexOutput vertex = { fragmentinput_fs_main.position, fragmentinput_fs_main.color };
    PrimitiveInput primitive = { fragmentinput_fs_main.colorMask };
    return (vertex.color * primitive.colorMask);
}
