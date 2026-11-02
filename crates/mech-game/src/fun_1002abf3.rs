// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: [i32; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10029e46(param_1: i32, param_2: u16, param_3: u16) -> u32;
    fn FUN_1002a446(param_1: i32, param_2: i16) -> i32;
    fn FUN_1002a4e0(param_1: i32, param_2: i16, param_3: u16, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002abf3(
    param_1: i32,
    param_2: u16,
    param_3: i16,
    param_4: u16,
) -> u32 {
    mech_trace::enter!("1002abf3", param_1, param_2, param_3, param_4);
    let mut local_1c: u16 = 0xffff;
    let mut local_8: u16 = 0;
    let mut b_var3: bool = true;
    let b_var2: bool = param_2 == 0x4200;
    // SAFETY: the callees and globals are the game's own; iVar1 points at a
    // game object whose fields are read and written at the original offsets.
    unsafe {
        loop {
            let u_var4 = mech_trace::cr!(
                "1002abf3",
                "10029e46",
                FUN_10029e46(param_1, param_2, local_1c)
            );
            local_1c = u_var4 as u16;
            if local_1c == 0xffff {
                break;
            }
            if (u_var4 & 0x200) != 0 {
                if local_1c == param_4 {
                    local_8 = local_8.wrapping_add(1);
                } else {
                    let i_var1: i32 = mech_trace::gr!(
                        "1002abf3",
                        "100eef40",
                        DAT_100eef40[(((local_1c as i16) as i32) & 0xff) as usize]
                    );
                    let p = i_var1 as usize as *mut u8;
                    let b_var6 =
                        mech_trace::rd!("1002abf3", p.byte_add(0xc).cast::<i32>(), read_unaligned)
                            == 4;
                    if param_3 == 2 || param_3 == 5 {
                        if b_var2 && b_var6 {
                            b_var3 = false;
                        }
                    } else if b_var6 {
                        b_var3 = false;
                    }
                    if mech_trace::rd!("1002abf3", p.byte_add(4).cast::<i32>(), read_unaligned)
                        != mech_trace::gr!("1002abf3", "100adf38", DAT_100adf38)
                        && b_var3
                    {
                        if mech_trace::rd!(
                            "1002abf3",
                            p.byte_add(0x176).cast::<i16>(),
                            read_unaligned
                        ) == 0xc
                            || mech_trace::cr!(
                                "1002abf3",
                                "1002a446",
                                FUN_1002a446(i_var1, param_3)
                            ) == 0
                        {
                            local_8 = local_8.wrapping_add(1);
                        } else if param_3 == 3 {
                            let v182 = mech_trace::rd!(
                                "1002abf3",
                                p.byte_add(0x182).cast::<i32>(),
                                read_unaligned
                            );
                            let t: u32 = if v182 == 0 { 1 } else { 0 };
                            mech_trace::wr!(
                                "1002abf3",
                                p.byte_add(0x182).cast::<u32>(),
                                write_unaligned,
                                t
                            );
                            let t = mech_trace::rd!(
                                "1002abf3",
                                p.byte_add(0x17e).cast::<u32>(),
                                read_unaligned
                            ) & 0xffff_fffc;
                            mech_trace::wr!(
                                "1002abf3",
                                p.byte_add(0x17e).cast::<u32>(),
                                write_unaligned,
                                t
                            );
                        } else {
                            mech_trace::wr!(
                                "1002abf3",
                                p.byte_add(0x17e).cast::<u32>(),
                                write_unaligned,
                                0
                            );
                            if param_3 == 7 {
                                mech_trace::cr!(
                                    "1002abf3",
                                    "1002a4e0",
                                    FUN_1002a4e0(i_var1, 7, param_4, 0)
                                );
                            } else {
                                mech_trace::cr!(
                                    "1002abf3",
                                    "1002a4e0",
                                    FUN_1002a4e0(i_var1, param_3, param_4, 1)
                                );
                            }
                            if param_3 == 2 {
                                let t = mech_trace::rd!(
                                    "1002abf3",
                                    p.byte_add(0x17e).cast::<u32>(),
                                    read_unaligned
                                ) | 0x11;
                                mech_trace::wr!(
                                    "1002abf3",
                                    p.byte_add(0x17e).cast::<u32>(),
                                    write_unaligned,
                                    t
                                );
                            } else {
                                if param_3 == 5 {
                                    mech_trace::wr!(
                                        "1002abf3",
                                        p.byte_add(0x182).cast::<u32>(),
                                        write_unaligned,
                                        0
                                    );
                                    let t = mech_trace::rd!(
                                        "1002abf3",
                                        p.byte_add(0x17e).cast::<u32>(),
                                        read_unaligned
                                    ) | 0x11;
                                    mech_trace::wr!(
                                        "1002abf3",
                                        p.byte_add(0x17e).cast::<u32>(),
                                        write_unaligned,
                                        t
                                    );
                                }
                                let t = mech_trace::rd!(
                                    "1002abf3",
                                    p.byte_add(0x17e).cast::<u32>(),
                                    read_unaligned
                                ) | 0x12;
                                mech_trace::wr!(
                                    "1002abf3",
                                    p.byte_add(0x17e).cast::<u32>(),
                                    write_unaligned,
                                    t
                                );
                            }
                        }
                    }
                }
            }
        }
    }
    mech_trace::rt!("1002abf3", 0xffff_0000u32 | u32::from(local_8))
}
