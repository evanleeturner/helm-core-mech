// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b06b0: u32;
    static mut DAT_100ba96c: u16;
    static mut DAT_100ba9e8: u32;
    static mut DAT_100ba9ec: u32;
    static mut DAT_100ba9f2: u8;
    static mut DAT_100ba9f5: u8;
    static mut DAT_100ba9f6: u8;
    static mut DAT_100ba9f7: u8;
    static mut DAT_10160be4: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004986d() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036a37(param_1: u32) -> i32;
    fn FUN_10036a72(param_1: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10048a80() {
    // SAFETY: the globals and callees are the game's own; the pointer returned
    // by FUN_1004986d is read at offset 4 as the original does.
    unsafe {
        DAT_10160be4 = 0;
        let i_var2 = FUN_1004986d();
        if i_var2 == 0 {
            return;
        }
        let u_var1: u32 = ((i_var2 as u32).wrapping_add(4) as *const u32).read_unaligned();
        if DAT_100ba96c == 0 {
            if (u_var1 & 1) != 0 {
                if DAT_100ba9f6 == 0 {
                    if DAT_100ba9f7 != 0 {
                        DAT_10160be4 = 0x1b;
                    }
                } else {
                    DAT_10160be4 = 0xd;
                }
                if DAT_10160be4 == 0 && FUN_10036a37(DAT_100b06b0) > 0x5a {
                    if (DAT_100ba9e8 as i32) < -0x2000 {
                        DAT_10160be4 = 0xc6;
                    } else if 0x2000 < DAT_100ba9e8 {
                        DAT_10160be4 = 199;
                    }
                }
                if DAT_10160be4 == 0 && FUN_10036a37(DAT_100b06b0) > 0x2d {
                    if (DAT_100ba9ec as i32) < -0x2000 {
                        DAT_10160be4 = 0xc9;
                    } else if 0x2000 < DAT_100ba9ec {
                        DAT_10160be4 = 0x20;
                    }
                }
            }
        } else {
            if DAT_100ba96c == 0x1b {
                DAT_10160be4 = 0x1b;
            } else if DAT_100ba96c < 0x30 || 0x39 < DAT_100ba96c {
                if (u_var1 & 1) != 0 {
                    let d = DAT_100ba96c;
                    if d < 0xe {
                        if d == 0xd || d == 9 {
                            DAT_10160be4 = d as u32;
                        }
                    } else if d < 0xca {
                        if 0xc5 < d || d == 0x20 {
                            DAT_10160be4 = d as u32;
                        }
                    } else if d == 0x209 {
                        DAT_10160be4 = d as u32;
                    }
                }
            } else {
                DAT_10160be4 = DAT_100ba96c as u32;
            }
            if DAT_100ba96c as u32 == DAT_10160be4 {
                DAT_100ba96c = 0;
            }
        }
        if DAT_10160be4 != 0 && (u_var1 & 1) != 0 {
            FUN_10036a72(DAT_100b06b0);
            DAT_100ba9f2 = 1;
            DAT_100ba9f5 = 1;
        }
    }
}
