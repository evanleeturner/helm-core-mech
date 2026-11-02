// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100305b5() -> *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f783(param_1: i32);
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32);
    fn FUN_1002fbc0(param_1: *mut i32);
    fn FUN_1007d0a4(param_1: i32) -> i32;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
}

#[inline(always)]
unsafe fn rd(addr: i32) -> u32 {
    // SAFETY: the caller passes an address into the game's own memory.
    unsafe { mech_trace::rd!("10030c40", (addr as *const u32), read_unaligned) }
}

#[inline(always)]
unsafe fn wr(addr: i32, v: u32) {
    // SAFETY: the caller passes an address into the game's own memory.
    unsafe { mech_trace::wr!("10030c40", (addr as *mut u32), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10030c40(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("10030c40", param_1, param_2);
    let mut local_8: u32 = 0xffffffff;
    let mut local_14: u32 = 0;
    let mut local_18: u32 = 0;
    let mut local_1c: u32 = 0;

    if param_1 < 0x40 && 0 < param_1 {
        // SAFETY: the callees, the task record they return and the pointers
        // inside it are the game's own memory, accessed as the original does.
        unsafe {
            match param_1 {
                1 => {
                    if param_2 == 0 {
                        local_8 = 0xffffffff;
                    } else {
                        let local_10 = mech_trace::cr!("10030c40", "100305b5", FUN_100305b5());
                        if !local_10.is_null() {
                            mech_trace::wr!(
                                "10030c40",
                                local_10.add(1),
                                write_unaligned,
                                param_1 as u32
                            );
                            mech_trace::wr!(
                                "10030c40",
                                local_10.add(2),
                                write_unaligned,
                                param_2 as u32
                            );
                            mech_trace::wr!("10030c40", local_10.add(0xb), write_unaligned, 1000);
                            local_8 = mech_trace::rd!("10030c40", local_10, read_unaligned);
                            let pi_var1 =
                                mech_trace::rd!("10030c40", local_10.add(0xc), read_unaligned)
                                    as *mut i32;
                            let i_var2 =
                                mech_trace::rd!("10030c40", pi_var1.add(0x1b), read_unaligned);
                            mech_trace::wr!(
                                "10030c40",
                                local_10.add(5),
                                write_unaligned,
                                rd(param_2.wrapping_add(0x50))
                            );
                            wr(
                                i_var2.wrapping_add(0x34),
                                mech_trace::rd!("10030c40", local_10.add(5), read_unaligned),
                            );
                            mech_trace::wr!(
                                "10030c40",
                                local_10.add(6),
                                write_unaligned,
                                rd(param_2.wrapping_add(0x74))
                            );
                            wr(
                                i_var2.wrapping_add(0x38),
                                mech_trace::rd!("10030c40", local_10.add(6), read_unaligned),
                            );
                            mech_trace::wr!(
                                "10030c40",
                                local_10.add(7),
                                write_unaligned,
                                rd(param_2.wrapping_add(0x58))
                            );
                            wr(
                                i_var2.wrapping_add(0x3c),
                                mech_trace::rd!("10030c40", local_10.add(7), read_unaligned),
                            );
                            wr(i_var2.wrapping_add(0x28), 0);
                            wr(i_var2.wrapping_add(0x2c), 0);
                            wr(i_var2.wrapping_add(0x30), 0);
                            FUN_1002f783(pi_var1 as i32);
                            FUN_1002f37a(
                                pi_var1 as i32,
                                mech_trace::rd!("10030c40", local_10.add(5), read_unaligned),
                                mech_trace::rd!("10030c40", local_10.add(6), read_unaligned),
                                mech_trace::rd!("10030c40", local_10.add(7), read_unaligned),
                            );
                            FUN_1002fbc0(pi_var1);
                        }
                    }
                }
                2 => {
                    let i_var2 = mech_trace::cr!("10030c40", "1007d0a4", FUN_1007d0a4(-1));
                    if i_var2 == 0 || param_2 == 0 {
                        local_8 = 0xffffffff;
                    } else {
                        let local_10 = mech_trace::cr!("10030c40", "100305b5", FUN_100305b5());
                        if !local_10.is_null() {
                            mech_trace::wr!(
                                "10030c40",
                                local_10.add(1),
                                write_unaligned,
                                param_1 as u32
                            );
                            mech_trace::wr!(
                                "10030c40",
                                local_10.add(2),
                                write_unaligned,
                                param_2 as u32
                            );
                            mech_trace::wr!("10030c40", local_10.add(0xb), write_unaligned, 2000);
                            local_8 = mech_trace::rd!("10030c40", local_10, read_unaligned);
                            FUN_1002f322(
                                rd(param_2.wrapping_add(0x30)) as i32,
                                &raw mut local_14,
                                &raw mut local_18,
                                &raw mut local_1c,
                            );
                            let pi_var1 =
                                mech_trace::rd!("10030c40", local_10.add(0xc), read_unaligned)
                                    as *mut i32;
                            let i_var2 =
                                mech_trace::rd!("10030c40", pi_var1.add(0x1b), read_unaligned);
                            mech_trace::wr!("10030c40", local_10.add(5), write_unaligned, local_14);
                            wr(
                                i_var2.wrapping_add(0x34),
                                mech_trace::rd!("10030c40", local_10.add(5), read_unaligned),
                            );
                            mech_trace::wr!("10030c40", local_10.add(6), write_unaligned, 0);
                            wr(
                                i_var2.wrapping_add(0x38),
                                mech_trace::rd!("10030c40", local_10.add(6), read_unaligned),
                            );
                            mech_trace::wr!("10030c40", local_10.add(7), write_unaligned, local_1c);
                            wr(
                                i_var2.wrapping_add(0x3c),
                                mech_trace::rd!("10030c40", local_10.add(7), read_unaligned),
                            );
                            wr(i_var2.wrapping_add(0x28), 0);
                            wr(i_var2.wrapping_add(0x2c), 0);
                            wr(i_var2.wrapping_add(0x30), 0);
                            FUN_1002f783(pi_var1 as i32);
                            FUN_1002f37a(
                                pi_var1 as i32,
                                mech_trace::rd!("10030c40", local_10.add(5), read_unaligned),
                                mech_trace::rd!("10030c40", local_10.add(6), read_unaligned),
                                mech_trace::rd!("10030c40", local_10.add(7), read_unaligned),
                            );
                            FUN_1002fbc0(pi_var1);
                        }
                    }
                }
                6 => {
                    let i_var2 = mech_trace::cr!("10030c40", "1007d0a4", FUN_1007d0a4(-1));
                    if i_var2 == 0 {
                        local_8 = 0xffffffff;
                    } else {
                        let local_10 = mech_trace::cr!("10030c40", "100305b5", FUN_100305b5());
                        if !local_10.is_null() {
                            mech_trace::wr!(
                                "10030c40",
                                local_10.add(1),
                                write_unaligned,
                                param_1 as u32
                            );
                            mech_trace::wr!("10030c40", local_10.add(2), write_unaligned, 0);
                            mech_trace::wr!("10030c40", local_10.add(0xb), write_unaligned, 1);
                            local_8 = mech_trace::rd!("10030c40", local_10, read_unaligned);
                            let pi_var1 =
                                mech_trace::rd!("10030c40", local_10.add(0xc), read_unaligned)
                                    as *mut i32;
                            let i_var2 =
                                mech_trace::rd!("10030c40", pi_var1.add(0x1b), read_unaligned);
                            mech_trace::wr!("10030c40", local_10.add(5), write_unaligned, 0);
                            wr(
                                i_var2.wrapping_add(0x34),
                                mech_trace::rd!("10030c40", local_10.add(5), read_unaligned),
                            );
                            mech_trace::wr!("10030c40", local_10.add(6), write_unaligned, 0);
                            wr(
                                i_var2.wrapping_add(0x38),
                                mech_trace::rd!("10030c40", local_10.add(6), read_unaligned),
                            );
                            mech_trace::wr!("10030c40", local_10.add(7), write_unaligned, 0);
                            wr(
                                i_var2.wrapping_add(0x3c),
                                mech_trace::rd!("10030c40", local_10.add(7), read_unaligned),
                            );
                            wr(i_var2.wrapping_add(0x28), 0);
                            wr(i_var2.wrapping_add(0x2c), 0);
                            wr(i_var2.wrapping_add(0x30), 0);
                            FUN_1002f783(pi_var1 as i32);
                            FUN_1002fbc0(pi_var1);
                        }
                    }
                }
                _ => {}
            }
        }
    } else {
        local_8 = 0xffffffff;
    }
    mech_trace::rt!("10030c40", local_8)
}
