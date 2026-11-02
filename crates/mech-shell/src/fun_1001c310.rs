// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008ff64: u32;
    static mut DAT_1008ff68: u32;
    static mut DAT_1008ff6c: u32;
    static mut DAT_1008ff70: u32;
    static mut DAT_1008ff74: u32;
    static mut DAT_1008ff78: u32;
    static mut DAT_1008ff7c: u32;
    static mut DAT_1008ff80: u32;
    static mut DAT_1008ff84: u32;
    static mut DAT_1008ff88: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001c310() {
    // SAFETY: the globals are the game's own memory, written in the original's order.
    unsafe {
        DAT_1008ff64 = 0;
        DAT_1008ff6c = 0;
        DAT_1008ff70 = 0;
        DAT_1008ff74 = 0;
        DAT_1008ff78 = 0;
        DAT_1008ff7c = 0;
        DAT_1008ff80 = 0;
        DAT_1008ff84 = 0;
        DAT_1008ff88 = 0;
        DAT_1008ff68 = 0;
    }
}
