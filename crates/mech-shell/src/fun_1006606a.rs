// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa2c8: [u8; 256];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10066542(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006606a(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) -> i32 {
    // SAFETY: the surface, the sprite data, the clip rectangle and the remap
    // table are the game's own memory; every access mirrors the original's
    // reads and writes.
    unsafe {
        let rd = |a: i32| -> i32 { (a as u32 as usize as *const i32).read_unaligned() };

        let pi2 = param_1.read_unaligned() as u32 as usize as *const i32;
        let mut iv11 = pi2.add(1).read_unaligned();
        let iv7 = iv11.wrapping_add(1);
        if iv7 == 0 || iv11 < -1 {
            return -1;
        }
        let mut iv12 = pi2.add(2).read_unaligned();
        if iv12 < 0 {
            return -1;
        }
        let mut iv3 = param_1.add(1).read_unaligned();
        let iv8 = if iv3 < 1 { 0 } else { iv3 };
        let mut iv4 = param_1.add(2).read_unaligned();
        let iv9 = if iv4 < 1 { 0 } else { iv4 };
        let mut iv10 = param_1.add(3).read_unaligned();
        if iv11 <= param_1.add(3).read_unaligned() {
            iv10 = iv11;
        }
        iv11 = param_1.add(4).read_unaligned();
        if iv12 <= param_1.add(4).read_unaligned() {
            iv11 = iv12;
        }
        if iv10 < iv8 || iv11 < iv9 {
            return -2;
        }
        iv3 = param_4.wrapping_add(iv3);
        iv4 = param_5.wrapping_add(iv4);
        let iv21 = rd(param_3
            .wrapping_mul(8)
            .wrapping_add(8)
            .wrapping_add(param_2))
        .wrapping_add(param_2);
        iv12 = rd(iv21.wrapping_add(8)).wrapping_add(iv3);
        let mut l24 = rd(iv21.wrapping_add(0xc)).wrapping_add(iv4);
        let iv13 = rd(iv21.wrapping_add(0x10)).wrapping_add(iv3);
        let iv14 = rd(iv21.wrapping_add(0x14)).wrapping_add(iv4);
        let mut p = (iv21.wrapping_add(0x18)) as u32 as usize as *const u8;
        if iv13 < iv12 || iv14 < l24 {
            return -4;
        }
        let b18: u8 = (((((iv12.wrapping_sub(iv8) < 0) as i32 * 2
            + (iv10.wrapping_sub(iv12) < 0) as i32)
            * 2
            + (l24.wrapping_sub(iv9) < 0) as i32)
            * 2)
            + (iv11.wrapping_sub(l24) < 0) as i32) as u8;
        let b20: u8 = (((((iv13.wrapping_sub(iv8) < 0) as i32 * 2
            + (iv10.wrapping_sub(iv13) < 0) as i32)
            * 2
            + (iv14.wrapping_sub(iv9) < 0) as i32)
            * 2)
            + (iv11.wrapping_sub(iv14) < 0) as i32) as u8;
        if (b20 & b18) != 0 {
            return -3;
        }
        if b18 == 0 && b20 == 0 {
            FUN_10066542(
                param_1,
                iv21,
                iv3.wrapping_sub(param_1.add(1).read_unaligned()),
                iv4.wrapping_sub(param_1.add(2).read_unaligned()),
            );
            return 0;
        }
        let mut l28: i32 = l24
            .wrapping_mul(iv7)
            .wrapping_add(pi2.read_unaligned())
            .wrapping_add(iv12);

        while l24 < iv9 {
            loop {
                let c = p.read();
                p = p.add(1);
                let bit = c & 1;
                let cnt = (c >> 1) as usize;
                if bit != 0 || cnt == 0 {
                    if cnt != 0 {
                        p = p.add(cnt - 1);
                    } else if bit == 0 {
                        break;
                    }
                }
                p = p.add(1);
            }
            l28 = l28.wrapping_add(iv7);
            l24 = l24.wrapping_add(1);
        }

        let mut l2c = l28.wrapping_add(iv8.wrapping_sub(iv12));
        let mut l30 = l28.wrapping_add(iv10.wrapping_sub(iv12));
        let left_flag = (b18 & 8) != 0;
        let r_flag = (b20 & 4) != 0;

        loop {
            if iv14 < l24 {
                return 0;
            }
            if iv11 < l24 {
                return 0;
            }
            let mut d = l28;
            let mut left_active = left_flag;
            let mut skipping = false;
            loop {
                let c = p.read();
                p = p.add(1);
                let bit = c & 1;
                let mut cnt = (c >> 1) as i32;
                if cnt == 0 {
                    if bit == 0 {
                        break;
                    }
                    if !skipping {
                        d = d.wrapping_add(p.read() as i32);
                    }
                    p = p.add(1);
                    continue;
                }
                if skipping {
                    p = p.add(if bit != 0 { cnt as usize } else { 1 });
                    continue;
                }
                if bit == 0 {
                    // run
                    if left_active {
                        let t = l2c.wrapping_sub(d);
                        if t < cnt {
                            if -1 < t {
                                d = d.wrapping_add(t);
                                cnt -= t;
                            }
                            left_active = false;
                        } else {
                            d = d.wrapping_add(cnt);
                            p = p.add(1);
                            continue;
                        }
                    }
                    let mut excess = 0i32;
                    if r_flag {
                        if l30 < d {
                            skipping = true;
                            p = p.add(1);
                            continue;
                        }
                        let x = d.wrapping_add(cnt - 1).wrapping_sub(l30);
                        excess = if x < 0 { 0 } else { x };
                    }
                    let n = cnt - excess;
                    let v = DAT_100aa2c8[p.read() as usize];
                    let mut i = 0;
                    while i < n {
                        (d.wrapping_add(i) as u32 as usize as *mut u8).write(v);
                        i += 1;
                    }
                    d = d.wrapping_add(n).wrapping_add(excess);
                    p = p.add(1);
                } else {
                    // literal
                    if left_active {
                        let t = l2c.wrapping_sub(d);
                        if t < cnt {
                            if -1 < t {
                                d = d.wrapping_add(t);
                                cnt -= t;
                                p = p.add(t as usize);
                            }
                            left_active = false;
                        } else {
                            d = d.wrapping_add(cnt);
                            p = p.add(cnt as usize);
                            continue;
                        }
                    }
                    let mut excess = 0i32;
                    if r_flag {
                        if l30 < d {
                            skipping = true;
                            p = p.add(cnt as usize);
                            continue;
                        }
                        let x = d.wrapping_add(cnt - 1).wrapping_sub(l30);
                        excess = if x < 0 { 0 } else { x };
                    }
                    let n = cnt - excess;
                    let mut i = 0;
                    while i < n {
                        let v = DAT_100aa2c8[p.read() as usize];
                        (d.wrapping_add(i) as u32 as usize as *mut u8).write(v);
                        p = p.add(1);
                        i += 1;
                    }
                    d = d.wrapping_add(n).wrapping_add(excess);
                    p = p.add(excess as usize);
                }
            }
            l28 = l28.wrapping_add(iv7);
            l2c = l2c.wrapping_add(iv7);
            l30 = l30.wrapping_add(iv7);
            l24 = l24.wrapping_add(1);
        }
    }
}
