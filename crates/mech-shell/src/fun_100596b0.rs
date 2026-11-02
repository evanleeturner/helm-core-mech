// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100596b0(param_1: *mut i32) -> i32 {
    mech_trace::enter!("100596b0", param_1);
    // SAFETY: param_1 points at the caller's array of 0x38-byte records;
    // reads happen exactly where the original reads them.
    unsafe {
        let mut param_1 = param_1;
        let mut i_var1 = mech_trace::rd!("100596b0", param_1, read_unaligned);
        let mut pi_var3 = param_1;
        loop {
            let pi_var2 = param_1;
            if i_var1 == -1 {
                return mech_trace::rt!("100596b0", -1);
            }
            if mech_trace::rd!("100596b0", pi_var3.add(0xb), read_unaligned) == 1 {
                break;
            }
            i_var1 = mech_trace::rd!("100596b0", pi_var2, read_unaligned);
            param_1 = pi_var2.add(0xe);
            pi_var3 = pi_var2;
        }
        mech_trace::rt!(
            "100596b0",
            mech_trace::rd!("100596b0", pi_var3.add(9), read_unaligned)
        )
    }
}
