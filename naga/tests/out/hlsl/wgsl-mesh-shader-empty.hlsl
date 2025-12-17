struct TaskPayload {
    uint dummy;
};

struct VertexOutput {
    float4 position : SV_Position;
};

struct PrimitiveOutput {
    uint3 indices_;
};

struct MeshOutput {
    VertexOutput vertices_[3];
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
    uint3 gridSize = uint3(1u, 1u, 1u);
    DispatchMesh(gridSize.x, gridSize.x, gridSize.x, _taskPayload);
}

[numthreads(1, 1, 1)]
[outputtopology("triangle")]
void ms_main(uint3 __local_invocation_id : SV_GroupThreadID, in payload TaskPayload _taskPayload)
{
    taskPayload = &_taskPayload;
    if (all(__local_invocation_id == uint3(0u, 0u, 0u))) {
        mesh_output = (MeshOutput)0;
    }
    GroupMemoryBarrierWithGroupSync();
}
