// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002ce4e(param_1: *mut i8) -> *mut u8;
    fn FUN_10081763(param_1: i32, param_2: i32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100816ea(param_1: *mut i8) -> u32 {
    mech_trace::enter!("100816ea", param_1);
    // SAFETY: the callees are the game's own; the returned pointer addresses
    // a table of seven pairs of i32 values, read where the original does.
    unsafe {
        let mut local_10 =
            mech_trace::cr!("100816ea", "1002ce4e", FUN_1002ce4e(param_1)).cast::<i32>();
        if local_10.is_null() {
            return mech_trace::rt!("100816ea", 0);
        }
        let mut local_c: i32 = 0;
        while local_c < 7 {
            let a = mech_trace::rd!("100816ea", local_10.byte_add(4), read_unaligned);
            let b = mech_trace::rd!("100816ea", local_10, read_unaligned);
            mech_trace::cr!("100816ea", "10081763", FUN_10081763(a, b));
            local_10 = local_10.wrapping_add(2);
            local_c = local_c.wrapping_add(1);
        }
        mech_trace::rt!("100816ea", 1)
    }
}
