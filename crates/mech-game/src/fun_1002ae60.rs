// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002ab38(param_1: i32) -> i32;
    fn FUN_1002abf3(param_1: i32, param_2: u16, param_3: i16, param_4: u16) -> u32;
    fn FUN_10008156(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002ae60(param_1: i32, param_2: i16) -> u32 {
    // SAFETY: globals and callees are the game's own; iVar1 points at a game
    // structure whose field at +0xdc is read where the original reads it.
    unsafe {
        let mut local_c: i16 = -1;
        let i_var1 = (&raw mut DAT_100eef40)
            .cast::<u32>()
            .add(DAT_100adf38 as usize)
            .read_unaligned() as i32;
        let u_var2 = FUN_1002ab38(param_1) as u32;
        if u_var2 == 0xffff_ffff {
            return 0;
        }
        let field = (i_var1 as u32).wrapping_add(0xdc) as *const i32;
        let mut local_14: u16 = 0x1000;
        match param_2 {
            2 => {
                if field.read_unaligned() != 0 {
                    local_14 = field.read_unaligned() as u16;
                }
                if (local_14 & 0x200) != 0 && (((local_14 as i16 as i32) & 0xff) as u32) == u_var2 {
                    local_14 = 0x1000;
                }
                if (local_14 & 0x100) != 0 {
                    local_14 = 0xffff;
                }
                local_c = 1;
            }
            3 => {
                local_14 = 0;
            }
            4 | 6 | 9 | 10 => {}
            5 => {
                local_14 = 0x2201;
                local_c = 2;
            }
            7 => {
                if field.read_unaligned() != 0 {
                    local_14 = field.read_unaligned() as u16;
                }
                local_c = 3;
                if (local_14 & 0x200) != 0 && (((local_14 as i16 as i32) & 0xff) as u32) == u_var2 {
                    local_14 = 0x1000;
                }
            }
            8 => {
                local_14 = 0x2100;
                local_c = 4;
            }
            0xb => {
                local_14 = 0;
                local_c = 9;
            }
            _ => {}
        }
        if (local_14 & 0x1000) == 0 {
            if u_var2 == DAT_100adf38 {
                FUN_1002abf3(i_var1, 0x4200, param_2, local_14);
            } else {
                FUN_1002abf3(i_var1, (u_var2 as u16) | 0x200, param_2, local_14);
            }
            if local_c != -1 {
                FUN_10008156(local_c as i32, param_1);
            }
            1
        } else {
            0
        }
    }
}
