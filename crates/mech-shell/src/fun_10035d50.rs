// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10091ffc: i32;
    static mut DAT_10092000: *mut u32;
    static mut DAT_10092004: *mut u32;
    static mut DAT_100a6798: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10030160(param_1: *mut u32, param_2: *mut u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035d50() {
    mech_trace::enter!("10035d50");
    // SAFETY: the globals and the callee are the game's own memory; the
    // pointer at DAT_100a6798 + 4 is read where the original reads it.
    unsafe {
        let u_var1 = mech_trace::cr!(
            "10035d50",
            "10030160",
            FUN_10030160(
                mech_trace::gr!("10035d50", "10092000", DAT_10092000),
                mech_trace::gr!("10035d50", "10092004", DAT_10092004)
            )
        );
        if (u_var1 as u8) == 1 {
            mech_trace::gw!(
                "10035d50",
                "10091ffc",
                DAT_10091ffc,
                mech_trace::gr!("10035d50", "10091ffc", DAT_10091ffc).wrapping_sub(1)
            );
            if mech_trace::gr!("10035d50", "10091ffc", DAT_10091ffc) < 0 {
                let v = mech_trace::rd!(
                    "10035d50",
                    ((mech_trace::gr!("10035d50", "100a6798", DAT_100a6798).wrapping_add(4))
                        as *const i32),
                    read_unaligned
                );
                mech_trace::gw!("10035d50", "10091ffc", DAT_10091ffc, v.wrapping_sub(1));
            }
        }
    }
}
