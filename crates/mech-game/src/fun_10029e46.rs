// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100ad0a4: i32;
    static mut DAT_100eef40: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004fefa(param_1: i32) -> i32;
    fn FUN_1004d47f(param_1: i32) -> i32;
    fn FUN_1004d5e3(param_1: i32) -> i32;
    fn FUN_1004d69b(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10029e46(param_1: i32, param_2: u16, mut param_3: u16) -> u32 {
    mech_trace::enter!("10029e46", param_1, param_2, param_3);
    let mut local_8: u16 = 0xffff;
    // SAFETY: param_1 points at the caller's game object; fields are read where the original does.
    unsafe {
        let i_var4: i32 = mech_trace::rd!(
            "10029e46",
            ((param_1 as usize).wrapping_add(4) as *const i32),
            read_unaligned
        );
        if (param_2 & 0x6000) == 0 {
            if ((param_2 ^ (param_3 & 0xfff)) & 0xfff) == 0 {
                return mech_trace::rt!("10029e46", 0xffff);
            } else {
                return mech_trace::rt!("10029e46", param_2 as u32);
            }
        }
        if param_3 != 0xffff {
            if (param_2 & 0x2000) != 0 {
                return mech_trace::rt!(
                    "10029e46",
                    ((((param_2 as i16) >> 0xf) as u16 as u32) << 16) | 0xffff
                );
            }
            param_3 &= 0xff;
        }
        let mut u_var1: u16 = local_8;
        if (param_2 & 0x200) == 0 {
            if (param_2 & 0x400) == 0 && (param_2 & 0x100) != 0 {
                if param_2 == 0x2100 {
                    let v = mech_trace::rd!(
                        "10029e46",
                        ((param_1 as usize).wrapping_add(8) as *const i32),
                        read_unaligned
                    );
                    u_var1 = mech_trace::cr!("10029e46", "1004fefa", FUN_1004fefa(v)) as u16;
                } else if param_2 == 0x2101 {
                    let tmp = mech_trace::rd!(
                        "10029e46",
                        ((param_1 as usize).wrapping_add(0x1a2) as *const i32),
                        read_unaligned
                    );
                    u_var1 = tmp as u16;
                }
            }
        } else if (param_2 & 0x4000) == 0 {
            if param_2 == 0x2200 {
                u_var1 = mech_trace::gr!("10029e46", "100adf38", DAT_100adf38) as u16;
            } else if param_2 == 0x2201 {
                let v = mech_trace::rd!(
                    "10029e46",
                    ((param_1 as usize).wrapping_add(8) as *const i32),
                    read_unaligned
                );
                u_var1 = mech_trace::cr!("10029e46", "1004d47f", FUN_1004d47f(v)) as u16;
            } else if param_2 == 0x2202 {
                u_var1 = mech_trace::rd!(
                    "10029e46",
                    ((param_1 as usize).wrapping_add(4) as *const i32),
                    read_unaligned
                ) as u16;
            }
        } else if param_2 == 0x4200 {
            loop {
                param_3 = param_3.wrapping_add(1);
                u_var1 = local_8;
                if mech_trace::gr!("10029e46", "100b38d8", DAT_100b38d8) <= (param_3 as i16) as i32
                {
                    break;
                }
                let base = (&raw mut DAT_100eef40).cast::<i32>();
                let obj = mech_trace::rd!(
                    "10029e46",
                    base.offset((param_3 as i16) as isize),
                    read_unaligned
                );
                let v = mech_trace::rd!(
                    "10029e46",
                    ((obj as usize).wrapping_add(8) as *const i32),
                    read_unaligned
                );
                let r = mech_trace::cr!("10029e46", "1004d47f", FUN_1004d47f(v));
                u_var1 = param_3;
                if r == mech_trace::gr!("10029e46", "100adf38", DAT_100adf38) {
                    break;
                }
                if mech_trace::gr!("10029e46", "100ad0a4", DAT_100ad0a4) != 0 {
                    let r2 = mech_trace::cr!(
                        "10029e46",
                        "1004d5e3",
                        FUN_1004d5e3((param_3 as i16) as i32)
                    );
                    u_var1 = param_3;
                    if r2 == 0 {
                        break;
                    }
                }
            }
        } else if param_2 == 0x4201 {
            loop {
                param_3 = param_3.wrapping_add(1);
                u_var1 = local_8;
                if mech_trace::gr!("10029e46", "100b38d8", DAT_100b38d8) <= (param_3 as i16) as i32
                {
                    break;
                }
                let b = mech_trace::cr!(
                    "10029e46",
                    "1004d69b",
                    FUN_1004d69b((param_3 as i16) as i32, i_var4)
                );
                if b == 0 {
                    let r = mech_trace::cr!(
                        "10029e46",
                        "1004d5e3",
                        FUN_1004d5e3((param_3 as i16) as i32)
                    );
                    u_var1 = param_3;
                    if r != 2 {
                        break;
                    }
                }
            }
        }
        local_8 = u_var1;
        let mut u_var6: u32 = (local_8 as i16) as i32 as u32;
        if u_var6 != 0xffff_ffff {
            u_var6 = (local_8 as i16) as i32 as u32;
            if (u_var6 & 0xf00) == 0 {
                u_var6 =
                    (((param_2 as i16) as i32 as u32) & 0xf00) | ((local_8 as i16) as i32 as u32);
                local_8 = u_var6 as u16;
            }
        }
        mech_trace::rt!(
            "10029e46",
            (((u_var6 >> 0x10) as u16 as u32) << 16) | (local_8 as u32)
        )
    }
}
