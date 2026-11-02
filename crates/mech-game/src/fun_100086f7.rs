// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7168: u32;
    static mut DAT_100a716c: u32;
    static mut DAT_100a7170: u32;
    static mut DAT_100a7174: u32;
    static mut DAT_100aafc4: u32;
    static mut DAT_100ab07c: u32;
    static mut DAT_100ab318: u32;
    static mut DAT_100adcd0: u32;
    static mut DAT_100adcd4: u32;
    static mut DAT_100adcd8: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_10215650: u32;
    static mut DAT_10215658: u32;
    static mut DAT_10215660: u32;
    static mut PTR_FUN_100a7164: u32;
    static mut PTR_PTR_100a7b40: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003742(param_1: u32, param_2: i32, param_3: u32, param_4: i32, param_5: u16) -> u8;
    fn FUN_100082c4(param_1: i32, param_2: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000895c();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100086f7() -> u32 {
    // SAFETY: all globals and callees are the game's own memory and code;
    // pointers derived from them are read where the original reads.
    unsafe {
        let mut local_8: u32 = 0;
        if DAT_10215658 < 3 && DAT_10215650 == 4 && DAT_100a7174 != 0 {
            DAT_100adcd4 = DAT_100a7168;
            DAT_100adcd8 = DAT_100a7170;
            DAT_100a7174 = 0;
        }
        if DAT_10215658 == 4 {
            let mut cond = DAT_100ab318 != 0;
            if !cond {
                let entry = (&raw mut DAT_100eef40)
                    .cast::<u32>()
                    .add(DAT_100adf38 as usize)
                    .read_unaligned();
                let flags = (entry.wrapping_add(0x14) as *const u16).read_unaligned();
                cond = (flags & 6) != 0;
            }
            if cond {
                DAT_10215660 = 5;
            }
        }
        if DAT_10215658 != DAT_10215660 {
            local_8 = 1;
            match DAT_10215660 {
                0 => {
                    DAT_10215650 = DAT_10215658;
                    DAT_10215658 = 0;
                }
                1 => {
                    DAT_10215650 = DAT_10215658;
                    DAT_10215658 = 1;
                }
                2 => {
                    DAT_10215650 = DAT_10215658;
                    DAT_10215658 = 2;
                }
                3 => {
                    DAT_10215660 = 4;
                    PTR_FUN_100a7164 = DAT_100aafc4;
                    DAT_100aafc4 = FUN_1000895c as usize as u32;
                    let v = (PTR_PTR_100a7b40.wrapping_add(0xc) as *const u32).read_unaligned();
                    if v != 0xffff_ffff {
                        FUN_10003742(v, 100, 0x40, 5, 0x50);
                    }
                    FUN_100082c4(0x12, -1);
                }
                4 => {
                    DAT_10215650 = DAT_10215658;
                    DAT_10215658 = 4;
                    if DAT_100a7174 == 0 {
                        DAT_100a7168 = DAT_100adcd4;
                        DAT_100adcd4 = 0;
                        DAT_100a7170 = DAT_100adcd8;
                        DAT_100adcd8 = 0;
                        DAT_100a716c = DAT_100adcd0;
                        DAT_100a7174 = 1;
                    }
                }
                5 => {
                    DAT_100aafc4 = PTR_FUN_100a7164;
                    DAT_10215660 = DAT_10215650;
                    let v = (PTR_PTR_100a7b40.wrapping_add(0x10) as *const u32).read_unaligned();
                    if v != 0xffff_ffff {
                        FUN_10003742(v, 100, 0x40, 5, 0x50);
                    }
                    if DAT_100ab07c == 1 {
                        FUN_100082c4(0x11, -1);
                    }
                }
                _ => {}
            }
        }
        local_8
    }
}
