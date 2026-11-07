// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf34: u32;
    static mut DAT_100adf38: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1004d5e3(param_1: i32) -> u32;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn fwrite(s: *const c_void, size: u32, count: u32, file: *mut c_void) -> u32;
}

#[inline(always)]
unsafe fn ld32(a: i32) -> i32 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { mech_trace::rd!("1006b810", (a as usize as *const i32), read_unaligned) }
}

#[inline(always)]
unsafe fn ld16(a: i32) -> u16 {
    // SAFETY: the address is the game's own memory, read as the original does.
    unsafe { mech_trace::rd!("1006b810", (a as usize as *const u16), read_unaligned) }
}

#[inline(always)]
unsafe fn st16(a: i32, v: u16) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { mech_trace::wr!("1006b810", (a as usize as *mut u16), write_unaligned, v) }
}

#[inline(always)]
unsafe fn st32(a: i32, v: i32) {
    // SAFETY: the address is the game's own memory, written as the original does.
    unsafe { mech_trace::wr!("1006b810", (a as usize as *mut i32), write_unaligned, v) }
}

#[inline(always)]
unsafe fn idiv(a: i32, b: i32) -> i32 {
    let q: i32;
    // SAFETY: the machine's own signed divide, faulting as the original does.
    unsafe {
        mech_trace::log::debug!(
            f = "1006b810", b = "idiv", a:% = mech_trace::Args([
                mech_trace::__shown!(b),
                mech_trace::__shown!(a),
            ]); "asm.in"
        );
        core::arch::asm!(
            "cdq",
            "idiv {b:e}",
            b = in(reg) b,
            inout("eax") a => q,
            lateout("edx") _,
        );
        mech_trace::log::debug!(
            f = "1006b810", b = "idiv", r = "eax", v:% = mech_trace::__shown!(q); "asm.out"
        );
    }
    q
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b810(param_1: *mut c_void, param_2: *mut i32) -> u32 {
    mech_trace::enter!("1006b810", param_1, param_2);
    // SAFETY: param_2 and everything it points at belong to the game; the
    // globals and callees are the game's own; local buffers are in bounds.
    unsafe {
        if param_2.is_null() || (ld16((param_2 as i32).wrapping_add(0x442)) & 2) == 0 {
            return mech_trace::rt!("1006b810", 0);
        }
        let pb = param_2 as i32;
        let g = |i: i32| -> i32 { ld32(pb.wrapping_add(i.wrapping_mul(4))) };

        let mut local_ec: i32 = 0;
        let mut local_c: i32 = 0;
        let mut local_dc: i32 = 0;
        let cvar1 = mech_trace::dr!(
            "1006b810",
            ((mech_trace::gr!("1006b810", "100adf34", DAT_100adf34).wrapping_add(5)) as usize
                as *const i8)
        );
        if cvar1 == 0 {
            local_ec = 1;
            local_c = 3;
            local_dc = 4;
        } else if cvar1 == 1 {
            local_ec = 3;
            local_c = 3;
            local_dc = 4;
        } else if cvar1 == 2 {
            local_ec = 4;
            local_c = 4;
            local_dc = 4;
        }
        let local_8: i32;
        let inner = ld32(ld32(pb).wrapping_add(4));
        if (inner as u32) == mech_trace::gr!("1006b810", "100adf38", DAT_100adf38) {
            local_8 = local_dc;
        } else {
            let iv = mech_trace::cr!("1006b810", "1004d5e3", FUN_1004d5e3(inner));
            if iv == 0 {
                local_8 = local_c;
            } else {
                local_8 = local_ec;
            }
        }

        let p16 = g(0x16);
        let p17 = g(0x17);
        let p15 = g(0x15);

        let mut e: i32 = 0;
        while e < g(0xff) {
            let iv3 = g(0x17).wrapping_add(e.wrapping_mul(0x1e));
            if (ld16(iv3.wrapping_add(2)) as i16) < 1 {
                let mut f0: i32 = 0;
                while f0 < 8 {
                    let mut f4: i32 = 0;
                    while f4 < 0xc {
                        let base = g(0x16)
                            .wrapping_add(f4.wrapping_mul(2))
                            .wrapping_add(0xc)
                            .wrapping_add(f0.wrapping_mul(0x28));
                        if (ld16(base) as u32) == ((ld16(iv3.wrapping_add(4)) as i16) as i32 as u32)
                        {
                            let mut l168 = f4;
                            while l168 < 0xb {
                                let d = g(0x16)
                                    .wrapping_add(l168.wrapping_mul(2))
                                    .wrapping_add(0xc)
                                    .wrapping_add(f0.wrapping_mul(0x28));
                                let s = l168
                                    .wrapping_mul(2)
                                    .wrapping_add(g(0x16))
                                    .wrapping_add(0xe)
                                    .wrapping_add(f0.wrapping_mul(0x28));
                                st16(d, ld16(s));
                                l168 += 1;
                            }
                            let d = g(0x16)
                                .wrapping_add(l168.wrapping_mul(2))
                                .wrapping_add(0xc)
                                .wrapping_add(f0.wrapping_mul(0x28));
                            st16(d, 0);
                        }
                        f4 += 1;
                    }
                    f0 += 1;
                }
            }
            e += 1;
        }

        let mut local_e4: i32 = if ld32(pb.wrapping_add(0x4ab)) == 0 {
            10
        } else {
            0x14
        };

        let mut out = [0i32; 80];
        let ob = out.as_mut_ptr() as usize as i32;

        e = 0;
        while e < 8 {
            let eb = ob.wrapping_add(e.wrapping_mul(40));
            let rb = g(0x16).wrapping_add(e.wrapping_mul(0x28));
            st16(
                eb.wrapping_add(38),
                (ld16(rb.wrapping_add(0x26)) | 0xf800) & 0xfff,
            );
            st16(eb.wrapping_add(36), ld16(rb.wrapping_add(0x24)));
            if ((ld32(rb.wrapping_add(8)) as u32 & 0xffff0000) as i32) < 1 {
                st32(eb.wrapping_add(8), 0);
                st32(eb, 0);
                st32(eb.wrapping_add(4), 0);
            } else {
                st32(eb.wrapping_add(8), ld32(rb.wrapping_add(8)) >> 0x10);
                st32(eb, idiv(ld32(rb) >> 0x10, local_8));
                st32(
                    eb.wrapping_add(4),
                    idiv(ld32(rb.wrapping_add(4)) >> 0x10, local_8),
                );
            }
            let mut f0: i32 = 0;
            while f0 < 0xc {
                let slot = eb.wrapping_add(12).wrapping_add(f0.wrapping_mul(2));
                if ld32(eb.wrapping_add(8)) < 1 {
                    st16(slot, 0);
                } else {
                    st16(
                        slot,
                        ld16(
                            g(0x16)
                                .wrapping_add(f0.wrapping_mul(2))
                                .wrapping_add(0xc)
                                .wrapping_add(e.wrapping_mul(0x28)),
                        ),
                    );
                    if (ld16(slot) as i16) == 8000 {
                        local_e4 = local_e4.wrapping_add(1);
                    }
                }
                f0 += 1;
            }
            e += 1;
        }

        let mut local_164 = [0u32; 28];
        let mut local_10: i32 = 0;
        let mut local_f0: i32 = 0;
        e = 0;
        loop {
            let b = g(0x15).wrapping_add(e.wrapping_mul(0x40));
            if -1 < ld32(b.wrapping_add(4)) {
                if ld32(b.wrapping_add(0x38)) == 0 {
                    local_164[(local_10 as usize).wrapping_mul(2)] =
                        ld32(b.wrapping_add(0x3c)) as u32;
                    local_164[(local_10 as usize).wrapping_mul(2).wrapping_add(1)] =
                        ((ld32(b.wrapping_add(0xc)) as u32 & 0xf) << 8)
                            | (ld32(b.wrapping_add(0x28)) as u32);
                    local_10 = local_10.wrapping_add(1);
                }
                local_f0 = local_f0.wrapping_add(1);
            }
            e += 1;
            if !(local_f0 < g(0xf7) && e < 0xe) {
                break;
            }
        }
        st32(pb.wrapping_add(0x4cb), local_10);

        let mut local_d8 = [0i32; 50];
        let mut local_e0: i32 = 0;
        e = 0;
        while e < g(0x18) && e < 0xe {
            let eo = pb.wrapping_add(e.wrapping_mul(0x3a));
            if ld32(eo.wrapping_add(0x66)) != 0 {
                let mut f0 = ld32(eo.wrapping_add(0x6e));
                while f0 < ld32(eo.wrapping_add(0x6a)) {
                    let iv3 = ld32(eo.wrapping_add(f0.wrapping_mul(4)).wrapping_add(0x72));
                    let rec = g(0x17).wrapping_add(iv3.wrapping_mul(0x1e));
                    local_d8[(local_e0 as usize).wrapping_mul(2)] =
                        (ld16(rec.wrapping_add(4)) as i16) as i32;
                    local_d8[(local_e0 as usize).wrapping_mul(2).wrapping_add(1)] =
                        (ld16(rec) as u32).wrapping_mul(100) as i32;
                    local_e0 = local_e0.wrapping_add(1);
                    f0 += 1;
                }
            }
            e += 1;
        }
        st32(pb.wrapping_add(0x4cf), local_e0);
        st32(pb.wrapping_add(0x4c7), local_e4);
        let uv = mech_trace::cr!(
            "1006b810",
            "10002570",
            FUN_10002570(g(0xee) as u32, 0x697e98)
        );
        st32(pb.wrapping_add(0x4bf), uv as i32);
        st32(pb.wrapping_add(0x4bb), g(0x106));
        st32(pb.wrapping_add(0x4c3), g(0xfe));
        let _ = (p15, p16, p17);

        mech_trace::cr!(
            "1006b810",
            "fwrite",
            fwrite(
                pb.wrapping_add(0x4bb) as usize as *const c_void,
                0x18,
                1,
                param_1,
            )
        );
        mech_trace::cr!(
            "1006b810",
            "fwrite",
            fwrite(out.as_ptr() as *const c_void, 0x140, 1, param_1)
        );
        if local_10 != 0 {
            mech_trace::cr!(
                "1006b810",
                "fwrite",
                fwrite(
                    local_164.as_ptr() as *const c_void,
                    (local_10 as u32).wrapping_shl(3),
                    1,
                    param_1,
                )
            );
        }
        if local_e0 != 0 {
            mech_trace::cr!(
                "1006b810",
                "fwrite",
                fwrite(
                    local_d8.as_ptr() as *const c_void,
                    (local_e0 as u32).wrapping_shl(3),
                    1,
                    param_1,
                )
            );
        }
        mech_trace::cr!(
            "1006b810",
            "fwrite",
            fwrite(
                pb.wrapping_add(0x489) as usize as *const c_void,
                0x32,
                1,
                param_1,
            )
        );
        mech_trace::rt!("1006b810", 1)
    }
}
