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

groupshared TaskPayload taskPayload;
groupshared MeshOutput mesh_output;

struct MeshVertexOutput_ms_main {
    float4 position : SV_Position;
};

struct MeshPrimitiveOutput_ms_main {
};

[numthreads(1, 1, 1)]
void ts_main()
{
    uint3 gridSize = uint3(1u, 1u, 1u);
    DispatchMesh(gridSize.x, gridSize.x, gridSize.x, taskPayload);
}

[numthreads(1, 1, 1)]
[outputtopology("line")]
void ms_main(uint __local_invocation_index : SV_GroupIndex, out indices uint2 lineIndices[1], out vertices MeshVertexOutput_ms_main vertices_[2], out primitives MeshPrimitiveOutput_ms_main primitives_[1], in payload TaskPayload taskPayload)
{
    if (all(__local_invocation_index == 0)) {
        mesh_output = (MeshOutput)0;
    }
    GroupMemoryBarrierWithGroupSync();
    SetMeshOutputCounts(mesh_output.vertex_count, mesh_output.primitive_count);
    for (int vertIndex = __local_invocation_index; vertIndex < mesh_output.vertex_count; vertIndex += 1) {
        vertices_[vertIndex].position = mesh_output.vertices_[vertIndex].position;
    }
    for (int primIndex = __local_invocation_index; primIndex < mesh_output.primitive_count; primIndex += 1) {
        lineIndices[primIndex] = mesh_output.primitives_[primIndex].indices_;
    }
    return;
}
