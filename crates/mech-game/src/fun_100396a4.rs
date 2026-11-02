// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aeb5c: [u8; 0];
    static mut DAT_100aef5d: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003993b(param_1: u32);
    fn FUN_100398e3(param_1: u32);
}

unsafe fn decode(
    mut src: *const u8,
    tab: *mut u8,
    buf: *mut u8,
    acc: &mut i32,
    limit: u32,
    bits: u32,
    count: i32,
) -> *const u8 {
    let mask = (1u32 << bits) - 1;
    let mut pos: u32 = 0;
    // SAFETY: src, tab and buf point at the game's own buffers, accessed as the original does.
    unsafe {
        loop {
            let mut b = mech_trace::rd!("100396a4", src, read);
            src = src.add(1);
            let mut n = count;
            loop {
                let v = u32::from(b);
                b = (b as u32 >> bits) as u8;
                let t = mech_trace::rd!(
                    "100396a4",
                    tab.add(((v & mask) as usize) * 4).cast::<i32>(),
                    read_unaligned
                );
                let mut l = t.wrapping_add(mech_trace::dr!("100396a4", acc));
                if l < 0x80 {
                    if l < -0x7f {
                        l = -0x7f;
                    }
                } else {
                    l = 0x7f;
                }
                mech_trace::dw!("100396a4", acc, l);
                mech_trace::dw!(
                    "100396a4",
                    buf.add(pos as usize),
                    (l as u8).wrapping_sub(0x80)
                );
                pos = pos.wrapping_add(1);
                n -= 1;
                if n == 0 {
                    break;
                }
            }
            if pos >= limit {
                break;
            }
        }
    }
    src
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100396a4(
    param_1: *mut u8,
    param_2: *mut u8,
    param_3: i32,
    param_4: u32,
    param_5: *mut i32,
) -> *mut u8 {
    mech_trace::enter!("100396a4", param_1, param_2, param_3, param_4, param_5);
    let mut param_1 = param_1;
    let mut param_3 = param_3;
    let mut local_18 = param_2;
    // SAFETY: the globals are the game's own memory; only their addresses are taken.
    let buf: *mut u8 = unsafe { (&raw mut DAT_100aeb5c).cast::<u8>() };
    let tab: *mut u8 = unsafe { (&raw mut DAT_100aef5d).cast::<u8>() };
    // SAFETY: all pointers are the caller's/game's buffers, accessed in the original's order.
    unsafe {
        loop {
            let b1 = mech_trace::rd!("100396a4", param_1, read) >> 6;
            let mut u3 = param_4;
            if b1 == 1 {
                u3 = param_4 >> 1;
            } else if b1 == 2 {
                u3 = param_4 >> 2;
            }
            let b2 = mech_trace::rd!("100396a4", param_1, read) & 0xf;
            let mut p9: *const u8 = param_1.add(1) as *const u8;
            let mut local_8: i32 = mech_trace::rd!("100396a4", param_5, read);
            if b2 == 0 {
                let mut p = buf;
                while u3 != 0 {
                    mech_trace::dw!("100396a4", p, 0x80);
                    p = p.add(1);
                    u3 -= 1;
                }
                local_8 = 0;
            } else if b2 != 1 {
                if b2 == 2 {
                    let t = u32::from(mech_trace::rd!("100396a4", p9, read))
                        .wrapping_mul(2)
                        .wrapping_sub(0x80);
                    mech_trace::wr!("100396a4", tab.cast::<u32>(), write_unaligned, t);
                    p9 = param_1.add(3) as *const u8;
                    let t = u32::from(mech_trace::rd!("100396a4", param_1.add(2), read))
                        .wrapping_mul(2)
                        .wrapping_sub(0x80);
                    mech_trace::wr!("100396a4", tab.add(4).cast::<u32>(), write_unaligned, t);
                    p9 = decode(p9, tab, buf, &mut local_8, u3, 1, 8);
                } else if b2 == 3 {
                    let mut off = 0usize;
                    for _ in 0..4 {
                        let t = u32::from(mech_trace::rd!("100396a4", p9, read))
                            .wrapping_mul(2)
                            .wrapping_sub(0x80);
                        mech_trace::wr!("100396a4", tab.add(off).cast::<u32>(), write_unaligned, t);
                        p9 = p9.add(1);
                        off += 4;
                    }
                    p9 = decode(p9, tab, buf, &mut local_8, u3, 2, 4);
                } else if b2 == 4 {
                    let mut off = 0usize;
                    for _ in 0..0x10 {
                        let t = u32::from(mech_trace::rd!("100396a4", p9, read))
                            .wrapping_mul(2)
                            .wrapping_sub(0x80);
                        mech_trace::wr!("100396a4", tab.add(off).cast::<u32>(), write_unaligned, t);
                        p9 = p9.add(1);
                        off += 4;
                    }
                    p9 = decode(p9, tab, buf, &mut local_8, u3, 4, 2);
                } else {
                    if b2 != 5 {
                        return mech_trace::rt!("100396a4", core::ptr::null_mut());
                    }
                    let mut i4: usize = 0;
                    let mut i7: usize;
                    loop {
                        i7 = i4;
                        mech_trace::dw!(
                            "100396a4",
                            buf.add(i7),
                            mech_trace::rd!("100396a4", p9, read)
                        );
                        p9 = p9.add(1);
                        u3 = u3.wrapping_sub(1);
                        i4 = i7.wrapping_add(1);
                        if u3 == 0 {
                            break;
                        }
                    }
                    local_8 =
                        i32::from(mech_trace::dr!("100396a4", buf.add(i7))).wrapping_sub(0x80);
                }
            }
            if b1 == 1 {
                FUN_1003993b(param_4);
            } else if b1 == 2 {
                FUN_100398e3(param_4);
            }
            mech_trace::wr!("100396a4", param_5, write, local_8);
            let mut p = buf;
            let mut n = param_4;
            while n != 0 {
                mech_trace::dw!("100396a4", local_18, mech_trace::dr!("100396a4", p));
                p = p.add(1);
                local_18 = local_18.add(1);
                n -= 1;
            }
            param_3 = param_3.wrapping_sub(1);
            param_1 = p9 as *mut u8;
            if param_3 == 0 {
                return mech_trace::rt!("100396a4", p9 as *mut u8);
            }
        }
    }
}
