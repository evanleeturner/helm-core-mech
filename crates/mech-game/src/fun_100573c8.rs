// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100573c8(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> u32 {
    mech_trace::enter!("100573c8", param_1, param_2, param_3, param_4);
    // SAFETY: all pointers come from the caller's own structures and the
    // game's bitmap buffers; reads and writes follow the original order.
    unsafe {
        let base =
            mech_trace::rd!("100573c8", param_1, read_unaligned) as u32 as usize as *const i32;
        let i_var1 = mech_trace::rd!("100573c8", base.add(1), read_unaligned);
        let i_var4 = i_var1.wrapping_add(1);
        if i_var4 != 0 && -2 < i_var1 {
            let p2 = param_2 as u32 as usize as *const u8;
            let i_var1 = mech_trace::rd!("100573c8", p2.add(0xc).cast::<i32>(), read_unaligned);
            let first = mech_trace::rd!("100573c8", base, read_unaligned) as u32;
            let a = first
                .wrapping_add(
                    mech_trace::rd!("100573c8", p2.add(8).cast::<i32>(), read_unaligned) as u32,
                )
                .wrapping_add(param_3 as u32)
                .wrapping_add(mech_trace::rd!("100573c8", param_1.add(1), read_unaligned) as u32);
            let b = (i_var1 as u32)
                .wrapping_add(param_4 as u32)
                .wrapping_add(mech_trace::rd!("100573c8", param_1.add(2), read_unaligned) as u32)
                .wrapping_mul(i_var4 as u32);
            let mut dst_row = a.wrapping_add(b) as usize as *mut u8;
            let i_var5 = mech_trace::rd!("100573c8", p2.add(0x14).cast::<i32>(), read_unaligned)
                .wrapping_add(1);
            let mut i_var6 = i_var5.wrapping_sub(i_var1);
            if i_var6 != 0 && i_var1 <= i_var5 {
                let mut src = p2.add(0x18);
                loop {
                    let h = mech_trace::dr!("100573c8", src);
                    src = src.wrapping_add(1);
                    let mut f = (h & 1) != 0;
                    let mut c = (h >> 1) as u32;
                    let mut dst = dst_row;
                    loop {
                        if c != 0 && !f {
                            // fill
                            let b = mech_trace::dr!("100573c8", src);
                            let hp = src.wrapping_add(1);
                            let mut n = c & 3;
                            while n != 0 {
                                mech_trace::dw!("100573c8", dst, b);
                                dst = dst.wrapping_add(1);
                                n -= 1;
                            }
                            let u2 = (b as u32).wrapping_mul(0x010101);
                            let val = u2.wrapping_shl(8) | (b as u32);
                            let mut d = (c & 0xff) >> 2;
                            while d != 0 {
                                mech_trace::wr!(
                                    "100573c8",
                                    dst.cast::<u32>(),
                                    write_unaligned,
                                    val
                                );
                                dst = dst.wrapping_add(4);
                                d -= 1;
                            }
                            src = src.wrapping_add(2);
                            let h = mech_trace::dr!("100573c8", hp);
                            f = (h & 1) != 0;
                            c = (h >> 1) as u32;
                        } else if c != 0 {
                            // copy
                            let mut n = c & 3;
                            while n != 0 {
                                mech_trace::dw!("100573c8", dst, mech_trace::dr!("100573c8", src));
                                src = src.wrapping_add(1);
                                dst = dst.wrapping_add(1);
                                n -= 1;
                            }
                            let mut p10 = src;
                            let mut d = (c & 0xff) >> 2;
                            while d != 0 {
                                let v =
                                    mech_trace::rd!("100573c8", p10.cast::<u32>(), read_unaligned);
                                mech_trace::wr!("100573c8", dst.cast::<u32>(), write_unaligned, v);
                                p10 = p10.wrapping_add(4);
                                dst = dst.wrapping_add(4);
                                d -= 1;
                            }
                            src = p10.wrapping_add(1);
                            let h = mech_trace::dr!("100573c8", p10);
                            f = (h & 1) != 0;
                            c = (h >> 1) as u32;
                        } else if f {
                            // skip
                            let hp = src.wrapping_add(1);
                            dst = dst.wrapping_add(mech_trace::dr!("100573c8", src) as usize);
                            src = src.wrapping_add(2);
                            let h = mech_trace::dr!("100573c8", hp);
                            f = (h & 1) != 0;
                            c = (h >> 1) as u32;
                        } else {
                            break;
                        }
                    }
                    dst_row = dst_row.wrapping_offset(i_var4 as isize);
                    i_var6 = i_var6.wrapping_sub(1);
                    if i_var6 == 0 {
                        break;
                    }
                }
            }
        }
        mech_trace::rt!("100573c8", 0)
    }
}
