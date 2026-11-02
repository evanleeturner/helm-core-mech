// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf34: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_1007fcc3(param_1: *mut i32, param_2: *mut i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007fc1d(param_1: *mut i32, param_2: *mut i32) {
    mech_trace::enter!("1007fc1d", param_1, param_2);
    // SAFETY: the globals and callees are the game's own; param_1 and param_2
    // point at the caller's int arrays, read at the original's offsets.
    unsafe {
        if mech_trace::dr!(
            "1007fc1d",
            ((mech_trace::gr!("1007fc1d", "100adf34", DAT_100adf34) as usize).wrapping_add(3)
                as *const i8)
        ) != 0
        {
            let a = mech_trace::rd!("1007fc1d", param_1.add(0x10d), read_unaligned).wrapping_sub(
                mech_trace::rd!("1007fc1d", param_2.add(0x10d), read_unaligned),
            );
            let b = mech_trace::rd!("1007fc1d", param_1.add(0x10e), read_unaligned).wrapping_sub(
                mech_trace::rd!("1007fc1d", param_2.add(0x10e), read_unaligned),
            );
            let c = mech_trace::rd!("1007fc1d", param_1.add(0x10f), read_unaligned).wrapping_sub(
                mech_trace::rd!("1007fc1d", param_2.add(0x10f), read_unaligned),
            );
            let i_var1 = mech_trace::cr!("1007fc1d", "1000fc30", FUN_1000fc30(a, b, c));
            if 200000 < i_var1 {
                let r = mech_trace::cr!(
                    "1007fc1d",
                    "10002570",
                    FUN_10002570(i_var1.wrapping_sub(200000) as u32, 1300000)
                ) as i32;
                FUN_1007fcc3(param_1, param_2, r.wrapping_mul(3));
            }
        }
    }
}
