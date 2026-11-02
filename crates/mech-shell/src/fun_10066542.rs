// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa2c8: [u8; 256];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066542(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("10066542", param_1, param_2, param_3, param_4);
    // SAFETY: all pointers and the table are the game's own memory; reads and
    // writes occur in the same order and at the same offsets as the original.
    unsafe {
        let base = mech_trace::rd!("10066542", param_1, read_unaligned) as u32 as *const i32;
        let i_var2 = mech_trace::rd!("10066542", base.wrapping_add(1), read_unaligned);
        let i_var5 = i_var2.wrapping_add(1);
        if i_var5 != 0 && -2 < i_var2 {
            let p2 = param_2 as u32 as *const u8;
            let i_var2 = mech_trace::rd!(
                "10066542",
                p2.wrapping_add(0xc).cast::<i32>(),
                read_unaligned
            );
            let mut puvar12: u32 = (mech_trace::rd!("10066542", base, read_unaligned) as u32)
                .wrapping_add(mech_trace::rd!(
                    "10066542",
                    p2.wrapping_add(8).cast::<i32>(),
                    read_unaligned
                ) as u32)
                .wrapping_add(param_3 as u32)
                .wrapping_add(
                    mech_trace::rd!("10066542", param_1.wrapping_add(1), read_unaligned) as u32,
                )
                .wrapping_add(
                    (i_var2 as u32)
                        .wrapping_add(param_4 as u32)
                        .wrapping_add(mech_trace::rd!(
                            "10066542",
                            param_1.wrapping_add(2),
                            read_unaligned
                        ) as u32)
                        .wrapping_mul(i_var5 as u32),
                );
            let i_var6 = mech_trace::rd!(
                "10066542",
                p2.wrapping_add(0x14).cast::<i32>(),
                read_unaligned
            )
            .wrapping_add(1);
            let mut i_var7 = i_var6.wrapping_sub(i_var2);
            if i_var7 != 0 && i_var2 <= i_var6 {
                let mut p10: *const u8 = p2.wrapping_add(0x18);
                let tbl = (&raw const DAT_100aa2c8).cast::<u8>();
                loop {
                    let b = mech_trace::rd!("10066542", p10, read);
                    p10 = p10.wrapping_add(1);
                    let mut flag = (b & 1) != 0;
                    let mut cnt: u32 = (b >> 1) as u32;
                    let mut p13: *mut u8 = puvar12 as *mut u8;
                    loop {
                        if !flag && cnt != 0 {
                            // fill run
                            let u_var1 = mech_trace::rd!(
                                "10066542",
                                tbl.add(mech_trace::rd!("10066542", p10, read) as usize),
                                read
                            );
                            let h = mech_trace::rd!("10066542", p10.wrapping_add(1), read);
                            let mut n = cnt & 0xff;
                            while n != 0 {
                                mech_trace::wr!("10066542", p13, write, u_var1);
                                p13 = p13.wrapping_add(1);
                                n -= 1;
                            }
                            p10 = p10.wrapping_add(2);
                            flag = (h & 1) != 0;
                            cnt = (h >> 1) as u32;
                        } else if cnt != 0 {
                            // copy run
                            let mut n = cnt & 0xff;
                            while n != 0 {
                                mech_trace::wr!(
                                    "10066542",
                                    p13,
                                    write,
                                    mech_trace::rd!(
                                        "10066542",
                                        tbl.add(mech_trace::rd!("10066542", p10, read) as usize),
                                        read
                                    )
                                );
                                p10 = p10.wrapping_add(1);
                                p13 = p13.wrapping_add(1);
                                n -= 1;
                            }
                            let h = mech_trace::rd!("10066542", p10, read);
                            p10 = p10.wrapping_add(1);
                            flag = (h & 1) != 0;
                            cnt = (h >> 1) as u32;
                        } else if flag {
                            // skip
                            p13 = p13.wrapping_add(mech_trace::rd!("10066542", p10, read) as usize);
                            let h = mech_trace::rd!("10066542", p10.wrapping_add(1), read);
                            p10 = p10.wrapping_add(2);
                            flag = (h & 1) != 0;
                            cnt = (h >> 1) as u32;
                        } else {
                            break;
                        }
                    }
                    puvar12 = puvar12.wrapping_add(i_var5 as u32);
                    i_var7 = i_var7.wrapping_sub(1);
                    if i_var7 == 0 {
                        break;
                    }
                }
            }
        }
        mech_trace::rt!("10066542", 0)
    }
}
