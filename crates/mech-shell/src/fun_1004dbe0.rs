// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: i32;
    static mut DAT_1009b3c4: i32;
    static mut s_mdeath_1009bc38: [i8; 0];
    static mut s_mdeath_1009bc40: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10016b60(param_1: i32);
    fn FUN_1000e970(param_1: *mut u8, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004dbe0() -> u32 {
    mech_trace::enter!("1004dbe0");
    // SAFETY: the globals and callees are the game's own; DAT_10083508 is a
    // base pointer whose +0x23bb field is read unaligned as the original does.
    unsafe {
        let i_var2 = mech_trace::gr!("1004dbe0", "1009b3c4", DAT_1009b3c4);
        let i_var1 = mech_trace::rd!(
            "1004dbe0",
            ((mech_trace::gr!("1004dbe0", "10083508", DAT_10083508) as usize).wrapping_add(0x23bb)
                as *const i32),
            read_unaligned
        );
        if i_var1 == 0 {
            if mech_trace::gr!("1004dbe0", "1009b3c4", DAT_1009b3c4) == 0 {
                FUN_10016b60(0x1004ec50);
                let u_var3 = mech_trace::cr!(
                    "1004dbe0",
                    "1000e970",
                    FUN_1000e970((&raw mut s_mdeath_1009bc40).cast::<u8>(), 0xc00a, 0xc00a)
                );
                mech_trace::gw!("1004dbe0", "1009b3c4", DAT_1009b3c4, 1);
                return mech_trace::rt!("1004dbe0", u_var3 & 0xffffff00);
            }
            mech_trace::gw!("1004dbe0", "1009b3c4", DAT_1009b3c4, 0);
            return mech_trace::rt!("1004dbe0", ((i_var2 as u32) & 0xffffff00) | 1);
        }
        if i_var1 != 1 {
            return mech_trace::rt!("1004dbe0", ((i_var1 as u32) & 0xffffff00) | 1);
        }
        FUN_10016b60(0x1004ec50);
        let u_var3 = mech_trace::cr!(
            "1004dbe0",
            "1000e970",
            FUN_1000e970((&raw mut s_mdeath_1009bc38).cast::<u8>(), 0xc02b, 0xc02b)
        );
        mech_trace::rt!("1004dbe0", u_var3 & 0xffffff00)
    }
}
