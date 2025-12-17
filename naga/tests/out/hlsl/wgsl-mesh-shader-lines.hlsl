struct TaskPayload {
    uint dummy;
};

struct VertexOutput {
    float4 position : SV_Position;
};

struct PrimitiveOutput {
    uint2 indices_;
};

struct MeshOutput {
    VertexOutput vertices_[2];
    PrimitiveOutput primitives_[1];
    uint vertex_count;
    uint primitive_count;
};

static TaskPayload* taskPayload;
groupshared TaskPayload _taskPayload;
groupshared MeshOutput mesh_output;

[numthreads(1, 1, 1)]
void ts_main()
{
    taskPayload = &_taskPayload;
    return uint3(1u, 1u, 1u);
}

[numthreads(1, 1, 1)]
[outputtopology("line")]
void ms_main(uint3 __local_invocation_id : SV_GroupThreadID, in payload TaskPayload _taskPayload)
{
    taskPayload = &_taskPayload;
    if (all(__local_invocation_id == uint3(0u, 0u, 0u))) {
        mesh_output = (MeshOutput)0;
    }
    GroupMemoryBarrierWithGroupSync();
    return;
}
