// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d810(param_1: *mut i32) -> i32 {
    // SAFETY: param_1 points at the caller's array of 0x38-byte records;
    // reads happen in the same order as the original.
    unsafe {
        let mut param_1 = param_1;
        let mut i_var1 = param_1.read_unaligned();
        let mut pi_var3 = param_1;
        loop {
            let pi_var2 = param_1;
            if i_var1 == -1 {
                return -1;
            }
            if pi_var3.wrapping_add(0xb).read_unaligned() == 1 {
                break;
            }
            i_var1 = pi_var2.read_unaligned();
            param_1 = pi_var2.wrapping_add(0xe);
            pi_var3 = pi_var2;
        }
        pi_var3.wrapping_add(9).read_unaligned()
    }
}
