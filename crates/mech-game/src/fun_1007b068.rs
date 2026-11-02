// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4ee0: i32;
    static mut DAT_100b4ee8: i32;
    static mut DAT_100b4eec: *mut i32;
    static mut DAT_100b4ef0: *mut i32;
    static mut DAT_100b4f70: i32;
    static mut DAT_100b4fd8: i32;
    static mut DAT_100b4fdc: i32;
    static mut DAT_100b4fe0: i32;
    static mut DAT_100b4fe4: [i32; 0];
    static mut DAT_100b67e4: u32;
    static mut DAT_100b67e8: u32;
    static mut DAT_100b67ec: u32;
    static mut DAT_10074068: [Option<unsafe extern "C" fn()>; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

#[inline(always)]
fn interp(a: i32, b: i32, t: i32) -> i32 {
    let d = b.wrapping_sub(a) as i64;
    let v = d.wrapping_mul(t as i64).wrapping_add(0x2000_0000i64) as u64;
    (a as u32).wrapping_add((v >> 30) as u32) as i32
}

unsafe fn clip_pass(
    mut cur: *const i32,
    mut prev: *const i32,
    mut out: *mut i32,
    mut count: i32,
    axis: usize,
    bound: i32,
    greater: bool,
) -> i32 {
    let other = 1 - axis;
    let mut produced: i32 = 0;
    loop {
        let c = cur;
        // SAFETY: cur/prev point at vertices of 6 ints in the caller's buffers;
        // out points into the other buffer, which does not overlap.
        unsafe {
            let pv = mech_trace::dr!("1007b068", prev.add(axis));
            let cv = mech_trace::dr!("1007b068", c.add(axis));
            let pv_out = if greater { bound < pv } else { pv < bound };
            let cv_out = if greater { bound < cv } else { cv < bound };
            let mut do_clip = false;
            if pv_out {
                if !cv_out {
                    do_clip = true;
                }
            } else {
                mech_trace::dw!(
                    "1007b068",
                    out.add(0),
                    mech_trace::dr!("1007b068", prev.add(0))
                );
                mech_trace::dw!(
                    "1007b068",
                    out.add(1),
                    mech_trace::dr!("1007b068", prev.add(1))
                );
                mech_trace::dw!(
                    "1007b068",
                    out.add(2),
                    mech_trace::dr!("1007b068", prev.add(2))
                );
                mech_trace::dw!(
                    "1007b068",
                    out.add(3),
                    mech_trace::dr!("1007b068", prev.add(3))
                );
                mech_trace::dw!(
                    "1007b068",
                    out.add(4),
                    mech_trace::dr!("1007b068", prev.add(4))
                );
                mech_trace::dw!(
                    "1007b068",
                    out.add(5),
                    mech_trace::dr!("1007b068", prev.add(5))
                );
                out = out.add(6);
                produced = produced.wrapping_add(1);
                if cv_out {
                    do_clip = true;
                }
            }
            if do_clip {
                let mut u5 = (pv as u32).wrapping_sub(cv as u32);
                let mut n = (bound as u32).wrapping_sub(cv as u32) as i32;
                let mut u8v: u32 = (n < 0) as u32;
                if n < 0 {
                    n = 0u32.wrapping_sub(n as u32) as i32;
                }
                if (u5 as i32) < 0 {
                    u8v = u8v.wrapping_sub(1);
                    u5 = 0u32.wrapping_sub(u5);
                }
                let mut q: u32;
                if u5 == 0 || ((n >> 2) as u32) >= u5 {
                    RaiseException(0xC000_0094, 0, 0, core::ptr::null());
                    q = 0;
                } else {
                    let nn: u64 =
                        (((n >> 2) as u32 as u64) << 32) | ((n as u32).wrapping_shl(30) as u64);
                    q = (nn / (u5 as u64)) as u32;
                    let r = (nn % (u5 as u64)) as u32;
                    let thr = ((u5 >> 1).wrapping_add(((u5 & 1) != 0) as u32)).wrapping_sub(1);
                    q = q.wrapping_add((thr < r) as u32);
                }
                if u8v != 0 {
                    q = 0u32.wrapping_sub(q);
                }
                let t = q as i32;
                mech_trace::dw!("1007b068", out.add(axis), bound);
                mech_trace::dw!(
                    "1007b068",
                    out.add(other),
                    interp(
                        mech_trace::dr!("1007b068", c.add(other)),
                        mech_trace::dr!("1007b068", prev.add(other)),
                        t
                    )
                );
                if mech_trace::gr!("1007b068", "100b67e4", DAT_100b67e4) != 0 {
                    mech_trace::dw!(
                        "1007b068",
                        out.add(3),
                        interp(
                            mech_trace::dr!("1007b068", c.add(3)),
                            mech_trace::dr!("1007b068", prev.add(3)),
                            t
                        )
                    );
                    mech_trace::dw!(
                        "1007b068",
                        out.add(4),
                        interp(
                            mech_trace::dr!("1007b068", c.add(4)),
                            mech_trace::dr!("1007b068", prev.add(4)),
                            t
                        )
                    );
                }
                if mech_trace::gr!("1007b068", "100b67e8", DAT_100b67e8) != 0 {
                    mech_trace::dw!(
                        "1007b068",
                        out.add(5),
                        interp(
                            mech_trace::dr!("1007b068", c.add(5)),
                            mech_trace::dr!("1007b068", prev.add(5)),
                            t
                        )
                    );
                }
                if mech_trace::gr!("1007b068", "100b67ec", DAT_100b67ec) != 0 {
                    mech_trace::dw!(
                        "1007b068",
                        out.add(2),
                        interp(
                            mech_trace::dr!("1007b068", c.add(2)),
                            mech_trace::dr!("1007b068", prev.add(2)),
                            t
                        )
                    );
                }
                out = out.add(6);
                produced = produced.wrapping_add(1);
            }
            cur = c.add(6);
            prev = c;
        }
        count = count.wrapping_sub(1);
        if count == 0 {
            break;
        }
    }
    produced
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007b068(
    param_1: *mut i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
) {
    mech_trace::enter!(
        "1007b068", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8
    );
    let mut param_3 = param_3;
    // SAFETY: all globals and the callee are the game's own; param_1/param_2
    // point at the caller's structures, accessed as the original does.
    unsafe {
        let mut local_20: u32 = 0;
        let xmax = mech_trace::dr!("1007b068", param_1.add(3));
        let xmin = mech_trace::dr!("1007b068", param_1.add(1));
        let ymax = mech_trace::dr!("1007b068", param_1.add(4));
        let ymin = mech_trace::dr!("1007b068", param_1.add(2));
        let hdr = mech_trace::dr!("1007b068", param_1) as u32 as *const i32;
        mech_trace::gw!(
            "1007b068",
            "100b4ee0",
            DAT_100b4ee0,
            (mech_trace::rd!("1007b068", hdr.add(1), read) as u32).wrapping_add(1) as i32
        );
        mech_trace::gw!(
            "1007b068",
            "100b4ee8",
            DAT_100b4ee8,
            mech_trace::rd!("1007b068", hdr, read)
        );

        let scratch: *mut i32 = (&raw mut DAT_100b4fe4).cast::<i32>();

        let mut i7 = param_3;
        let mut p12 = param_2;
        loop {
            let x = mech_trace::dr!("1007b068", p12);
            let y = mech_trace::dr!("1007b068", p12.add(1));
            local_20 |= (((((xmax as u32).wrapping_sub(x as u32)) >> 31) << 1
                | (((x as u32).wrapping_sub(xmin as u32)) >> 31))
                << 1
                | (((ymax as u32).wrapping_sub(y as u32)) >> 31))
                << 1
                | (((y as u32).wrapping_sub(ymin as u32)) >> 31);
            p12 = p12.add(6);
            i7 = i7.wrapping_sub(1);
            if i7 == 0 {
                break;
            }
        }

        let mut p14: *mut i32 = param_2.offset((param_3.wrapping_sub(1) as isize) * 6);
        let mut p15: *mut i32 = scratch;
        p12 = param_2;

        if local_20 != 0 {
            let mut b4: u8 = 0;
            mech_trace::gw!("1007b068", "100b67e4", DAT_100b67e4, param_4 & 0x400);
            mech_trace::gw!(
                "1007b068",
                "100b67e8",
                DAT_100b67e8,
                ((param_4 & 0x600) == 0x600) as u32
            );
            mech_trace::gw!("1007b068", "100b67ec", DAT_100b67ec, 1);
            let u5 = param_4 & 0x1c0;
            if u5 != 0x40 && u5 != 0xc0 && u5 != 0x100 && (param_4 & 7) != 4 && (param_4 & 7) != 7 {
                mech_trace::gw!("1007b068", "100b67ec", DAT_100b67ec, 0);
            }

            // x > xmax
            if (local_20 & 8) != 0 {
                let c = clip_pass(p12, p14, p15, param_3, 0, xmax, true);
                b4 = 1;
                param_3 = c;
                if c == 0 {
                    return mech_trace::rt!(
                        "1007b068",
                        finish(p12, param_3, param_4, param_5, param_6, param_7, param_8)
                    );
                }
                p12 = scratch;
                p14 = scratch.offset((c.wrapping_sub(1) as isize) * 6);
                p15 = param_2;
            }

            // y < ymin
            if (local_20 & 1) != 0 {
                let c = clip_pass(p12, p14, p15, param_3, 1, ymin, false);
                param_3 = c;
                if c == 0 {
                    return mech_trace::rt!(
                        "1007b068",
                        finish(p12, param_3, param_4, param_5, param_6, param_7, param_8)
                    );
                }
                b4 ^= 1;
                if b4 != 0 {
                    p14 = scratch.offset((c.wrapping_sub(1) as isize) * 6);
                    p12 = scratch;
                    p15 = param_2;
                } else {
                    p14 = param_2.offset((c.wrapping_sub(1) as isize) * 6);
                    p15 = scratch;
                    p12 = param_2;
                }
            }

            // x < xmin
            if (local_20 & 4) != 0 {
                let c = clip_pass(p12, p14, p15, param_3, 0, xmin, false);
                param_3 = c;
                if c == 0 {
                    return mech_trace::rt!(
                        "1007b068",
                        finish(p12, param_3, param_4, param_5, param_6, param_7, param_8)
                    );
                }
                b4 = b4.wrapping_add(1);
                if (b4 & 1) != 0 {
                    p14 = scratch.offset((c.wrapping_sub(1) as isize) * 6);
                    p12 = scratch;
                    p15 = param_2;
                } else {
                    p14 = param_2.offset((c.wrapping_sub(1) as isize) * 6);
                    p15 = scratch;
                    p12 = param_2;
                }
            }

            // y > ymax
            if (local_20 & 2) != 0 {
                let c = clip_pass(p12, p14, p15, param_3, 1, ymax, true);
                param_3 = c;
                if c != 0 {
                    p12 = param_2;
                    if (b4.wrapping_add(1) & 1) != 0 {
                        p12 = scratch;
                    }
                }
            }
        }

        finish(p12, param_3, param_4, param_5, param_6, param_7, param_8);
    }
}

#[allow(non_snake_case)]
unsafe fn finish(
    p12: *mut i32,
    param_3: i32,
    param_4: u32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
) {
    if 2 < param_3 {
        // SAFETY: the globals are the game's own; the dispatch table is indexed
        // by param_4 exactly as the original does.
        unsafe {
            mech_trace::gw!(
                "1007b068",
                "100b4ef0",
                DAT_100b4ef0,
                p12.wrapping_offset((param_3 as isize) * 6)
            );
            mech_trace::gw!("1007b068", "100b4fd8", DAT_100b4fd8, param_6);
            mech_trace::gw!("1007b068", "100b4fdc", DAT_100b4fdc, param_7);
            mech_trace::gw!("1007b068", "100b4fe0", DAT_100b4fe0, param_8);
            mech_trace::gw!("1007b068", "100b4f70", DAT_100b4f70, param_5);
            mech_trace::gw!("1007b068", "100b4eec", DAT_100b4eec, p12);
            let f = mech_trace::rd!(
                "1007b068",
                (&raw mut DAT_10074068)
                    .cast::<Option<unsafe extern "C" fn()>>()
                    .add(param_4 as usize),
                read
            );
            if let Some(f) = f {
                mech_trace::cr!("1007b068", "ind", f());
            }
        }
    }
}
