struct Structure {
    uint num_subgroups;
    uint subgroup_size;
};

groupshared uint workgroup_var;

struct ComputeInput_main {
    uint3 local_invocation_id_1 : SV_GroupThreadID;
    uint local_invocation_index_1 : SV_GroupIndex;
};

[numthreads(1, 1, 1)]
void main(ComputeInput_main computeinput_main, uint3 local_invocation_id_2 : SV_GroupThreadID)
{
    if (all(local_invocation_id_2 == uint3(0u, 0u, 0u))) {
        workgroup_var = (uint)0;
    }
    GroupMemoryBarrierWithGroupSync();
    Structure sizes = { (1u + WaveGetLaneCount() - 1u) / WaveGetLaneCount(), WaveGetLaneCount() };
    uint subgroup_id = computeinput_main.local_invocation_index_1 / WaveGetLaneCount();
    uint subgroup_invocation_id = WaveGetLaneIndex();
    uint3 local_invocation_id = computeinput_main.local_invocation_id_1;
    uint local_invocation_index = computeinput_main.local_invocation_index_1;
    const uint4 _e9 = WaveActiveBallot(((subgroup_invocation_id & 1u) == 1u));
    const uint4 _e10 = WaveActiveBallot(true);
    const bool _e13 = WaveActiveAllTrue((subgroup_invocation_id != 0u));
    const bool _e16 = WaveActiveAnyTrue((subgroup_invocation_id == 0u));
    const uint _e18 = WaveActiveSum(subgroup_invocation_id);
    workgroup_var = _e18;
    uint _e20 = workgroup_var;
    const uint _e21 = WaveActiveProduct(_e20);
    const uint _e23 = WaveActiveMin(local_invocation_id.x);
    const uint _e24 = WaveActiveMax(local_invocation_index);
    const uint _e25 = WaveActiveBitAnd(subgroup_invocation_id);
    const uint _e26 = WaveActiveBitOr(subgroup_invocation_id);
    const uint _e27 = WaveActiveBitXor(subgroup_invocation_id);
    const uint _e28 = WavePrefixSum(subgroup_invocation_id);
    const uint _e29 = WavePrefixProduct(subgroup_invocation_id);
    const uint _e30 = subgroup_invocation_id + WavePrefixSum(subgroup_invocation_id);
    const uint _e31 = subgroup_invocation_id * WavePrefixProduct(subgroup_invocation_id);
    const uint _e32 = WaveReadLaneFirst(subgroup_invocation_id);
    const uint _e34 = WaveReadLaneAt(subgroup_invocation_id, 4u);
    const uint _e39 = WaveReadLaneAt(subgroup_invocation_id, ((sizes.subgroup_size - 1u) - subgroup_invocation_id));
    const uint _e41 = WaveReadLaneAt(subgroup_invocation_id, WaveGetLaneIndex() + 1u);
    const uint _e43 = WaveReadLaneAt(subgroup_invocation_id, WaveGetLaneIndex() - 1u);
    const uint _e47 = WaveReadLaneAt(subgroup_invocation_id, WaveGetLaneIndex() ^ (sizes.subgroup_size - 1u));
    const uint _e49 = QuadReadLaneAt(subgroup_invocation_id, 4u);
    const uint _e50 = QuadReadAcrossX(subgroup_invocation_id);
    const uint _e51 = QuadReadAcrossY(subgroup_invocation_id);
    const uint _e52 = QuadReadAcrossDiagonal(subgroup_invocation_id);
    return;
}
