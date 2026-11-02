// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aae80: u32;
    static mut DAT_100ada50: i32;
    static mut DAT_100ada68: u32;
    static mut DAT_10168900: u8;
    static mut DAT_10168904: u8;
    static mut DAT_10168908: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10031a97(param_1: i32, param_2: i32) -> u8;
    fn FUN_1007d0a4(param_1: i32) -> i32;
    fn FUN_100312ba(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031b85(param_1: i32) {
    // SAFETY: the globals and callees are the game's own; the table entries
    // are read at the same offsets and order as the original.
    unsafe {
        if -1 < param_1 && param_1 < DAT_100ada50 {
            let i_var1 = param_1.wrapping_mul(0x34);
            let kind = (((&raw mut DAT_10168904) as i32).wrapping_add(i_var1) as usize
                as *const u32)
                .read_unaligned();
            match kind {
                1 => {
                    if DAT_100ada68 != 0 && DAT_100aae80 == 0 {
                        let p = (((&raw mut DAT_10168908) as i32).wrapping_add(i_var1) as usize
                            as *const i32)
                            .read_unaligned();
                        let v = (p.wrapping_add(0x40) as usize as *const i32).read_unaligned();
                        FUN_10031a97(((&raw mut DAT_10168900) as i32).wrapping_add(i_var1), v);
                    }
                }
                2 | 6 => {
                    let i_var2 = FUN_1007d0a4(-1);
                    if i_var2 != 0 && DAT_100aae80 == 0 {
                        FUN_100312ba(((&raw mut DAT_10168900) as i32).wrapping_add(i_var1));
                    }
                }
                _ => {}
            }
        }
    }
}
