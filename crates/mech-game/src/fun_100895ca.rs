// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008941c(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u8;
    fn FUN_10089060(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
    ) -> u8;
    fn FUN_10086973(param_1: *mut u32, param_2: *mut u32, param_3: u32) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100895ca(param_1: *mut i32, param_2: *mut i32, param_3: *mut u32) {
    mech_trace::enter!("100895ca", param_1, param_2, param_3);
    let mut local_40: i32;
    let mut local_3c: i32;
    let mut local_38: i32;
    let mut local_34: [i32; 12] = [0; 12];
    // SAFETY: the callees are the game's own; param_1 and param_2 point at the
    // caller's data, and local_34 holds the nine-int output plus the three
    // trailing ints copied out afterwards, as in the original stack layout.
    unsafe {
        mech_trace::cr!(
            "100895ca",
            "1008941c",
            FUN_1008941c(param_1, param_2, local_34.as_mut_ptr())
        );
        local_38 = mech_trace::rd!("100895ca", param_2.add(9), read_unaligned);
        local_3c = mech_trace::rd!("100895ca", param_2.add(10), read_unaligned);
        local_40 = mech_trace::rd!("100895ca", param_2.add(11), read_unaligned);
        mech_trace::cr!(
            "100895ca",
            "10089060",
            FUN_10089060(
                param_1,
                &raw mut local_38,
                &raw mut local_3c,
                &raw mut local_40,
            )
        );
        local_34[9] = local_38;
        local_34[10] = local_3c;
        local_34[11] = local_40;
        mech_trace::cr!(
            "100895ca",
            "10086973",
            FUN_10086973(param_3, local_34.as_mut_ptr().cast::<u32>(), 0x30)
        );
    }
}
