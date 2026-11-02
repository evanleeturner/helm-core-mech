// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67a8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004b140() -> i32 {
    // SAFETY: the global and the callee are the game's own; the collection
    // pointer and the items it returns are read where the original reads them.
    unsafe {
        let mut i_var2: i32 = 0;
        let mut i_var1: i32 = 0;
        if 0 < ((DAT_100a67a8 as usize + 4) as *const i32).read_unaligned() {
            loop {
                let i_var3 = i_var1.wrapping_add(1);
                let item = FUN_10018060(DAT_100a67a8 as i32, i_var1);
                i_var2 =
                    i_var2.wrapping_add(((item as usize + 0x24) as *const i32).read_unaligned());
                i_var1 = i_var3;
                if !(i_var3 < ((DAT_100a67a8 as usize + 4) as *const i32).read_unaligned()) {
                    break;
                }
            }
        }
        i_var2
    }
}
