// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a553c: u32;
    static mut DAT_100aa1ac: u32;
    static mut DAT_100aa1f4: u32;
    static mut DAT_100aafa4: u32;
    static mut DAT_100ab09c: u32;
    static mut DAT_100ab318: u32;
    static mut DAT_100ad6e0: u32;
    static mut DAT_100adf28: u32;
    static mut DAT_100adf2c: u32;
    static mut DAT_100adf34: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100adf88: u32;
    static mut DAT_100b09f0: i32;
    static mut DAT_100b09f4: i32;
    static mut DAT_100b0a04: u32;
    static mut DAT_100b0a1c: u32;
    static mut DAT_100b0a20: u32;
    static mut DAT_100b38fc: i32;
    static mut DAT_100b9420: u32;
    static mut DAT_100ba96c: u16;
    static mut DAT_100ba9ca: u8;
    static mut DAT_100ba9cd: u8;
    static mut DAT_100ba9e0: u8;
    static mut DAT_100b0de8: i8;
    static mut DAT_100b0e00: i8;
    static mut DAT_100b0e18: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007c93(param_1: i32) -> u8;
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1001a7d4(param_1: i32, param_2: i32) -> u8;
    fn FUN_1000242a(param_1: u32, param_2: i32) -> u8;
    fn FUN_10052235(param_1: u16) -> u8;
    fn FUN_100528ee(param_1: u32) -> u32;
    fn FUN_10088ea8(param_1: i16) -> u32;
    fn FUN_100482cb(param_1: i32) -> u32;
    fn FUN_10071150(param_1: i32, param_2: i32, param_3: u32, param_4: u32) -> u32;
    fn FUN_1001a5e1(param_1: i32) -> u8;
    fn FUN_10053021(param_1: u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10052b78() {
    // SAFETY: all globals and callees are the game's own; the pointer read
    // at DAT_100adf34 + 8 is the game's object, read where the original does.
    unsafe {
        if DAT_100adf88 != 0 {
            return;
        }
        if 0 < DAT_100b09f4 && DAT_100b09f4 < DAT_100b38fc {
            FUN_10007c93(1);
            DAT_100b09f4 = 0;
        }
        if DAT_100b09f0 == 0 && DAT_100b0a04 == 0 {
            if DAT_100b0a1c == 0 && DAT_100b9420 == 0 {
                if DAT_100ab318 != 0 {
                    DAT_100ad6e0 = 0;
                    FUN_1001a7d4(0, 1);
                    DAT_100aafa4 = 0;
                    FUN_1000242a(0, 0);
                    DAT_100b09f4 = DAT_100b38fc.wrapping_add(0x108);
                    if DAT_100aa1f4 == 0 {
                        DAT_100b0a04 = 1;
                        DAT_100b09f0 = DAT_100b38fc.wrapping_add(0x16a);
                        FUN_1005be24(&raw mut DAT_100b0e00, 1, 0x1536, 100);
                    } else {
                        DAT_100b09f0 = DAT_100b38fc.wrapping_add(0x43e);
                    }
                }
            } else {
                DAT_100b0a20 = 0;
                DAT_100b0a04 = 1;
                DAT_100b09f4 = DAT_100b38fc.wrapping_add(0x108);
                DAT_100b09f0 = DAT_100b38fc.wrapping_add(0x16a);
                FUN_1005be24(&raw mut DAT_100b0de8, 1, 0x1536, 100);
            }
        } else if 0 < DAT_100b09f0 && DAT_100b09f0 < DAT_100b38fc {
            DAT_100b0a04 = 1;
            if DAT_100aa1f4 == 0 || DAT_100b0a1c != 0 || DAT_100b9420 != 0 {
                DAT_100adf28 = 1;
                DAT_100adf2c = 0x29a;
            } else if *((DAT_100adf34.wrapping_add(8)) as *const i8) == 0 {
                DAT_100b09f0 = -1;
                FUN_1005be24(&raw mut DAT_100b0e18, 1, 0x58610, 100);
                DAT_100b0a20 = 1;
                DAT_100ab09c = 1;
            } else {
                DAT_100b09f0 = 0;
                DAT_100b0a20 = 2;
            }
        }
        let u_var1 = DAT_100ba96c;
        if DAT_100aa1ac == 0 {
            FUN_10052235(u_var1);
        }
        if DAT_100ad6e0 == 0 || FUN_100528ee(u_var1 as u32) == 0 {
            let mut u_var3 = FUN_10088ea8(u_var1 as i16);
            if (u_var3 as i16) != 0 {
                if DAT_100b0a04 == 0 {
                    FUN_10053021(u_var3 & 0xffff);
                } else if DAT_100aa1f4 == 0 || DAT_100b0a1c != 0 || DAT_100b9420 != 0 {
                    if (u_var3 & 0xffff) == 0x71 {
                        DAT_100adf28 = 1;
                        DAT_100adf2c = 0x29a;
                    }
                } else if *((DAT_100adf34.wrapping_add(8)) as *const i8) == 0 {
                    if DAT_100ab09c != 0 {
                        u_var3 &= 0xffff;
                        if u_var3 == 0x4a {
                            FUN_100482cb(4);
                        } else if u_var3 == 0x67 {
                            FUN_1001a7d4(1, 0);
                        } else if u_var3 == 0x71 {
                            DAT_100adf28 = 1;
                            DAT_100adf2c = 0x29a;
                        }
                    }
                } else {
                    u_var3 &= 0xffff;
                    if u_var3 == 0x4a {
                        FUN_100482cb(4);
                    } else if u_var3 == 0x67 {
                        DAT_100aafa4 = 0;
                        FUN_1000242a(0, 0);
                        FUN_10071150(DAT_100adf38 as i32, 0, 0, 0);
                        FUN_1001a5e1(0);
                        DAT_100ba9ca = 1;
                        DAT_100ba9cd = 1;
                        DAT_100ba9e0 = 1;
                        DAT_100a553c = 0;
                        DAT_100b09f4 = 0;
                        DAT_100b0a20 = 0;
                        DAT_100b09f0 = 0;
                        DAT_100b0a04 = 0;
                        DAT_100b0a1c = 0;
                        DAT_100ab09c = 0;
                    } else if u_var3 == 0x71 {
                        DAT_100adf28 = 1;
                        DAT_100adf2c = 0x29a;
                    }
                }
            }
        }
    }
}
