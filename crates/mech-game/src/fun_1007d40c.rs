// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aae80: u32;
    static mut DAT_100aafa4: u32;
    static mut DAT_100aafb4: u32;
    static mut DAT_100b6814: u32;
    static mut DAT_100b6818: u32;
    static mut DAT_100b681c: *mut u32;
    static mut DAT_100b6824: u32;
    static mut DAT_100b6828: u32;
    static mut DAT_100c96b0: u32;
    static mut DAT_100c96bc: u32;
    static mut DAT_100c9ec0: u32;
    static mut DAT_100c9ec4: u32;
    static mut DAT_100c9ec8: u32;
    static mut DAT_10115160: i32;
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100807b5(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_1007d2bc(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007d40c(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    // SAFETY: all globals and callees are the game's own; the pointers derived
    // from the parameters point at the caller's structures, accessed as the
    // original does.
    unsafe {
        let u_var5 = DAT_100c9ec8;
        let u_var4 = DAT_100c9ec4;
        let u_var3 = DAT_100c9ec0;
        let u_var2 = DAT_100c96b0;
        if param_3 != 0 && param_1 != 0 && param_2 != 0 {
            let mut b_var7 = DAT_100aafa4 == 3;
            if b_var7 || DAT_100b6828 != 0 {
                let p = DAT_100aaf68 as usize as *const u8;
                DAT_100c9ec4 = p.cast::<u32>().read_unaligned();
                DAT_100c9ec8 = p.add(4).cast::<u32>().read_unaligned();
                DAT_100c9ec0 = p.add(8).cast::<u32>().read_unaligned();
                DAT_100c96b0 = 0;
            }
            let mut local_1c: i32 = 0;
            while local_1c < DAT_10115160 {
                let pv = (param_3.wrapping_add(local_1c.wrapping_mul(4)) as usize as *const u32)
                    .read_unaligned() as usize as *mut i32;
                let flags = (pv as *const u8).add(0x1d).read();
                if flags & 2 == 0 {
                    pv.add(0xb).write_unaligned(DAT_100c96bc as i32);
                    if DAT_100b6818 != 0 || b_var7 || DAT_100b6828 != 0 {
                        let r = FUN_100807b5(
                            pv.read_unaligned(),
                            pv.add(1).read_unaligned(),
                            pv.add(2).read_unaligned(),
                            pv.add(8).read_unaligned(),
                            pv.add(9).read_unaligned(),
                            pv.add(10).read_unaligned(),
                        );
                        let p = (r as u32).wrapping_mul(DAT_100b6814);
                        let t = p.wrapping_add((p >> 0x1f) & 0x7f);
                        let i_var6 = (t as i32) >> 7;
                        if 0 < i_var6 {
                            let v = pv.add(0xb).read_unaligned();
                            pv.add(0xb).write_unaligned(v.wrapping_add(i_var6));
                        }
                    }
                    if 0 < DAT_100aafb4 && !b_var7 {
                        let sh = (pv.add(2).read_unaligned() as u32).wrapping_shl(0xb);
                        let q = (sh / DAT_100aafb4) >> 4;
                        let v = pv.add(0xb).read_unaligned().wrapping_sub(q as i32);
                        pv.add(0xb).write_unaligned(v);
                        if v < 0 {
                            pv.add(0xb).write_unaligned(0);
                        }
                    }
                }
                local_1c = local_1c.wrapping_add(1);
            }
            DAT_100c96b0 = 1;
            if DAT_100b6824 != 0 && !b_var7 && DAT_100aae80 == 0 {
                let mut local_28 = DAT_100b681c;
                while !local_28.is_null() {
                    let r = FUN_1007d2bc(param_1, param_2, local_28 as usize as i32);
                    if r != 0 {
                        let idx = ((param_1.wrapping_add(
                            (param_1.wrapping_add(4) as usize as *const i32).read_unaligned(),
                        )) as usize as *const u8)
                            .read() as i32;
                        let i_var6 = idx.wrapping_mul(0x44).wrapping_add(param_2);
                        let b = i_var6 as usize as *const u8;
                        DAT_100c9ec4 = local_28
                            .add(5)
                            .read_unaligned()
                            .wrapping_sub(b.add(0xc).cast::<u32>().read_unaligned());
                        DAT_100c9ec8 = local_28
                            .add(6)
                            .read_unaligned()
                            .wrapping_sub(b.add(0x10).cast::<u32>().read_unaligned());
                        DAT_100c9ec0 = local_28
                            .add(7)
                            .read_unaligned()
                            .wrapping_sub(b.add(0x14).cast::<u32>().read_unaligned());
                        if DAT_100c9ec4 == 0 && DAT_100c9ec8 == 0 && DAT_100c9ec0 == 0 {
                            b_var7 = true;
                        } else {
                            let abs = |d: u32| -> i32 {
                                let s = ((d as i32) >> 0x1f) as u32;
                                (d ^ s).wrapping_sub(s) as i32
                            };
                            if abs(DAT_100c9ec4) < 0xffff
                                && abs(DAT_100c9ec8) < 0xffff
                                && abs(DAT_100c9ec0) < 0xffff
                            {
                                DAT_100c9ec4 = DAT_100c9ec4.wrapping_mul(0x10000);
                                DAT_100c9ec8 = DAT_100c9ec8.wrapping_mul(0x10000);
                                DAT_100c9ec0 = DAT_100c9ec0.wrapping_mul(0x10000);
                            }
                            b_var7 = false;
                        }
                        local_1c = 0;
                        while local_1c < DAT_10115160 {
                            let pv = (param_3.wrapping_add(local_1c.wrapping_mul(4)) as usize
                                as *const u32)
                                .read_unaligned() as usize
                                as *mut i32;
                            let flags = (pv as *const u8).add(0x1d).read();
                            if flags & 2 == 0 && pv.add(0xb).read_unaligned() < 0x80 {
                                let local_c = if b_var7 {
                                    0x80
                                } else {
                                    FUN_100807b5(
                                        pv.read_unaligned(),
                                        pv.add(1).read_unaligned(),
                                        pv.add(2).read_unaligned(),
                                        pv.add(8).read_unaligned(),
                                        pv.add(9).read_unaligned(),
                                        pv.add(10).read_unaligned(),
                                    )
                                };
                                if 0 < local_c {
                                    let v = pv.add(0xb).read_unaligned().wrapping_add(local_c);
                                    pv.add(0xb).write_unaligned(v);
                                    if 0x80 < v {
                                        pv.add(0xb).write_unaligned(0x80);
                                    }
                                }
                            }
                            local_1c = local_1c.wrapping_add(1);
                        }
                    }
                    local_28 = local_28.read_unaligned() as usize as *mut u32;
                }
            }
            DAT_100c96b0 = u_var2;
            DAT_100c9ec0 = u_var3;
            DAT_100c9ec4 = u_var4;
            DAT_100c9ec8 = u_var5;
            local_1c = 0;
            while local_1c < DAT_10115160 {
                let i_var6 = (param_3.wrapping_add(local_1c.wrapping_mul(4)) as usize as *const u32)
                    .read_unaligned() as usize as *mut u8;
                if i_var6.add(0x1d).read() & 2 == 0 {
                    let f = i_var6.add(0x2c).cast::<i32>();
                    f.write_unaligned(f.read_unaligned().wrapping_mul(param_4 >> 1));
                    f.write_unaligned(f.read_unaligned() / 0x440);
                    let v = f.read_unaligned();
                    if v < 1 {
                        f.write_unaligned(0);
                    } else if 0xf < v {
                        f.write_unaligned(0xf);
                    }
                    let fl = i_var6.add(0x1d);
                    fl.write(fl.read() | 2);
                }
                local_1c = local_1c.wrapping_add(1);
            }
        }
    }
}
