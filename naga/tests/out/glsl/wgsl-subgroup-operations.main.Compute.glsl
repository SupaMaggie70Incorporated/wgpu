#version 430 core
#extension GL_ARB_compute_shader : require
#extension GL_KHR_shader_subgroup_basic : require
#extension GL_KHR_shader_subgroup_vote : require
#extension GL_KHR_shader_subgroup_arithmetic : require
#extension GL_KHR_shader_subgroup_ballot : require
#extension GL_KHR_shader_subgroup_shuffle : require
#extension GL_KHR_shader_subgroup_shuffle_relative : require
#extension GL_KHR_shader_subgroup_quad : require
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

struct Structure {
    uint num_subgroups;
    uint subgroup_size;
};
shared uint workgroup_var;


void main() {
    if (gl_LocalInvocationID == uvec3(0u)) {
        workgroup_var = 0u;
    }
    memoryBarrierShared();
    barrier();
    Structure sizes = Structure(gl_NumSubgroups, gl_SubgroupSize);
    uint subgroup_id = gl_SubgroupID;
    uint subgroup_invocation_id = gl_SubgroupInvocationID;
    uvec3 local_invocation_id = gl_LocalInvocationID;
    uint local_invocation_index = gl_LocalInvocationIndex;
    uvec4 _e9 = subgroupBallot(((subgroup_invocation_id & 1u) == 1u));
    uvec4 _e10 = subgroupBallot(true);
    bool _e13 = subgroupAll((subgroup_invocation_id != 0u));
    bool _e16 = subgroupAny((subgroup_invocation_id == 0u));
    uint _e18 = subgroupAdd(subgroup_invocation_id);
    workgroup_var = _e18;
    uint _e20 = workgroup_var;
    uint _e21 = subgroupMul(_e20);
    uint _e23 = subgroupMin(local_invocation_id.x);
    uint _e24 = subgroupMax(local_invocation_index);
    uint _e25 = subgroupAnd(subgroup_invocation_id);
    uint _e26 = subgroupOr(subgroup_invocation_id);
    uint _e27 = subgroupXor(subgroup_invocation_id);
    uint _e28 = subgroupExclusiveAdd(subgroup_invocation_id);
    uint _e29 = subgroupExclusiveMul(subgroup_invocation_id);
    uint _e30 = subgroupInclusiveAdd(subgroup_invocation_id);
    uint _e31 = subgroupInclusiveMul(subgroup_invocation_id);
    uint _e32 = subgroupBroadcastFirst(subgroup_invocation_id);
    uint _e34 = subgroupBroadcast(subgroup_invocation_id, 4u);
    uint _e39 = subgroupShuffle(subgroup_invocation_id, ((sizes.subgroup_size - 1u) - subgroup_invocation_id));
    uint _e41 = subgroupShuffleDown(subgroup_invocation_id, 1u);
    uint _e43 = subgroupShuffleUp(subgroup_invocation_id, 1u);
    uint _e47 = subgroupShuffleXor(subgroup_invocation_id, (sizes.subgroup_size - 1u));
    uint _e49 = subgroupQuadBroadcast(subgroup_invocation_id, 4u);
    uint _e50 = subgroupQuadSwapHorizontal(subgroup_invocation_id);
    uint _e51 = subgroupQuadSwapVertical(subgroup_invocation_id);
    uint _e52 = subgroupQuadSwapDiagonal(subgroup_invocation_id);
    return;
}

