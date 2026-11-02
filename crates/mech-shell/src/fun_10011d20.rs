// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008eac4: *mut u8;
    static mut DAT_1008eac8: *mut u8;
    static mut DAT_1008eacc: *mut u8;
    static mut DAT_1008ead0: *mut u8;
    static mut DAT_1008ead4: *mut u8;
    static mut DAT_1008ead8: *mut u8;
    static mut DAT_100c41e6: u16;
    static mut DAT_100c42a9: u8;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f7d0(this: *mut u8, param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10011d20() {
    mech_trace::enter!("10011d20");
    // SAFETY: the globals and the callee are the game's own; the object
    // pointers are passed through unchanged as the original does.
    unsafe {
        let mut selected_object = mech_trace::gr!("10011d20", "1008eac4", DAT_1008eac4);
        let mut hit = mech_trace::gr!("10011d20", "100c42a9", DAT_100c42a9) == 1;
        if !hit {
            selected_object = mech_trace::gr!("10011d20", "1008eac8", DAT_1008eac8);
            hit = mech_trace::gr!("10011d20", "100c42a9", DAT_100c42a9) == 2;
        }
        if !hit {
            selected_object = mech_trace::gr!("10011d20", "1008eacc", DAT_1008eacc);
            hit = mech_trace::gr!("10011d20", "100c42a9", DAT_100c42a9) == 3;
        }
        if hit {
            mech_trace::cr!("10011d20", "1005f7d0", FUN_1005f7d0(selected_object, 1));
        }
        if mech_trace::gr!("10011d20", "100c41e6", DAT_100c41e6) == 1 {
            mech_trace::cr!(
                "10011d20",
                "1005f7d0",
                FUN_1005f7d0(mech_trace::gr!("10011d20", "1008ead0", DAT_1008ead0), 1)
            );
            return;
        }
        if mech_trace::gr!("10011d20", "100c41e6", DAT_100c41e6) == 2 {
            mech_trace::cr!(
                "10011d20",
                "1005f7d0",
                FUN_1005f7d0(mech_trace::gr!("10011d20", "1008ead4", DAT_1008ead4), 1)
            );
            return;
        }
        if mech_trace::gr!("10011d20", "100c41e6", DAT_100c41e6) == 3 {
            mech_trace::cr!(
                "10011d20",
                "1005f7d0",
                FUN_1005f7d0(mech_trace::gr!("10011d20", "1008ead8", DAT_1008ead8), 1)
            );
            return;
        }
    }
}
