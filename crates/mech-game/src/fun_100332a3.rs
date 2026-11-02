// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38dc: u32;
    static mut DAT_100adae8: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100332a3() -> i32 {
    mech_trace::enter!("100332a3");
    let mut local_8: i32 = -1;
    // SAFETY: the globals and the callee are the game's own memory and code;
    // the string address is passed through as the original does.
    unsafe {
        if mech_trace::gr!("100332a3", "100b38dc", DAT_100b38dc) < 0xfe {
            local_8 = mech_trace::gr!("100332a3", "100b38dc", DAT_100b38dc) as i32;
            mech_trace::gw!(
                "100332a3",
                "100b38dc",
                DAT_100b38dc,
                mech_trace::gr!("100332a3", "100b38dc", DAT_100b38dc).wrapping_add(1)
            );
        } else {
            mech_trace::cr!(
                "100332a3",
                "100108a0",
                FUN_100108a0(0x2f, (&raw mut DAT_100adae8).cast::<i8>())
            );
        }
    }
    mech_trace::rt!("100332a3", local_8)
}
