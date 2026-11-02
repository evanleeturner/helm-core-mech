// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160bd8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100496bb(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100482cb(param_1: i32) -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the global and the callee are the game's own; the returned
    // pointer, when non-null, points at a record whose third dword is written.
    unsafe {
        if DAT_10160bd8 == 0 {
            let pi_var1 = FUN_100496bb(param_1);
            if !pi_var1.is_null() {
                pi_var1.add(2).write_unaligned(1);
                local_8 = 1;
            }
        }
    }
    local_8
}
