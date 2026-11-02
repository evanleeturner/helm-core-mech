// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0678: u32;
    static mut DAT_101f52d4: u32;
    static mut DAT_101f52f4: u32;
    static mut DAT_101f5300: u8;
    static mut s_mw2_04d_gif_100b0694: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_10051cb0(param_1: *mut i8) -> u32;
    fn FUN_10051d99(param_1: i32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10051d19();
    fn FUN_10051e25();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10048051() {
    mech_trace::enter!("10048051");
    let mut local_14: [i8; 16] = [0; 16];
    // SAFETY: the globals, the format string and the callees are the game's
    // own; the locals outlive every call that receives their addresses.
    unsafe {
        let i_var1 = mech_trace::gr!("10048051", "100b0678", DAT_100b0678) as i32;
        let mut local_28: [u32; 5] = [0; 5];
        local_28[0] = (&raw mut DAT_101f5300) as u32;
        local_28[1] = 0;
        local_28[2] = 0;
        local_28[3] = mech_trace::gr!("10048051", "101f52f4", DAT_101f52f4);
        local_28[4] = mech_trace::gr!("10048051", "101f52d4", DAT_101f52d4);
        if mech_trace::gr!("10048051", "100b0678", DAT_100b0678) < 1000 {
            mech_trace::gw!(
                "10048051",
                "100b0678",
                DAT_100b0678,
                mech_trace::gr!("10048051", "100b0678", DAT_100b0678).wrapping_add(1)
            );
            mech_trace::cr!(
                "10048051",
                "sprintf",
                sprintf(
                    local_14.as_mut_ptr(),
                    &raw const s_mw2_04d_gif_100b0694,
                    i_var1,
                )
            );
            mech_trace::cr!("10048051", "10051cb0", FUN_10051cb0(local_14.as_mut_ptr()));
            FUN_10051d19();
            mech_trace::cr!(
                "10048051",
                "10051d99",
                FUN_10051d99(local_28.as_mut_ptr() as i32)
            );
            FUN_10051e25();
        }
    }
}
