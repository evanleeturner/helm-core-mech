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
    // SAFETY: the globals and the callee are the game's own; the object
    // pointers are passed through unchanged as the original does.
    unsafe {
        let mut selected_object = DAT_1008eac4;
        let mut hit = DAT_100c42a9 == 1;
        if !hit {
            selected_object = DAT_1008eac8;
            hit = DAT_100c42a9 == 2;
        }
        if !hit {
            selected_object = DAT_1008eacc;
            hit = DAT_100c42a9 == 3;
        }
        if hit {
            FUN_1005f7d0(selected_object, 1);
        }
        if DAT_100c41e6 == 1 {
            FUN_1005f7d0(DAT_1008ead0, 1);
            return;
        }
        if DAT_100c41e6 == 2 {
            FUN_1005f7d0(DAT_1008ead4, 1);
            return;
        }
        if DAT_100c41e6 == 3 {
            FUN_1005f7d0(DAT_1008ead8, 1);
            return;
        }
    }
}
