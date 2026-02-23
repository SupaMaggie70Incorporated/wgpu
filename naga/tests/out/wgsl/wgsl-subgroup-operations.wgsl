struct Structure {
    @builtin(num_subgroups) num_subgroups: u32,
    @builtin(subgroup_size) subgroup_size: u32,
}

var<workgroup> workgroup_var: u32;

@compute @workgroup_size(1, 1, 1) 
fn main(sizes: Structure, @builtin(subgroup_id) subgroup_id: u32, @builtin(subgroup_invocation_id) subgroup_invocation_id: u32, @builtin(local_invocation_id) local_invocation_id: vec3<u32>, @builtin(local_invocation_index) local_invocation_index: u32) {
    let _e9 = subgroupBallot(((subgroup_invocation_id & 1u) == 1u));
    let _e10 = subgroupBallot();
    let _e13 = subgroupAll((subgroup_invocation_id != 0u));
    let _e16 = subgroupAny((subgroup_invocation_id == 0u));
    let _e18 = subgroupAdd(subgroup_invocation_id);
    workgroup_var = _e18;
    let _e20 = workgroup_var;
    let _e21 = subgroupMul(_e20);
    let _e23 = subgroupMin(local_invocation_id.x);
    let _e24 = subgroupMax(local_invocation_index);
    let _e25 = subgroupAnd(subgroup_invocation_id);
    let _e26 = subgroupOr(subgroup_invocation_id);
    let _e27 = subgroupXor(subgroup_invocation_id);
    let _e28 = subgroupExclusiveAdd(subgroup_invocation_id);
    let _e29 = subgroupExclusiveMul(subgroup_invocation_id);
    let _e30 = subgroupInclusiveAdd(subgroup_invocation_id);
    let _e31 = subgroupInclusiveMul(subgroup_invocation_id);
    let _e32 = subgroupBroadcastFirst(subgroup_invocation_id);
    let _e34 = subgroupBroadcast(subgroup_invocation_id, 4u);
    let _e39 = subgroupShuffle(subgroup_invocation_id, ((sizes.subgroup_size - 1u) - subgroup_invocation_id));
    let _e41 = subgroupShuffleDown(subgroup_invocation_id, 1u);
    let _e43 = subgroupShuffleUp(subgroup_invocation_id, 1u);
    let _e47 = subgroupShuffleXor(subgroup_invocation_id, (sizes.subgroup_size - 1u));
    let _e49 = quadBroadcast(subgroup_invocation_id, 4u);
    let _e50 = quadSwapX(subgroup_invocation_id);
    let _e51 = quadSwapY(subgroup_invocation_id);
    let _e52 = quadSwapDiagonal(subgroup_invocation_id);
    return;
}
