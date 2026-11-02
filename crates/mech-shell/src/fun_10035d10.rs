// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: i32;
    static mut DAT_10092004: i32;
    static mut DAT_10091ffc: i32;
    static mut DAT_100a6798: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10030160(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035d10() {
    mech_trace::enter!("10035d10");
    // SAFETY: the callee and the globals are the game's own; DAT_100a6798
    // holds an address of a structure whose field at offset 4 is read.
    unsafe {
        let u_var1 = mech_trace::cr!(
            "10035d10",
            "10030160",
            FUN_10030160(
                mech_trace::gr!("10035d10", "10092000", DAT_10092000),
                mech_trace::gr!("10035d10", "10092004", DAT_10092004)
            )
        );
        if (u_var1 as u8) as i8 == 1 {
            mech_trace::gw!(
                "10035d10",
                "10091ffc",
                DAT_10091ffc,
                mech_trace::gr!("10035d10", "10091ffc", DAT_10091ffc).wrapping_add(1)
            );
            let limit = mech_trace::rd!(
                "10035d10",
                ((mech_trace::gr!("10035d10", "100a6798", DAT_100a6798) as u32).wrapping_add(4)
                    as *const i32),
                read_unaligned
            );
            if limit <= mech_trace::gr!("10035d10", "10091ffc", DAT_10091ffc) {
                mech_trace::gw!("10035d10", "10091ffc", DAT_10091ffc, 0);
            }
        }
    }
}
