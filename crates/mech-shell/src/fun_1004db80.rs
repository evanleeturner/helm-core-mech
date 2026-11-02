// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009b3c4: i32;
    static mut DAT_10083508: u32;
    static mut DAT_1009bc30: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d0a0() -> u32;
    fn FUN_10016b60(param_1: u32);
    fn FUN_1000e970(param_1: *mut u8, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004db80() -> u32 {
    mech_trace::enter!("1004db80");
    // SAFETY: the globals and callees are the game's own; the state block
    // pointer in DAT_10083508 addresses memory the original reads at +0x23bb.
    unsafe {
        if mech_trace::gr!("1004db80", "1009b3c4", DAT_1009b3c4) != 0 {
            mech_trace::gw!("1004db80", "1009b3c4", DAT_1009b3c4, 0);
            let mut u_var1 = mech_trace::gr!("1004db80", "10083508", DAT_10083508);
            let i_field = mech_trace::rd!(
                "1004db80",
                (u_var1.wrapping_add(0x23bb) as *const i32),
                read_unaligned
            );
            if i_field == 1 {
                u_var1 = mech_trace::cr!("1004db80", "1004d0a0", FUN_1004d0a0());
            }
            return mech_trace::rt!("1004db80", u_var1 & 0xFFFF_FF00);
        }
        FUN_10016b60(0x1004ec50);
        let u_var2 = mech_trace::cr!(
            "1004db80",
            "1000e970",
            FUN_1000e970(&raw mut DAT_1009bc30, 0xc00a, 0xc00a)
        );
        mech_trace::gw!("1004db80", "1009b3c4", DAT_1009b3c4, 1);
        mech_trace::rt!("1004db80", (u_var2 & 0xFFFF_FF00) | 1)
    }
}
