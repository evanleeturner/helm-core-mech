// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67a4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000ad70(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000ce60(param_1: i32) -> i32 {
    // SAFETY: the globals and callees are the game's own; the collection
    // pointer and item pointers are read where the original reads them.
    unsafe {
        let mut i_var1: i32 = 0;
        let mut i_var2: i32 = 0;
        if 0 < ((DAT_100a67a4 as usize + 4) as *const i32).read_unaligned() {
            loop {
                i_var1 = FUN_10018060(DAT_100a67a4 as i32, i_var2) as i32;
                if ((i_var1 as u32).wrapping_add(0x2064) as *const i8).read() == 1 {
                    break;
                }
                i_var2 = i_var2.wrapping_add(1);
                if ((DAT_100a67a4 as usize + 4) as *const i32).read_unaligned() <= i_var2 {
                    return 0;
                }
            }
            i_var1 = FUN_1000ad70(
                param_1,
                ((i_var1 as u32).wrapping_add(0x24) as *const i32).read_unaligned(),
            );
        }
        i_var1
    }
}
