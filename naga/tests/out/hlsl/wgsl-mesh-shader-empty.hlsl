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

MESH TODO TaskPayload taskPayload;
groupshared MeshOutput mesh_output;

[numthreads(1, 1, 1)]
uint3 ts_main()
{
    return uint3(1u, 1u, 1u);
}

[numthreads(1, 1, 1)]
[outputtopology("triangle")]
void ms_main(uint3 __local_invocation_id : SV_GroupThreadID)
{
    if (all(__local_invocation_id == uint3(0u, 0u, 0u))) {
        mesh_output = (MeshOutput)0;
    }
    GroupMemoryBarrierWithGroupSync();
    return;
}
