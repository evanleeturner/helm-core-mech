// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008f2f8: u32;
    static mut DAT_1008f2fc: u32;
    static mut DAT_1008f300: u32;
    static mut DAT_1008f304: u32;
    static mut DAT_1008f308: u32;
    static mut DAT_1008f30c: u32;
    static mut DAT_1008f310: u32;
    static mut DAT_1008f324: u32;
    static mut DAT_1008f328: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10016d40() {
    // SAFETY: the globals are the game's own memory, written in the original's order.
    unsafe {
        DAT_1008f2f8 = 0;
        DAT_1008f2fc = 0;
        DAT_1008f300 = 0;
        DAT_1008f304 = 0;
        DAT_1008f308 = 0;
        DAT_1008f30c = 0;
        DAT_1008f310 = 0;
        DAT_1008f324 = 0;
        DAT_1008f328 = 0xffffffff;
    }
}
