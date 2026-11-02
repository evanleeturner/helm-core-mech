// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: i32;
    static mut DAT_100aefa0: i32;
    static mut DAT_100aefa4: i32;
    static mut DAT_10160fe0: [u8; 1];
    static mut DAT_100aaf68: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_1003aa95(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
    fn FUN_1003b049(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
}

unsafe fn jitter(v: i32) -> i32 {
    if v != 0 {
        // SAFETY: the callee is the game's own function.
        let r = unsafe { mech_trace::cr!("10039de0", "10051f43", FUN_10051f43(v)) };
        r.wrapping_sub(v >> 1)
    } else {
        v
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10039de0() {
    mech_trace::enter!("10039de0");
    // SAFETY: globals and callees are the game's own memory; the entry table
    // is read and written at the offsets the original uses.
    unsafe {
        if mech_trace::gr!("10039de0", "100b3904", DAT_100b3904) != 0
            && mech_trace::gr!("10039de0", "100aefa0", DAT_100aefa0) != 0
        {
            let base = (&raw mut DAT_10160fe0).cast::<u8>();
            let mut idx: i32 = 0;
            while idx < 9 {
                let off = idx.wrapping_mul(0x80c);
                let p = base.byte_add(off as usize);
                if mech_trace::rd!("10039de0", p.cast::<i32>(), read_unaligned) != 0 {
                    let pf4 = p.byte_add(0xf4).cast::<f32>();
                    let v = (mech_trace::gr!("10039de0", "100b3904", DAT_100b3904) as f32)
                        * mech_trace::rd!(
                            "10039de0",
                            p.byte_add(0xf8).cast::<f32>(),
                            read_unaligned
                        )
                        + mech_trace::rd!("10039de0", pf4, read_unaligned);
                    mech_trace::wr!("10039de0", pf4, write_unaligned, v);
                    let count = mech_trace::rd!("10039de0", pf4, read_unaligned) as i32;
                    if count != 0 {
                        mech_trace::wr!(
                            "10039de0",
                            pf4,
                            write_unaligned,
                            mech_trace::rd!("10039de0", pf4, read_unaligned) - (count as f32)
                        );
                        let mode = mech_trace::gr!("10039de0", "100aefa4", DAT_100aefa4);
                        let f: Option<unsafe extern "C" fn(i32, i32, i32, i32, i32, i32, i32)> =
                            if mode == 1 {
                                Some(FUN_1003aa95)
                            } else if mode == 2 {
                                Some(FUN_1003b049)
                            } else {
                                None
                            };
                        if let Some(f) = f {
                            let mut n: i32 = 0;
                            while n < count {
                                let l10 = jitter(mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0xe4).cast::<i32>(),
                                    read_unaligned
                                ));
                                let l14 = jitter(mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0xe8).cast::<i32>(),
                                    read_unaligned
                                ));
                                let l18 = jitter(mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0xec).cast::<i32>(),
                                    read_unaligned
                                ));
                                let a1 = mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0xc).cast::<i32>(),
                                    read_unaligned
                                )
                                .wrapping_add(mech_trace::rd!(
                                    "10039de0",
                                    mech_trace::gr!("10039de0", "100aaf68", DAT_100aaf68)
                                        .cast::<i32>(),
                                    read_unaligned
                                ))
                                .wrapping_add(l10);
                                let a2 = mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0x10).cast::<i32>(),
                                    read_unaligned
                                )
                                .wrapping_add(l14);
                                let a3 = mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0x14).cast::<i32>(),
                                    read_unaligned
                                )
                                .wrapping_add(mech_trace::rd!(
                                    "10039de0",
                                    mech_trace::gr!("10039de0", "100aaf68", DAT_100aaf68)
                                        .byte_add(8)
                                        .cast::<i32>(),
                                    read_unaligned
                                ))
                                .wrapping_add(l18);
                                let a4 = mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0xac).cast::<i32>(),
                                    read_unaligned
                                );
                                let a5 = mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0xb0).cast::<i32>(),
                                    read_unaligned
                                );
                                let a6 = mech_trace::rd!(
                                    "10039de0",
                                    p.byte_add(0xac).cast::<i32>(),
                                    read_unaligned
                                );
                                mech_trace::cr!(
                                    "10039de0",
                                    "ind",
                                    f(a1, a2, a3, a4, a5, a6, p as i32)
                                );
                                n = n.wrapping_add(1);
                            }
                        }
                    }
                }
                idx = idx.wrapping_add(1);
            }
        }
    }
}
