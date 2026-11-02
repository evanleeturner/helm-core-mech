// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6798: u32;
    static mut DAT_100a67b4: u32;
    static mut DAT_100a67b8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_100507a0(param_1: i32) -> i32;
    fn FUN_10050490(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d440() -> i32 {
    // SAFETY: the globals and callees are the game's own; the globals hold
    // pointers to collections whose count lives at offset 4.
    unsafe {
        let mut i_var2: i32 = 0;

        let mut i_var1: i32 = 0;
        if 0 < ((DAT_100a6798 as usize + 4) as *const i32).read_unaligned() {
            loop {
                let i_var3 = i_var1.wrapping_add(1);
                let item = FUN_10018060(DAT_100a6798 as i32, i_var1);
                let v = FUN_100507a0(item as i32);
                i_var2 = i_var2.wrapping_add(v);
                i_var1 = i_var3;
                if !(i_var3 < ((DAT_100a6798 as usize + 4) as *const i32).read_unaligned()) {
                    break;
                }
            }
        }

        i_var1 = 0;
        if 0 < ((DAT_100a67b4 as usize + 4) as *const i32).read_unaligned() {
            loop {
                let i_var3 = i_var1.wrapping_add(1);
                let item = FUN_10018060(DAT_100a67b4 as i32, i_var1);
                let v = FUN_10050490(item as i32);
                i_var2 = i_var2.wrapping_add(v as i32);
                i_var1 = i_var3;
                if !(i_var3 < ((DAT_100a67b4 as usize + 4) as *const i32).read_unaligned()) {
                    break;
                }
            }
        }

        i_var1 = 0;
        if 0 < ((DAT_100a67b8 as usize + 4) as *const i32).read_unaligned() {
            loop {
                let i_var3 = i_var1.wrapping_add(1);
                let item = FUN_10018060(DAT_100a67b8 as i32, i_var1);
                let v = FUN_10050490(item as i32);
                i_var2 = i_var2.wrapping_add(v as i32);
                i_var1 = i_var3;
                if !(i_var3 < ((DAT_100a67b8 as usize + 4) as *const i32).read_unaligned()) {
                    break;
                }
            }
        }

        i_var2
    }
}
