// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002950f(param_1: i32, param_2: i16) -> bool;
    fn FUN_1002a132(param_1: i32) -> i32;
    fn FUN_10022b70(param_1: i32, param_2: i32) -> i32;
    fn FUN_10044bba(param_1: i32, param_2: i32) -> u32;
    fn FUN_10043e6b(param_1: i32) -> u32;
    fn FUN_1004403d(param_1: i32, param_2: u8) -> u8;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_100297b4(param_1: i32) -> i32;
    fn FUN_10043f2c(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004373c(param_1: i32, param_2: i16) -> bool {
    let mut b_var2 = false;
    // SAFETY: the callees and the global are the game's own; param_1 is the
    // address of a game object whose fields are read and written at the
    // offsets the original uses.
    unsafe {
        FUN_1002950f(param_1, param_2);
        let mut i_var1 = FUN_1002a132(param_1);
        FUN_10022b70(param_1, i_var1);
        i_var1 = ((param_1 as u32).wrapping_add(0x1d4) as *const i32).read_unaligned();
        if i_var1 == 0 {
            let r = FUN_10044bba(param_1, 0x28);
            if r == 0 || FUN_10043e6b(param_1) == 0 {
                FUN_1004403d(param_1, 0);
                ((param_1 as u32).wrapping_add(0x1d4) as *mut u32).write_unaligned(1);
                let r = FUN_10044bba(param_1, 0x14);
                if r != 0 && FUN_10051f43(4) == 0 {
                    ((param_1 as u32).wrapping_add(0x1c8) as *mut u32).write_unaligned(5);
                }
                let g = DAT_100b38fc;
                ((param_1 as u32).wrapping_add(0x1d0) as *mut u32)
                    .write_unaligned(g.wrapping_add(0xb5));
            } else {
                FUN_1004403d(param_1, 1);
            }
        } else if i_var1 == 1 {
            let v = ((param_1 as u32).wrapping_add(0x1d0) as *const i32).read_unaligned();
            if v < DAT_100b38fc as i32 {
                ((param_1 as u32).wrapping_add(0x1d4) as *mut u32).write_unaligned(2);
                FUN_1004403d(param_1, 0);
            } else {
                FUN_1004403d(param_1, 1);
            }
            FUN_100297b4(param_1);
        } else if i_var1 == 2 {
            let r = FUN_10043f2c(param_1);
            b_var2 = r != 0;
        }
    }
    b_var2
}
