struct Structure {
    @builtin(num_subgroups) num_subgroups: u32,
    @builtin(subgroup_size) subgroup_size: u32,
};

var<workgroup> workgroup_var: u32;

@compute @workgroup_size(1)
fn main(
    sizes: Structure,
    @builtin(subgroup_id) subgroup_id: u32,
    @builtin(subgroup_invocation_id) subgroup_invocation_id: u32,
    @builtin(local_invocation_id) local_invocation_id: vec3<u32>,
    @builtin(local_invocation_index) local_invocation_index: u32,
) {
    _ = subgroupBallot((subgroup_invocation_id & 1u) == 1u);
    _ = subgroupBallot();

    _ = subgroupAll(subgroup_invocation_id != 0u);
    _ = subgroupAny(subgroup_invocation_id == 0u);
    workgroup_var = subgroupAdd(subgroup_invocation_id);
    _ = subgroupMul(workgroup_var);
    _ = subgroupMin(local_invocation_id.x);
    _ = subgroupMax(local_invocation_index);
    _ = subgroupAnd(subgroup_invocation_id);
    _ = subgroupOr(subgroup_invocation_id);
    _ = subgroupXor(subgroup_invocation_id);
    _ = subgroupExclusiveAdd(subgroup_invocation_id);
    _ = subgroupExclusiveMul(subgroup_invocation_id);
    _ = subgroupInclusiveAdd(subgroup_invocation_id);
    _ = subgroupInclusiveMul(subgroup_invocation_id);

    _ = subgroupBroadcastFirst(subgroup_invocation_id);
    _ = subgroupBroadcast(subgroup_invocation_id, 4u);
    _ = subgroupShuffle(subgroup_invocation_id, sizes.subgroup_size - 1u - subgroup_invocation_id);
    _ = subgroupShuffleDown(subgroup_invocation_id, 1u);
    _ = subgroupShuffleUp(subgroup_invocation_id, 1u);
    _ = subgroupShuffleXor(subgroup_invocation_id, sizes.subgroup_size - 1u);

    _ = quadBroadcast(subgroup_invocation_id, 4u);
    _ = quadSwapX(subgroup_invocation_id);
    _ = quadSwapY(subgroup_invocation_id);
    _ = quadSwapDiagonal(subgroup_invocation_id);
}
