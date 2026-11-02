// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1016765c: i32;
    static mut DAT_100adcec: i32;
    static mut DAT_10168288: i32;
    static mut DAT_10167660: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035f13() {
    // SAFETY: the globals are the game's own memory, read and written
    // where the original does.
    unsafe {
        DAT_10168288 = DAT_1016765c;
        DAT_10167660 = (DAT_100adcec as u32)
            .wrapping_add(DAT_1016765c as u32)
            .wrapping_add(0xffffffd0u32) as i32;
    }
}
