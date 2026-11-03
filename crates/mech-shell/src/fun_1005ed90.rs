// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1006ec30(param_1: *mut u8);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_1005ef70(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_1005ed90(param_1: *mut i32) {
    mech_trace::enter!("1005ed90", param_1);
    // SAFETY: param_1 points at the caller's object, whose first field is a
    // pointer to a structure with a count at offset 4; the callees are the
    // game's own and are called in the original order.
    unsafe {
        let mut i_var1 = mech_trace::rd!(
            "1005ed90",
            ((mech_trace::rd!("1005ed90", param_1, read_unaligned) as usize + 4) as *const i32),
            read_unaligned
        );
        while 0 < i_var1 {
            let pv_var2 = mech_trace::cr!(
                "1005ed90",
                "10018060",
                FUN_10018060(mech_trace::rd!("1005ed90", param_1, read_unaligned), 0)
            );
            if pv_var2 != 0 {
                FUN_1005ef70(pv_var2 as i32);
                FUN_1006ec30(pv_var2 as *mut u8);
            }
            i_var1 = mech_trace::rd!(
                "1005ed90",
                ((mech_trace::rd!("1005ed90", param_1, read_unaligned) as usize + 4) as *const i32),
                read_unaligned
            );
        }
    }
}
