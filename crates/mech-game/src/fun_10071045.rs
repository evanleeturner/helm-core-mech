// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn _fseek(file: *mut u8, offset: i32, origin: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10071045(param_1: *mut u32, param_2: i32) -> u32 {
    mech_trace::enter!("10071045", param_1, param_2);
    if param_2 == -1 {
        return mech_trace::rt!("10071045", 0xffffffff);
    }
    // SAFETY: param_1 points at the caller's table; the file handle and the
    // entries are read where the original reads them, and _fseek is the game's own.
    unsafe {
        let idx = param_2.wrapping_mul(3) as isize;
        let file = mech_trace::rd!("10071045", param_1, read_unaligned) as *mut u8;
        let offset = mech_trace::rd!("10071045", param_1.offset(idx + 2), read_unaligned) as i32;
        let r = mech_trace::cr!("10071045", "_fseek", _fseek(file, offset, 0));
        mech_trace::rt!(
            "10071045",
            if r == 0 {
                mech_trace::rd!("10071045", param_1.offset(idx + 3), read_unaligned)
            } else {
                0xffffffff
            }
        )
    }
}
