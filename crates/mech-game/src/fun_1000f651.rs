// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101ec750: u8;
    static mut DAT_101ec780: u8;
    static mut DAT_101ec78c: u8;
    static mut DAT_101ec794: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32);
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10044cc4(param_1: i32, param_2: *mut i32);
}

unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: the caller passes an address of readable game memory.
    unsafe { mech_trace::rd!("1000f651", (addr as *const i32), read_unaligned) }
}

unsafe fn wr(addr: i32, v: i32) {
    // SAFETY: the caller passes an address of writable game memory.
    unsafe { mech_trace::wr!("1000f651", (addr as *mut i32), write_unaligned, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000f651(param_1: i32, param_2: i32, param_3: i32) -> u32 {
    mech_trace::enter!("1000f651", param_1, param_2, param_3);
    let mut local_c: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_18: i32 = 0;
    // SAFETY: the globals and callees are the game's own; the params point
    // at the caller's structures, accessed where the original does.
    unsafe {
        if rd(param_3.wrapping_add(8)) == 1 {
            let i_var1 = mech_trace::cr!(
                "1000f651",
                "1004d5e3",
                FUN_1004d5e3(rd(param_1.wrapping_add(4)))
            );
            let b750 = (&raw mut DAT_101ec750) as i32;
            let b780 = (&raw mut DAT_101ec780) as i32;
            let b78c = (&raw mut DAT_101ec78c) as i32;
            let b794 = (&raw mut DAT_101ec794) as i32;
            let mut local_1c: i32 = 0;
            while local_1c < 0xff {
                let off = local_1c.wrapping_mul(0x60);
                let local_14 = b750.wrapping_add(off);
                if rd(b78c.wrapping_add(off)) != 0 && rd(b780.wrapping_add(off)) != 0 {
                    let t = rd(local_14);
                    if (t == 3 || t == 4 || t == 10 || t == 0xc || t == 9)
                        && mech_trace::cr!(
                            "1000f651",
                            "1004d5e3",
                            FUN_1004d5e3(rd(b794.wrapping_add(off)))
                        ) != i_var1
                    {
                        FUN_1002f322(
                            rd(local_14.wrapping_add(0x30)),
                            (&raw mut local_c).cast::<u32>(),
                            (&raw mut local_10).cast::<u32>(),
                            (&raw mut local_18).cast::<u32>(),
                        );
                        let i_var2 = rd(param_1.wrapping_add(0x50)).wrapping_sub(local_c);
                        let i_var3 = rd(param_1.wrapping_add(0x54)).wrapping_sub(local_10);
                        let local_8 = rd(param_1.wrapping_add(0x58)).wrapping_sub(local_18);
                        let i_var4 = mech_trace::cr!(
                            "1000f651",
                            "1000fc30",
                            FUN_1000fc30(i_var2, i_var3, local_8)
                        );
                        if i_var4 < 0xc351 {
                            let sum = rd(local_14.wrapping_add(0x14))
                                .wrapping_mul(local_8)
                                .wrapping_add(rd(local_14.wrapping_add(0x10)).wrapping_mul(i_var3))
                                .wrapping_add(rd(local_14.wrapping_add(0xc)).wrapping_mul(i_var2));
                            if 0 < sum {
                                wr(param_3.wrapping_add(8), 2);
                                wr(param_3.wrapping_add(0x24), rd(param_2.wrapping_add(0x2c)));
                                wr(param_3.wrapping_add(0x10), 0);
                                wr(param_3.wrapping_add(0x1c), 0);
                                wr(param_3.wrapping_add(0x18), rd(param_3.wrapping_add(0x1c)));
                                FUN_10044cc4(param_3, rd(param_1.wrapping_add(0x20)) as *mut i32);
                            }
                        }
                    }
                }
                local_1c = local_1c.wrapping_add(1);
            }
        }
    }
    mech_trace::rt!("1000f651", 1)
}
