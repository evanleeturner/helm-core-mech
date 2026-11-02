// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3518: u32;
    static mut DAT_100b3520: u32;
    static mut DAT_100b3524: u32;
    static mut DAT_100b3528: u32;
    static mut DAT_100f3170: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005f69e() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005f462(param_1: i32, param_2: i32) -> u32 {
    let mut param_2 = param_2;
    let mut local_c: i32 = 0;
    let mut b_var1 = false;
    // SAFETY: the globals and the callee are the game's own; the table at
    // DAT_100f3170 is the game's array, indexed within its bounds as the
    // original does.
    unsafe {
        if DAT_100b3518 == 0xffff_fffe {
            FUN_1005f69e();
        }
        if param_2 == -1 {
            if DAT_100b3524 != 0 {
                DAT_100b3520 = DAT_100b3520.wrapping_add(1);
                DAT_100b3524 = 0;
            }
            param_2 = DAT_100b3520 as i32;
        }
        if param_1 < 1 || param_2 < 0 || 0x1ff < param_2 {
            0xffff_ffff
        } else {
            let base = (&raw mut DAT_100f3170).cast::<u8>();
            let row = param_2.wrapping_mul(0x100) as isize;
            if base.byte_offset(row).cast::<i16>().read_unaligned() < 1 {
                DAT_100b3528 = 0;
            } else {
                DAT_100b3528 = 1;
                while !b_var1 {
                    local_c = local_c.wrapping_add(1);
                    if 0x1f < local_c {
                        DAT_100b3528 = 1;
                        return 0xffff_ffff;
                    }
                    let off = (local_c.wrapping_mul(8) as isize).wrapping_add(row);
                    if base.byte_offset(off).cast::<i16>().read_unaligned() == -1 {
                        b_var1 = true;
                    }
                }
            }
            let off = (local_c.wrapping_mul(8) as isize).wrapping_add(row);
            base.byte_offset(off)
                .cast::<i16>()
                .write_unaligned(param_1 as i16);
            0
        }
    }
}
