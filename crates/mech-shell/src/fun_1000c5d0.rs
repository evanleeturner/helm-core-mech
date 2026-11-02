// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a67bc: u32;
    static mut DAT_100a67c0: u32;
    static mut DAT_100a67c4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
    fn FUN_1000c6b0(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c5d0(param_1: i32, param_2: i32) -> *mut i32 {
    // SAFETY: the globals and callees are the game's own; the collections and
    // items they yield are read where the original reads them.
    unsafe {
        let mut i_var2: i32 = 0;
        if 0 < ((DAT_100a67c4.wrapping_add(4)) as *const i32).read_unaligned() {
            loop {
                let pi_var1 = FUN_10018060(DAT_100a67c4 as i32, i_var2) as *mut i32;
                if pi_var1.read_unaligned() == param_1 {
                    FUN_1000c6b0(pi_var1 as i32, param_2);
                    return pi_var1;
                }
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= ((DAT_100a67c4.wrapping_add(4)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
        i_var2 = 0;
        if 0 < ((DAT_100a67bc.wrapping_add(4)) as *const i32).read_unaligned() {
            loop {
                let pi_var1 = FUN_10018060(DAT_100a67bc as i32, i_var2) as *mut i32;
                if pi_var1.read_unaligned() == param_1 {
                    FUN_1000c6b0(pi_var1 as i32, param_2);
                    return pi_var1;
                }
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= ((DAT_100a67bc.wrapping_add(4)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
        i_var2 = 0;
        if 0 < ((DAT_100a67c0.wrapping_add(4)) as *const i32).read_unaligned() {
            loop {
                let pi_var1 = FUN_10018060(DAT_100a67c0 as i32, i_var2) as *mut i32;
                if pi_var1.read_unaligned() == param_1 {
                    FUN_1000c6b0(pi_var1 as i32, param_2);
                    return pi_var1;
                }
                i_var2 = i_var2.wrapping_add(1);
                if i_var2 >= ((DAT_100a67c0.wrapping_add(4)) as *const i32).read_unaligned() {
                    break;
                }
            }
        }
        core::ptr::null_mut()
    }
}
