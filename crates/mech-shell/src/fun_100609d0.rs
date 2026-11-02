// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c4af8: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100609d0(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    if param_2 < 8 {
        let mut i_var2: i32 = param_2.wrapping_mul(0xc);
        let mut i_var1: i32 = param_3;
        loop {
            loop {
                i_var1 = i_var1.wrapping_add(1);
                if i_var1 >= 0xc {
                    break;
                }
                let idx = i_var2.wrapping_add(i_var1);
                let addr = param_1
                    .wrapping_add(0x368)
                    .wrapping_add(idx.wrapping_mul(4));
                // SAFETY: the table and the flag array are the game's own memory;
                // the read and write happen where the original does.
                unsafe {
                    if (addr as usize as *const i32).read_unaligned() == param_4 {
                        (&raw mut DAT_100c4af8).byte_offset(idx as isize).write(1);
                    }
                }
            }
            i_var2 = i_var2.wrapping_add(0xc);
            i_var1 = param_3;
            if i_var2 >= 0x60 {
                break;
            }
        }
    }
}
