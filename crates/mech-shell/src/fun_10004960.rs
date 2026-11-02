// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083778: u32;
    static mut DAT_1008379c: u8;
    static mut DAT_10083800: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10004960() {
    // SAFETY: the globals are the game's own memory; the table at
    // DAT_1008379c is read as i32 entries up to the address DAT_10083800.
    unsafe {
        let mut p_var1 = (&raw mut DAT_1008379c) as *const i32;
        let end = (&raw mut DAT_10083800) as *const i32;
        DAT_10083778 = 0;
        loop {
            if p_var1.read_unaligned() == -1 {
                return;
            }
            p_var1 = p_var1.wrapping_add(1);
            DAT_10083778 = DAT_10083778.wrapping_add(100);
            if !(p_var1 < end) {
                break;
            }
        }
    }
}
