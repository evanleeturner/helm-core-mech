// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a57d8: [u8; 0];
    static mut DAT_100c6070: [u32; 0];
    static mut DAT_100c607c: u32;
    static mut DAT_100c6100: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b8e0(param_1: u32, param_2: u32, param_3: i32) {
    let mut u_var3: u32 = (param_2 & 0xff0000) >> 0x10;
    // SAFETY: the globals are the game's own memory; the table at
    // DAT_100a57d8 and the bitmap at DAT_100c6070 are read and written
    // at the offsets the original uses.
    unsafe {
        if u_var3 != 0 {
            if ((param_2 >> 0x10) & 0x100) != 0 && u_var3 < 0x59 {
                u_var3 = (&raw mut DAT_100a57d8)
                    .cast::<u8>()
                    .byte_add(u_var3.wrapping_mul(4) as usize)
                    .cast::<u32>()
                    .read_unaligned();
            }
            if u_var3 != 0 {
                let i_var4: i32 = u_var3.wrapping_sub(1) as i32;
                let adj: u32 = ((i_var4 >> 0x1f) as u32) & 0x1f;
                let i_var1: i32 = (i_var4.wrapping_add(adj as i32)) >> 5;
                let p = (&raw mut DAT_100c6070)
                    .cast::<u32>()
                    .offset(i_var1 as isize);
                let bit: u32 = 1u32.wrapping_shl(((i_var4 as u8) & 0x1f) as u32);
                let u_var2: u32 = if param_3 == 0 {
                    p.read_unaligned() & !bit
                } else {
                    p.read_unaligned() | bit
                };
                p.write_unaligned(u_var2);
                match u_var3 {
                    0x1d | 0x70 => {
                        if param_3 == 0 {
                            DAT_100c607c &= 0xff7fffff;
                        } else {
                            DAT_100c607c |= 0x800000;
                        }
                    }
                    0x2a | 0x36 => {
                        if param_3 == 0 {
                            DAT_100c607c &= 0xffbfffff;
                        } else {
                            DAT_100c607c |= 0x400000;
                        }
                    }
                    0x38 | 0x71 => {
                        if param_3 == 0 {
                            DAT_100c607c &= 0xfeffffff;
                        } else {
                            DAT_100c607c |= 0x1000000;
                        }
                    }
                    _ => {}
                }
            }
        }
        DAT_100c6100 = param_1;
    }
}
