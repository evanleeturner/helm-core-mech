// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: u32;
    static mut DAT_100ef030: u8;
    static mut DAT_101f5480: u32;
    static mut DAT_101f64a4: u8;
    static mut DAT_101f64a8: u8;
    static mut DAT_101f64ac: u8;
    static mut DAT_101f64ad: u8;
    static mut DAT_101f64b8: u8;
    static mut DAT_101f64c0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d403(param_1: i32) -> i32;
    fn FUN_1000dd06(param_1: i32) -> u8;
    fn FUN_1000d875(param_1: i32, param_2: i32) -> u32;
    fn FUN_1007c203(param_1: *mut i32, param_2: *mut u8, param_3: i32) -> u8;
    fn FUN_1000e3ec(param_1: u32) -> u32;
    fn FUN_1000e4a8(param_1: *mut i32) -> u8;
}

#[inline(always)]
fn at(base: *mut u8, off: i32) -> *mut u8 {
    base.wrapping_byte_offset(off as isize)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d659() {
    mech_trace::enter!("1000d659");
    // SAFETY: all globals and callees belong to the game; addresses are
    // computed from the game's table bases exactly as the original does.
    unsafe {
        let a4 = &raw mut DAT_101f64a4;
        let a8 = &raw mut DAT_101f64a8;
        let ac = &raw mut DAT_101f64ac;
        let ad = &raw mut DAT_101f64ad;
        let b8 = &raw mut DAT_101f64b8;
        let c0 = &raw mut DAT_101f64c0;
        let ef = &raw mut DAT_100ef030;

        let mut local_c: u32 = 0;
        while (local_c as i32) < (mech_trace::gr!("1000d659", "100a7584", DAT_100a7584) as i32) {
            let i_var1: i32 = (local_c as i32).wrapping_mul(0x7c);
            if -1 < mech_trace::rd!("1000d659", at(a8, i_var1).cast::<i32>(), read_unaligned) {
                let idx = mech_trace::rd!("1000d659", at(a8, i_var1).cast::<i32>(), read_unaligned);
                if (mech_trace::rd!("1000d659", at(ad, idx.wrapping_mul(0x7c)), read) & 2) != 0
                    && (mech_trace::rd!("1000d659", at(ad, i_var1), read) & 2) == 0
                {
                    mech_trace::gw!("1000d659", "101f5480", DAT_101f5480, 1);
                    mech_trace::cr!("1000d659", "1000d403", FUN_1000d403(local_c as i32));
                }
                let idx = mech_trace::rd!("1000d659", at(a8, i_var1).cast::<i32>(), read_unaligned);
                if (mech_trace::rd!("1000d659", at(ad, idx.wrapping_mul(0x7c)), read) & 8) == 0 {
                    if (mech_trace::rd!("1000d659", at(ad, i_var1), read) & 8) != 0
                        && (mech_trace::rd!("1000d659", at(ad, i_var1), read) & 2) == 0
                    {
                        mech_trace::gw!("1000d659", "101f5480", DAT_101f5480, 1);
                        let p = at(ac, i_var1).cast::<u32>();
                        mech_trace::wr!(
                            "1000d659",
                            p,
                            write_unaligned,
                            mech_trace::rd!("1000d659", p, read_unaligned) & 0xfffff7ff
                        );
                        let i_var2 = mech_trace::cr!(
                            "1000d659",
                            "1000d875",
                            FUN_1000d875(
                                local_c as i32,
                                mech_trace::rd!(
                                    "1000d659",
                                    at(a4, i_var1).cast::<i32>(),
                                    read_unaligned
                                ),
                            )
                        );
                        if i_var2 == 0 {
                            let p = at(ac, i_var1).cast::<u32>();
                            mech_trace::wr!(
                                "1000d659",
                                p,
                                write_unaligned,
                                mech_trace::rd!("1000d659", p, read_unaligned) & 0x800
                            );
                        } else {
                            let off = (local_c as i32).wrapping_mul(0x7c);
                            let v = mech_trace::rd!(
                                "1000d659",
                                at(b8, off).cast::<i32>(),
                                read_unaligned
                            );
                            if v != -1 {
                                let s = mech_trace::rd!(
                                    "1000d659",
                                    at(ef, v.wrapping_mul(0x40)).cast::<i16>(),
                                    read_unaligned
                                );
                                if ((s as i32 as u32) & 0x8000) != 0 {
                                    let ptr = mech_trace::rd!(
                                        "1000d659",
                                        at(c0, off).cast::<*mut i32>(),
                                        read_unaligned
                                    );
                                    mech_trace::cr!(
                                        "1000d659",
                                        "1007c203",
                                        FUN_1007c203(ptr, FUN_1000e4a8 as usize as *mut u8, 0)
                                    );
                                    mech_trace::cr!("1000d659", "1000e3ec", FUN_1000e3ec(local_c));
                                }
                            }
                        }
                    }
                } else if (mech_trace::rd!("1000d659", at(ad, i_var1), read) & 8) == 0 {
                    mech_trace::gw!("1000d659", "101f5480", DAT_101f5480, 1);
                    let p = at(ac, i_var1).cast::<u32>();
                    mech_trace::wr!(
                        "1000d659",
                        p,
                        write_unaligned,
                        mech_trace::rd!("1000d659", p, read_unaligned) | 0x800
                    );
                    mech_trace::cr!("1000d659", "1000dd06", FUN_1000dd06(local_c as i32));
                }
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
