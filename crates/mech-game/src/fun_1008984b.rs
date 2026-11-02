// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008975d(param_1: *mut i32);
    fn FUN_10006b9a(param_1: i32) -> i32;
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_100891d0(param_1: *mut i32, param_2: i32);
    fn FUN_10089643(param_1: *mut i32, param_2: *mut i32);
}

fn fx(a: i32, b: i32) -> u32 {
    let p = (a as i64).wrapping_mul(b as i64);
    let lo = p as u32;
    let hi = ((p as u64).wrapping_shr(32)) as u32;
    (lo.wrapping_shr(29) | hi.wrapping_shl(3)).wrapping_add(((lo.wrapping_shr(28) & 1) != 0) as u32)
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008984b(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
    param_8: u32,
) {
    mech_trace::enter!(
        "1008984b", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8
    );
    let p = param_1;
    // SAFETY: param_1 points at the caller's matrix of at least 12 ints; the
    // callees are the game's own functions.
    unsafe {
        'done: {
            if param_2 == 0 {
                if param_3 == 0 {
                    FUN_1008975d(p);
                    if param_4 != 0 {
                        let v = mech_trace::cr!("1008984b", "10006b9a", FUN_10006b9a(param_4));
                        mech_trace::wr!("1008984b", p.add(4), write, v);
                        mech_trace::wr!(
                            "1008984b",
                            p,
                            write,
                            mech_trace::rd!("1008984b", p.add(4), read)
                        );
                        let v = mech_trace::cr!("1008984b", "10006b20", FUN_10006b20(param_4));
                        mech_trace::wr!("1008984b", p.add(3), write, v);
                        mech_trace::wr!(
                            "1008984b",
                            p.add(1),
                            write,
                            0u32.wrapping_sub(mech_trace::rd!("1008984b", p.add(3), read) as u32)
                                as i32
                        );
                    }
                    break 'done;
                }
                if param_4 == 0 {
                    FUN_1008975d(p);
                    let v = mech_trace::cr!("1008984b", "10006b9a", FUN_10006b9a(param_3));
                    mech_trace::wr!("1008984b", p.add(8), write, v);
                    mech_trace::wr!(
                        "1008984b",
                        p,
                        write,
                        mech_trace::rd!("1008984b", p.add(8), read)
                    );
                    let v = mech_trace::cr!("1008984b", "10006b20", FUN_10006b20(param_3));
                    mech_trace::wr!("1008984b", p.add(2), write, v);
                    mech_trace::wr!(
                        "1008984b",
                        p.add(6),
                        write,
                        0u32.wrapping_sub(mech_trace::rd!("1008984b", p.add(2), read) as u32)
                            as i32
                    );
                    break 'done;
                }
            } else if param_3 == 0 && param_4 == 0 {
                FUN_1008975d(p);
                let v = mech_trace::cr!("1008984b", "10006b9a", FUN_10006b9a(param_2));
                mech_trace::wr!("1008984b", p.add(8), write, v);
                mech_trace::wr!(
                    "1008984b",
                    p.add(4),
                    write,
                    mech_trace::rd!("1008984b", p.add(8), read)
                );
                let v = mech_trace::cr!("1008984b", "10006b20", FUN_10006b20(param_2));
                mech_trace::wr!("1008984b", p.add(7), write, v);
                mech_trace::wr!(
                    "1008984b",
                    p.add(5),
                    write,
                    0u32.wrapping_sub(mech_trace::rd!("1008984b", p.add(7), read) as u32) as i32
                );
                break 'done;
            }
            let i2 = mech_trace::cr!("1008984b", "10006b9a", FUN_10006b9a(param_2));
            let i3 = mech_trace::cr!("1008984b", "10006b9a", FUN_10006b9a(param_3));
            let i4 = mech_trace::cr!("1008984b", "10006b9a", FUN_10006b9a(param_4));
            let mut lc = mech_trace::cr!("1008984b", "10006b20", FUN_10006b20(param_2));
            let mut l10 = mech_trace::cr!("1008984b", "10006b20", FUN_10006b20(param_3));
            let mut l14 = mech_trace::cr!("1008984b", "10006b20", FUN_10006b20(param_4));
            if (param_8 & 4) != 0 {
                lc = 0u32.wrapping_sub(lc as u32) as i32;
                l10 = 0u32.wrapping_sub(l10 as u32) as i32;
                l14 = 0u32.wrapping_sub(l14 as u32) as i32;
            }
            let u5 = param_8 & 3;
            if u5 == 0 {
                mech_trace::wr!(
                    "1008984b",
                    p,
                    write,
                    fx(i4, i3).wrapping_add(fx(fx(l14, lc) as i32, l10)) as i32
                );
                mech_trace::wr!("1008984b", p.add(3), write, fx(i2, l14) as i32);
                mech_trace::wr!(
                    "1008984b",
                    p.add(6),
                    write,
                    fx(fx(i3, l14) as i32, lc).wrapping_sub(fx(i4, l10)) as i32
                );
                mech_trace::wr!("1008984b", p.add(2), write, fx(i2, l10) as i32);
                mech_trace::wr!(
                    "1008984b",
                    p.add(5),
                    write,
                    0u32.wrapping_sub(lc as u32) as i32
                );
                mech_trace::wr!("1008984b", p.add(8), write, fx(i2, i3) as i32);
                FUN_100891d0(p, 1);
            } else if u5 == 1 {
                mech_trace::wr!("1008984b", p, write, fx(i4, i3) as i32);
                mech_trace::wr!(
                    "1008984b",
                    p.add(3),
                    write,
                    fx(i2, l14).wrapping_add(fx(fx(i4, lc) as i32, l10)) as i32
                );
                mech_trace::wr!(
                    "1008984b",
                    p.add(6),
                    write,
                    fx(l14, lc).wrapping_sub(fx(fx(i4, i2) as i32, l10)) as i32
                );
                mech_trace::wr!("1008984b", p.add(2), write, l10);
                mech_trace::wr!(
                    "1008984b",
                    p.add(5),
                    write,
                    0u32.wrapping_sub(fx(i3, lc)) as i32
                );
                mech_trace::wr!("1008984b", p.add(8), write, fx(i2, i3) as i32);
                FUN_100891d0(p, 1);
            } else if u5 == 2 {
                mech_trace::wr!("1008984b", p, write, fx(i4, i3) as i32);
                mech_trace::wr!(
                    "1008984b",
                    p.add(3),
                    write,
                    fx(lc, l10).wrapping_add(fx(fx(i2, i3) as i32, l14)) as i32
                );
                mech_trace::wr!(
                    "1008984b",
                    p.add(6),
                    write,
                    fx(fx(i3, l14) as i32, lc).wrapping_sub(fx(i2, l10)) as i32
                );
                mech_trace::wr!(
                    "1008984b",
                    p.add(1),
                    write,
                    0u32.wrapping_sub(l14 as u32) as i32
                );
                mech_trace::wr!("1008984b", p.add(4), write, fx(i4, i2) as i32);
                mech_trace::wr!("1008984b", p.add(7), write, fx(i4, lc) as i32);
                FUN_100891d0(p, 2);
            }
            if (param_8 & 4) != 0 {
                FUN_10089643(p, p);
            }
        }
        mech_trace::wr!("1008984b", p.add(9), write, param_5);
        mech_trace::wr!("1008984b", p.add(10), write, param_6);
        mech_trace::wr!("1008984b", p.add(11), write, param_7);
    }
}
