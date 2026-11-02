// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008371c: i32;
    static mut DAT_10083720: i32;
    static mut DAT_10083724: i32;
    static mut DAT_10083728: i32;
    static mut DAT_1008372c: i32;
    static mut DAT_10083710: i32;
    static mut DAT_10084c58: i32;
    static mut DAT_10084c5c: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100046f0() {
    // SAFETY: the globals are the game's own memory; the table at
    // DAT_10084c58/DAT_10084c5c is indexed by byte offset as the original does.
    unsafe {
        let i_var1 = (DAT_1008371c % 10000).wrapping_mul(0x10) as isize;
        DAT_10083724 = (&raw mut DAT_10084c5c).byte_offset(i_var1).read_unaligned();
        DAT_10083720 = (&raw mut DAT_10084c58).byte_offset(i_var1).read_unaligned();
        if 9999 < DAT_1008371c {
            DAT_10083724 = DAT_10083724 / 2;
        }
        DAT_10083728 = ((DAT_1008371c < 10000) as i32).wrapping_sub(1) & 7;
        DAT_1008372c = DAT_10083710.wrapping_sub(DAT_10083724);
    }
}
