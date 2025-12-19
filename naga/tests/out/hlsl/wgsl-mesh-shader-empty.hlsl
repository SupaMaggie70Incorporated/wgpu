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

static TaskPayload taskPayloadStatic;
groupshared TaskPayload taskPayloadShared;
static bool taskPayloadIsShared;
#define taskPayload (taskPayloadIsShared ? taskPayloadShared : taskPayloadStatic)
groupshared MeshOutput mesh_output;

struct MeshVertexOutput_ms_main {
    float4 position : SV_Position;
};

struct MeshPrimitiveOutput_ms_main {
};

uint3 _ts_main(uint __local_invocation_index)
{
    return uint3(1u, 1u, 1u);
}
[numthreads(1, 1, 1)]
void ts_main(uint __local_invocation_index : SV_GroupIndex) {
    taskPayloadIsShared = true;
    if (all(__local_invocation_index == 0)) {
        taskPayloadShared = (TaskPayload)0;
    }
    GroupMemoryBarrierWithGroupSync();
    uint3 gridSize = _ts_main(__local_invocation_index);
    GroupMemoryBarrierWithGroupSync();
    DispatchMesh(gridSize.x, gridSize.y, gridSize.z, taskPayload);
}

void _ms_main(uint __local_invocation_index)
{
    return;
}
[numthreads(1, 1, 1)]
[outputtopology("triangle")]
void ms_main(uint __local_invocation_index : SV_GroupIndex, out indices uint3 triangleIndices[1], out vertices MeshVertexOutput_ms_main vertices_[3], out primitives MeshPrimitiveOutput_ms_main primitives_[1], in payload TaskPayload _taskPayload) {
    taskPayloadIsShared = false;
    taskPayloadStatic = _taskPayload;
    if (all(__local_invocation_index == 0)) {
        mesh_output = (MeshOutput)0;
    }
    GroupMemoryBarrierWithGroupSync();
    _ms_main(__local_invocation_index);
    GroupMemoryBarrierWithGroupSync();
    SetMeshOutputCounts(mesh_output.vertex_count, mesh_output.primitive_count);
    for (int vertIndex = __local_invocation_index; vertIndex < mesh_output.vertex_count; vertIndex += 1) {
        vertices_[vertIndex].position = mesh_output.vertices_[vertIndex].position;
    }
    for (int primIndex = __local_invocation_index; primIndex < mesh_output.primitive_count; primIndex += 1) {
        triangleIndices[primIndex] = mesh_output.primitives_[primIndex].indices_;
    }
}
