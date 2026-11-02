// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100596e0(param_1: *mut i32) -> i32 {
    let mut param_1 = param_1;
    // SAFETY: param_1 points at the caller's array of 0x38-byte records,
    // read exactly as the original does.
    unsafe {
        let mut i_var1: i32 = param_1.read_unaligned();
        let mut pi_var3: *mut i32 = param_1;
        loop {
            let pi_var2: *mut i32 = param_1;
            if i_var1 == -1 {
                return -1;
            }
            if pi_var3.add(0xb).read_unaligned() == 1 {
                break;
            }
            i_var1 = pi_var2.read_unaligned();
            param_1 = pi_var2.add(0xe);
            pi_var3 = pi_var2;
        }
        pi_var3.add(9).read_unaligned()
    }
}
