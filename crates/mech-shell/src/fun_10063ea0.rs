// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040680(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ec30(param_1: *mut core::ffi::c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063ea0(param_1: *mut i32) {
    mech_trace::enter!("10063ea0", param_1);
    let mut param_1 = param_1;
    if !param_1.is_null() {
        // SAFETY: param_1 points at the caller's array of 56-byte records,
        // terminated by a record whose first word is -1; the callees are the
        // game's own.
        unsafe {
            let mut i_var2 = mech_trace::rd!("10063ea0", param_1, read_unaligned);
            while i_var2 != -1 {
                let pv_var3 = mech_trace::rd!("10063ea0", param_1.add(6), read_unaligned) as usize
                    as *mut core::ffi::c_void;
                if !pv_var3.is_null() {
                    mech_trace::cr!(
                        "10063ea0",
                        "10040680",
                        FUN_10040680(pv_var3 as usize as i32)
                    );
                    mech_trace::cr!("10063ea0", "1006ec30", FUN_1006ec30(pv_var3));
                    mech_trace::wr!("10063ea0", param_1.add(6), write_unaligned, 0);
                }
                param_1 = param_1.add(0xe);
                i_var2 = mech_trace::rd!("10063ea0", param_1, read_unaligned);
            }
        }
    }
}
