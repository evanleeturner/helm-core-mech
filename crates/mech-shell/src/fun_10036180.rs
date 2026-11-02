// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093900: u8;
    static mut DAT_10093a18: u8;
    static mut DAT_10093b2c: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036180(param_1: i32) -> *mut u32 {
    mech_trace::enter!("10036180", param_1);
    if param_1 < 0 {
        // SAFETY: the global is the game's own pointer variable, read by value.
        return mech_trace::rt!("10036180", unsafe {
            mech_trace::gr!("10036180", "10093b2c", DAT_10093b2c)
        } as *mut u32);
    }
    // SAFETY: only the addresses of the game's globals are taken, no reads.
    unsafe {
        let mut p_var1 = &raw mut DAT_10093a18 as *mut u32;
        if param_1 == 0 {
            p_var1 = &raw mut DAT_10093900 as *mut u32;
        }
        mech_trace::rt!("10036180", p_var1)
    }
}
