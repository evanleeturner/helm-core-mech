// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008f630: u32;
    static mut DAT_1008f634: u32;
    static mut DAT_1008f638: u32;
    static mut DAT_1008f63c: u32;
    static mut DAT_1008f640: u32;
    static mut DAT_1008f650: u32;
    static mut DAT_1008f654: u32;
    static mut DAT_1008f658: u32;
    static mut DAT_1008f65c: u32;
    static mut DAT_1008f660: u32;
    static mut DAT_1008f664: u32;
    static mut DAT_1008f5f8: u32;
    static mut DAT_1008f5fc: u32;
    static mut DAT_1008f600: u32;
    static mut DAT_1008f604: u32;
    static mut DAT_1008f608: u32;
    static mut DAT_1008f60c: u32;
    static mut DAT_1008f610: u32;
    static mut DAT_1008f614: u32;
    static mut DAT_1008f618: u32;
    static mut DAT_1008f61c: u32;
    static mut DAT_1008f620: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100181d0() {
    // SAFETY: the globals are the game's own memory; each is written
    // with zero in the original's order.
    unsafe {
        DAT_1008f630 = 0;
        DAT_1008f634 = 0;
        DAT_1008f638 = 0;
        DAT_1008f63c = 0;
        DAT_1008f640 = 0;
        DAT_1008f650 = 0;
        DAT_1008f654 = 0;
        DAT_1008f658 = 0;
        DAT_1008f65c = 0;
        DAT_1008f660 = 0;
        DAT_1008f664 = 0;
        DAT_1008f5f8 = 0;
        DAT_1008f5fc = 0;
        DAT_1008f600 = 0;
        DAT_1008f604 = 0;
        DAT_1008f608 = 0;
        DAT_1008f60c = 0;
        DAT_1008f610 = 0;
        DAT_1008f614 = 0;
        DAT_1008f618 = 0;
        DAT_1008f61c = 0;
        DAT_1008f620 = 0;
    }
}
