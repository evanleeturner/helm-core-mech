// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(code: u32, flags: u32, nargs: u32, args: *const u32);
}

fn absu(x: u32) -> u32 {
    let s = ((x as i32) >> 31) as u32;
    (x ^ s).wrapping_sub(s)
}

fn fix(x: u32, m: u32, base: u32) -> i32 {
    (x ^ m).wrapping_sub(m).wrapping_add(base) as i32
}

fn mulc(a: u32, d: u32) -> u32 {
    let l = (a as u64).wrapping_mul(d as u64);
    ((l >> 32) as u32).wrapping_add((0x7fffffffu32 < (l as u32)) as u32)
}

fn dq(q: u32, r: u32, k: u32) -> u32 {
    q.wrapping_add(k).wrapping_sub((r == 0) as u32)
}

unsafe fn divc(hi: u32, d: u32) -> (u32, u32) {
    let mut q = 0u32;
    let mut r = 0u32;
    let v = ((hi as u64) << 32) | 0x80000000u64;
    if d == 0 || hi >= d {
        // SAFETY: raises the machine's divide-overflow exception as the original does.
        unsafe {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
    } else {
        q = (v / (d as u64)) as u32;
        r = (v % (d as u64)) as u32;
    }
    (q, r)
}

unsafe fn pix(mode: i32, t: i32, c: u8, p: u32) {
    // SAFETY: p is a pixel address inside the caller's surface; t is the caller's table.
    unsafe {
        if mode == 1 {
            let v = *(p as *const u8);
            let nv = *((t as u32).wrapping_add(v as u32) as *const u8);
            *(p as *mut u8) = nv;
        } else {
            *(p as *mut u8) = c;
        }
    }
}

unsafe fn call(f: i32, x: i32, y: i32) {
    // SAFETY: f is the caller's plot callback with this cdecl signature.
    unsafe {
        let func = core::mem::transmute::<usize, unsafe extern "C" fn(i32, i32)>(f as u32 as usize);
        func(x, y);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056489(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: i32,
    param_7: i32,
) -> u32 {
    // SAFETY: param_1 points at the caller's surface/clip descriptor; the pixel
    // pointers derived from it stay inside the surface as in the original.
    unsafe {
        let pi_var1 = param_1.read_unaligned() as u32 as *mut i32;
        let mut i_var29 = pi_var1.add(1).read_unaligned();
        let u_var5 = (i_var29 as u32).wrapping_add(1);
        if u_var5 == 0 || i_var29 < -1 {
            return 0xffffffff;
        }
        let i_var16 = pi_var1.add(2).read_unaligned();
        if i_var16 < 0 {
            return 0xffffffff;
        }
        let mut local_30 = param_1.add(1).read_unaligned();
        let mut i_var6 = local_30;
        if local_30 < 1 {
            i_var6 = 0;
        }
        let mut local_34 = param_1.add(2).read_unaligned();
        let mut i_var7 = local_34;
        if local_34 < 1 {
            i_var7 = 0;
        }
        let mut i_var8 = param_1.add(3).read_unaligned();
        if i_var29 <= param_1.add(3).read_unaligned() {
            i_var8 = i_var29;
        }
        i_var29 = param_1.add(4).read_unaligned();
        if i_var16 <= param_1.add(4).read_unaligned() {
            i_var29 = i_var16;
        }
        if i_var8 < i_var6 || i_var29 < i_var7 {
            return 0xfffffffe;
        }
        let mut i_var16 = param_2.wrapping_add(local_30);
        local_30 = param_4.wrapping_add(local_30);
        let mut i_var17 = param_3.wrapping_add(local_34);
        local_34 = param_5.wrapping_add(local_34);
        let u_var9 = (local_30 as u32).wrapping_sub(i_var16 as u32);
        let mut u_var19 = ((u_var9 as i32) >> 31) as u32;
        let mut u_var10 = absu(u_var9);
        let u_var11 = (local_34 as u32).wrapping_sub(i_var17 as u32);
        let u_var20 = ((u_var11 as i32) >> 31) as u32;
        let mut u_var12 = absu(u_var11);
        let b_var4 = param_7 as u8;
        let mut local_28 = i_var16;
        let mut local_2c: i32;
        let mut cnt: i32;

        if u_var9 == 0 {
            if i_var16 < i_var6 || i_var8 < i_var16 {
                return 2;
            }
            let mut t = i_var17;
            if i_var17 <= local_34 {
                t = local_34;
            }
            if t < i_var7 {
                return 2;
            }
            t = i_var17;
            if local_34 <= i_var17 {
                t = local_34;
            }
            if i_var29 < t {
                return 2;
            }
            if i_var17 <= i_var7 {
                i_var17 = i_var7;
            }
            local_2c = i_var17;
            if i_var29 <= i_var17 {
                local_2c = i_var29;
            }
            if local_34 <= i_var7 {
                local_34 = i_var7;
            }
            if i_var29 <= local_34 {
                local_34 = i_var29;
            }
            u_var12 = (u_var5 ^ u_var20).wrapping_sub(u_var20);
            cnt = absu((local_34 as u32).wrapping_sub(local_2c as u32)).wrapping_add(1) as i32;
        } else {
            local_2c = i_var17;
            if u_var11 == 0 {
                if i_var17 < i_var7 {
                    return 2;
                }
                if i_var29 < i_var17 {
                    return 2;
                }
                i_var29 = i_var16;
                if i_var16 <= local_30 {
                    i_var29 = local_30;
                }
                if i_var29 < i_var6 {
                    return 2;
                }
                i_var29 = i_var16;
                if local_30 <= i_var16 {
                    i_var29 = local_30;
                }
                if i_var8 < i_var29 {
                    return 2;
                }
                if i_var16 <= i_var6 {
                    i_var16 = i_var6;
                }
                if i_var8 <= i_var16 {
                    i_var16 = i_var8;
                }
                if local_30 <= i_var6 {
                    local_30 = i_var6;
                }
                if i_var8 <= local_30 {
                    local_30 = i_var8;
                }
                u_var12 = u_var19.wrapping_add(1) | u_var19;
                cnt = absu((local_30 as u32).wrapping_sub(i_var16 as u32)).wrapping_add(1) as i32;
                local_28 = i_var16;
            } else {
                let u_var13 = (((u_var9 ^ u_var11) as i32) >> 31) as u32;
                let mut u_var14: u32 = 0xffffffff;
                if u_var10 != u_var12 {
                    u_var14 = u_var10;
                    let mut u_var23 = u_var12;
                    if (u_var12 as i32) <= (u_var10 as i32) {
                        u_var14 = u_var12;
                        u_var23 = u_var10;
                    }
                    if u_var23 == 0 || u_var14 >= u_var23 {
                        RaiseException(0xC0000094, 0, 0, core::ptr::null());
                        u_var14 = 0;
                    } else {
                        u_var14 = (((u_var14 as u64) << 32) / (u_var23 as u64)) as u32;
                    }
                }
                let x0 = i_var16 as u32;
                let y0 = i_var17 as u32;
                let u6 = i_var6 as u32;
                let u7 = i_var7 as u32;
                let u8 = i_var8 as u32;
                let umy = i_var29 as u32;
                loop {
                    let a = (local_28.wrapping_sub(i_var6) < 0) as u8;
                    let b = (i_var8.wrapping_sub(local_28) < 0) as u8;
                    let c = (local_2c.wrapping_sub(i_var7) < 0) as u8;
                    let d = (i_var29.wrapping_sub(local_2c) < 0) as u8;
                    let b18 = (((a * 2 + b) * 2 + c) * 2) + d;
                    let a = (local_30.wrapping_sub(i_var6) < 0) as u8;
                    let b = (i_var8.wrapping_sub(local_30) < 0) as u8;
                    let c = (local_34.wrapping_sub(i_var7) < 0) as u8;
                    let d = (i_var29.wrapping_sub(local_34) < 0) as u8;
                    let b22 = (((a * 2 + b) * 2 + c) * 2) + d;
                    if b18 == 0 && b22 == 0 {
                        break;
                    }
                    if (b22 & b18) != 0 {
                        return 2;
                    }
                    if (u_var10 as i32) < (u_var12 as i32) {
                        if b18 & 8 != 0 {
                            let (q, r) = divc(u6.wrapping_sub(x0).wrapping_sub(1), u_var14);
                            local_2c = fix(dq(q, r, 1), u_var13, y0);
                            local_28 = i_var6;
                        } else if b18 & 4 != 0 {
                            let (q, r) =
                                divc(0xffffffffu32.wrapping_sub(u8.wrapping_sub(x0)), u_var14);
                            local_2c = fix(dq(q, r, 1), !u_var13, y0);
                            local_28 = i_var8;
                        } else if b18 & 2 != 0 {
                            local_2c = i_var7;
                            local_28 = fix(mulc(u7.wrapping_sub(y0), u_var14), u_var13, x0);
                        } else if b18 & 1 != 0 {
                            local_2c = i_var29;
                            local_28 = fix(
                                mulc(0u32.wrapping_sub(umy.wrapping_sub(y0)), u_var14),
                                !u_var13,
                                x0,
                            );
                        } else if b22 & 8 != 0 {
                            let (q, r) = divc(0u32.wrapping_sub(u6.wrapping_sub(x0)), u_var14);
                            local_34 = fix(dq(q, r, 0), !u_var13, y0);
                            local_30 = i_var6;
                        } else if b22 & 4 != 0 {
                            let (q, r) = divc(u8.wrapping_sub(x0), u_var14);
                            local_34 = fix(dq(q, r, 0), u_var13, y0);
                            local_30 = i_var8;
                        } else if b22 & 2 != 0 {
                            local_34 = i_var7;
                            local_30 = fix(
                                mulc(0u32.wrapping_sub(u7.wrapping_sub(y0)), u_var14),
                                !u_var13,
                                x0,
                            );
                        } else if b22 & 1 != 0 {
                            local_34 = i_var29;
                            local_30 = fix(mulc(umy.wrapping_sub(y0), u_var14), u_var13, x0);
                        }
                    } else if b18 & 8 != 0 {
                        local_2c = fix(mulc(u6.wrapping_sub(x0), u_var14), u_var13, y0);
                        local_28 = i_var6;
                    } else if b18 & 4 != 0 {
                        local_2c = fix(
                            mulc(0u32.wrapping_sub(u8.wrapping_sub(x0)), u_var14),
                            !u_var13,
                            y0,
                        );
                        local_28 = i_var8;
                    } else if b18 & 2 != 0 {
                        local_2c = i_var7;
                        let (q, r) = divc(u7.wrapping_sub(y0).wrapping_sub(1), u_var14);
                        local_28 = fix(dq(q, r, 1), u_var13, x0);
                    } else if b18 & 1 != 0 {
                        local_2c = i_var29;
                        let (q, r) =
                            divc(0xffffffffu32.wrapping_sub(umy.wrapping_sub(y0)), u_var14);
                        local_28 = fix(dq(q, r, 1), !u_var13, x0);
                    } else if b22 & 8 != 0 {
                        local_34 = fix(
                            mulc(0u32.wrapping_sub(u6.wrapping_sub(x0)), u_var14),
                            !u_var13,
                            y0,
                        );
                        local_30 = i_var6;
                    } else if b22 & 4 != 0 {
                        local_34 = fix(mulc(u8.wrapping_sub(x0), u_var14), u_var13, y0);
                        local_30 = i_var8;
                    } else if b22 & 2 != 0 {
                        local_34 = i_var7;
                        let (q, r) = divc(0u32.wrapping_sub(u7.wrapping_sub(y0)), u_var14);
                        local_30 = fix(dq(q, r, 0), !u_var13, x0);
                    } else if b22 & 1 != 0 {
                        local_34 = i_var29;
                        let (q, r) = divc(umy.wrapping_sub(y0), u_var14);
                        local_30 = fix(dq(q, r, 0), u_var13, x0);
                    }
                }
                let mut p = (local_2c as u32)
                    .wrapping_mul(u_var5)
                    .wrapping_add(pi_var1.read_unaligned() as u32)
                    .wrapping_add(local_28 as u32);
                let stride = (u_var5 ^ u_var20).wrapping_sub(u_var20);
                if u_var10 != u_var12 {
                    if (u_var10 as i32) < (u_var12 as i32) {
                        let mut n = absu((local_34 as u32).wrapping_sub(local_2c as u32))
                            .wrapping_add(1) as i32;
                        let mut acc = absu((local_2c as u32).wrapping_sub(y0))
                            .wrapping_mul(u_var14)
                            .wrapping_add(0x80000000);
                        if (u_var9 as i32) < 0 {
                            if param_6 < 2 {
                                loop {
                                    pix(param_6, param_7, b_var4, p);
                                    let ns = acc.wrapping_add(u_var14);
                                    let cy = (ns < acc) as u32;
                                    acc = ns;
                                    p = p.wrapping_add(stride.wrapping_sub(cy));
                                    n = n.wrapping_sub(1);
                                    if n == 0 {
                                        break;
                                    }
                                }
                            } else {
                                local_28 = local_28.wrapping_sub(param_1.add(1).read_unaligned());
                                local_2c = local_2c.wrapping_sub(param_1.add(2).read_unaligned());
                                loop {
                                    call(param_7, local_28, local_2c);
                                    let ns = acc.wrapping_add(u_var14);
                                    let cy = ns < acc;
                                    acc = ns;
                                    if cy {
                                        local_28 = local_28.wrapping_sub(1);
                                    }
                                    local_2c = local_2c.wrapping_add(
                                        u_var20.wrapping_mul(2).wrapping_add(1) as i32,
                                    );
                                    n = n.wrapping_sub(1);
                                    if n == 0 {
                                        break;
                                    }
                                }
                            }
                        } else if param_6 < 2 {
                            loop {
                                pix(param_6, param_7, b_var4, p);
                                let ns = acc.wrapping_add(u_var14);
                                let cy = (ns < acc) as u32;
                                acc = ns;
                                p = p.wrapping_add(cy.wrapping_add(stride));
                                n = n.wrapping_sub(1);
                                if n == 0 {
                                    break;
                                }
                            }
                        } else {
                            local_28 = local_28.wrapping_sub(param_1.add(1).read_unaligned());
                            local_2c = local_2c.wrapping_sub(param_1.add(2).read_unaligned());
                            loop {
                                call(param_7, local_28, local_2c);
                                let ns = acc.wrapping_add(u_var14);
                                let cy = ns < acc;
                                acc = ns;
                                if cy {
                                    local_28 = local_28.wrapping_add(1);
                                }
                                local_2c = local_2c
                                    .wrapping_add(u_var20.wrapping_mul(2).wrapping_add(1) as i32);
                                n = n.wrapping_sub(1);
                                if n == 0 {
                                    break;
                                }
                            }
                        }
                    } else {
                        let mut n = absu((local_30 as u32).wrapping_sub(local_28 as u32))
                            .wrapping_add(1) as i32;
                        let mut acc = absu((local_28 as u32).wrapping_sub(x0))
                            .wrapping_mul(u_var14)
                            .wrapping_add(0x80000000);
                        if (u_var9 as i32) < 0 {
                            if param_6 < 2 {
                                loop {
                                    pix(param_6, param_7, b_var4, p);
                                    p = p.wrapping_sub(1);
                                    let ns = acc.wrapping_add(u_var14);
                                    if ns < acc {
                                        p = p.wrapping_add(stride);
                                    }
                                    acc = ns;
                                    n = n.wrapping_sub(1);
                                    if n == 0 {
                                        break;
                                    }
                                }
                            } else {
                                local_28 = local_28.wrapping_sub(param_1.add(1).read_unaligned());
                                local_2c = local_2c.wrapping_sub(param_1.add(2).read_unaligned());
                                loop {
                                    call(param_7, local_28, local_2c);
                                    let ns = acc.wrapping_add(u_var14);
                                    let cy = ns < acc;
                                    acc = ns;
                                    if cy {
                                        local_2c = local_2c.wrapping_sub(1);
                                    }
                                    local_28 = local_28.wrapping_add(
                                        u_var20.wrapping_mul(2).wrapping_add(1) as i32,
                                    );
                                    n = n.wrapping_sub(1);
                                    if n == 0 {
                                        break;
                                    }
                                }
                            }
                        } else if param_6 < 2 {
                            loop {
                                pix(param_6, param_7, b_var4, p);
                                p = p.wrapping_add(1);
                                let ns = acc.wrapping_add(u_var14);
                                if ns < acc {
                                    p = p.wrapping_add(stride);
                                }
                                acc = ns;
                                n = n.wrapping_sub(1);
                                if n == 0 {
                                    break;
                                }
                            }
                        } else {
                            local_28 = local_28.wrapping_sub(param_1.add(1).read_unaligned());
                            local_2c = local_2c.wrapping_sub(param_1.add(2).read_unaligned());
                            loop {
                                call(param_7, local_28, local_2c);
                                let ns = acc.wrapping_add(u_var14);
                                let cy = ns < acc;
                                acc = ns;
                                if cy {
                                    local_2c = local_2c.wrapping_add(1);
                                }
                                local_28 = local_28
                                    .wrapping_add(u_var20.wrapping_mul(2).wrapping_add(1) as i32);
                                n = n.wrapping_sub(1);
                                if n == 0 {
                                    break;
                                }
                            }
                        }
                    }
                    return (local_34 != 0) as u32;
                }
                u_var12 = (u_var5 ^ u_var20)
                    .wrapping_sub(u_var20)
                    .wrapping_add(u_var19.wrapping_add(1) | u_var19);
                u_var10 = ((local_30.wrapping_sub(local_28)) >> 31) as u32;
                cnt = absu((local_30 as u32).wrapping_sub(local_28 as u32)).wrapping_add(1) as i32;
                let _ = u_var10;
            }
        }

        let mut p = (local_2c as u32)
            .wrapping_mul(u_var5)
            .wrapping_add(pi_var1.read_unaligned() as u32)
            .wrapping_add(local_28 as u32);
        if param_6 < 2 {
            loop {
                pix(param_6, param_7, b_var4, p);
                p = p.wrapping_add(u_var12);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        } else {
            local_28 = local_28.wrapping_sub(param_1.add(1).read_unaligned());
            local_2c = local_2c.wrapping_sub(param_1.add(2).read_unaligned());
            u_var19 = ((u_var9 != 0) as u32) | u_var19;
            loop {
                call(param_7, local_28, local_2c);
                local_2c = local_2c.wrapping_add((((u_var11 != 0) as u32) | u_var20) as i32);
                local_28 = local_28.wrapping_add(u_var19 as i32);
                cnt = cnt.wrapping_sub(1);
                if cnt == 0 {
                    break;
                }
            }
        }
        (local_34 != 0) as u32
    }
}
