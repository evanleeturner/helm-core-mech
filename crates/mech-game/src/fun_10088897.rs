// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100adf98: u8;
    static mut DAT_100ba9f8: i32;
    static mut DAT_100bf7d0: i32;
    static mut DAT_100baa00: i32;
    static mut DAT_100baa04: i32;
    static mut DAT_100baa08: i32;
    static mut DAT_100ad6e0: i32;
    static mut DAT_100ba96c: i16;

    static mut DAT_100baa10: u8;
    static mut DAT_100baa14: u8;
    static mut DAT_100baa18: u8;
    static mut DAT_100baa1c: u8;
    static mut DAT_100baa20: u8;
    static mut DAT_100baa24: u8;
    static mut DAT_100baa28: u8;
    static mut DAT_100baa2c: u8;
    static mut DAT_100baa8c: u8;
    static mut DAT_100c8908: u8;
    static mut DAT_100c8dc0: u8;
    static mut DAT_100c8990: u8;
    static mut DAT_100c8b68: u8;
    static mut DAT_100c8ba8: u8;
    static mut DAT_100bf7f0: u8;
    static mut PTR_DAT_100bf7ec: u8;
    static mut DAT_100bccd8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
    fn FUN_100879ee(param_1: i32, param_2: *mut c_void) -> u32;
    fn FUN_100885c8(param_1: *mut i32) -> i32;
    fn FUN_10081fdc();
}

unsafe fn ri(p: *const u8, off: i32) -> i32 {
    // SAFETY: the caller passes a game-owned address and offset the original reads.
    unsafe { p.offset(off as isize).cast::<i32>().read_unaligned() }
}

unsafe fn wi(p: *mut u8, off: i32, v: i32) {
    // SAFETY: the caller passes a game-owned address and offset the original writes.
    unsafe { p.offset(off as isize).cast::<i32>().write_unaligned(v) }
}

unsafe fn rp(p: *const u8, off: i32) -> *mut u8 {
    // SAFETY: the caller passes a game-owned address and offset the original reads.
    unsafe { p.offset(off as isize).cast::<*mut u8>().read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10088897() {
    // SAFETY: all globals, tables and function pointers are the game's own
    // memory, accessed at the same offsets and in the same order as the original.
    unsafe {
        let mut local_8: [i16; 2] = [0; 2];
        let a10 = &raw mut DAT_100baa10 as *mut u8;
        let a14 = &raw mut DAT_100baa14 as *mut u8;
        let a18 = &raw mut DAT_100baa18 as *mut u8;
        let a1c = &raw mut DAT_100baa1c as *mut u8;
        let a20 = &raw mut DAT_100baa20 as *mut u8;
        let a24 = &raw mut DAT_100baa24 as *mut u8;
        let a28 = &raw mut DAT_100baa28 as *mut u8;
        let a2c = &raw mut DAT_100baa2c as *mut u8;
        let a8c = &raw mut DAT_100baa8c as *mut u8;
        let c8908 = &raw mut DAT_100c8908 as *mut u8;
        let c8dc0 = &raw mut DAT_100c8dc0 as *mut u8;
        let c8990 = &raw mut DAT_100c8990 as *mut u8;
        let c8b68 = &raw mut DAT_100c8b68 as *mut u8;
        let c8ba8 = &raw mut DAT_100c8ba8 as *mut u8;
        let f7f0 = &raw mut DAT_100bf7f0 as *mut u8;
        let f7ec = &raw mut PTR_DAT_100bf7ec as *mut *mut u8;
        let ccd8 = &raw mut DAT_100bccd8 as *mut u8;

        if DAT_100aa1ac == 0 || (DAT_100adf98 & 1) != 0 {
            let mut local_c: u32 = 0;
            while (local_c as i32) < DAT_100ba9f8 {
                let i_var2 = local_c.wrapping_mul(0x80) as i32;
                let p = rp(a24, i_var2);
                let pc = p.byte_add(0x2a).cast::<*const i8>().read_unaligned();
                if pc.is_null() || *pc == 0 {
                    let pc2 = p.byte_add(0x2e).cast::<*const i8>().read_unaligned();
                    if !pc2.is_null() && *pc2 != 0 {
                        let q = rp(p, 4).cast::<i32>();
                        q.write_unaligned(0);
                        let l = __ftol();
                        wi(p, 0x1a, l as i32);
                    }
                } else {
                    if ri(a18, i_var2) != 0 {
                        let i_var1 = ri(a10, i_var2);
                        if ri(c8908, i_var1.wrapping_mul(4)) != 0 {
                            let tbl = ri(c8dc0, i_var1.wrapping_mul(4));
                            let f = (tbl.wrapping_add(0x10) as usize
                                as *const unsafe extern "C" fn(i32, i32))
                                .read_unaligned();
                            f(ri(c8990, i_var1.wrapping_mul(0x74)), ri(a14, i_var2));
                        }
                    }
                    let t = ri(p, 0x10);
                    wi(p, 0x1a, t);
                    rp(p, 4).cast::<i32>().write_unaligned(0);
                    wi(a1c, i_var2, 0);
                }
                p.byte_add(0x15).write(0);
                local_c = local_c.wrapping_add(1);
            }
            local_c = 0;
            while local_c < 0x59 {
                if ri(f7f0, local_c.wrapping_mul(0x20) as i32) != 0 {
                    let ptr = f7ec.add(local_c.wrapping_mul(8) as usize).read_unaligned();
                    ptr.write(0);
                }
                local_c = local_c.wrapping_add(1);
            }
            local_c = 0;
            while (local_c as i32) < DAT_100bf7d0 {
                if ri(c8908, local_c.wrapping_mul(4) as i32) != 0
                    && (DAT_100ad6e0 == 0 || (local_c as i32) != DAT_100baa08)
                {
                    let tbl = ri(c8dc0, local_c.wrapping_mul(4) as i32);
                    let f = (tbl.wrapping_add(0x14) as usize
                        as *const unsafe extern "C" fn(i32, *mut u8, *mut u8))
                        .read_unaligned();
                    f(
                        ri(c8990, local_c.wrapping_mul(0x74) as i32),
                        c8b68.offset(local_c.wrapping_mul(0x50) as i32 as isize),
                        c8ba8.offset(local_c.wrapping_mul(0x50) as i32 as isize),
                    );
                }
                local_c = local_c.wrapping_add(1);
            }
            local_c = 0;
            while (local_c as i32) < DAT_100baa00 {
                let b = ccd8.offset(local_c.wrapping_mul(0x6e) as i32 as isize);
                let tmp = ri(b, 0xa);
                if DAT_100baa04 != 0 || tmp != 0 {
                    let tmp2 = ri(b, 6);
                    let u_var3 = FUN_100879ee(tmp2, b.byte_add(0xe).cast::<c_void>());
                    let local_28 = u_var3 as u8;
                    if b.byte_add(4).read() == 0 || b.byte_add(5).read() == 0 {
                        let bp = b.cast::<*mut u8>().read_unaligned();
                        bp.write(bp.read() | local_28);
                    }
                    b.byte_add(5).write(local_28);
                }
                local_c = local_c.wrapping_add(1);
            }
            local_c = 0;
            while (local_c as i32) < DAT_100ba9f8 {
                let i_var2 = local_c.wrapping_mul(0x80) as i32;
                if DAT_100baa04 != 0 || ri(a8c, i_var2) != 0 {
                    let p = rp(a24, i_var2);
                    let mut i_var4 = FUN_100885c8(a10.offset(i_var2 as isize).cast::<i32>());
                    if i_var4 == 0 && ri(a18, i_var2) != 0 && {
                        i_var4 = FUN_100879ee(
                            ri(a28, i_var2),
                            a2c.offset(i_var2 as isize).cast::<c_void>(),
                        ) as i32;
                        i_var4 != 0
                    } {
                        let first = rp(a18, i_var2).cast::<i32>().read_unaligned();
                        i_var4 = (first as u32).wrapping_mul(ri(a20, i_var2) as u32) as i32;
                        if ri(a1c, i_var2) == i_var4 {
                            let tmp = ri(p, 0x1a);
                            if rp(p, 4).cast::<i32>().read_unaligned() == tmp {
                                wi(p, 0x1a, i_var4);
                            }
                        } else {
                            wi(p, 0x1a, i_var4);
                        }
                        wi(a1c, i_var2, i_var4);
                    }
                    let mut local_2c = ri(p, 0x1a);
                    if 0x10000 < local_2c {
                        local_2c = 0x10000;
                    }
                    if local_2c < -0x10000 {
                        local_2c = -0x10000;
                    }
                    let prod = (ri(p, 8) as u32).wrapping_mul(local_2c.wrapping_add(0x10000) as u32)
                        as i32;
                    let sum = ((prod / 2) as u32).wrapping_add(ri(p, 0xc) as u32) as i32;
                    let sh = (p.byte_add(0x14).read() & 0x1f) as u32;
                    rp(p, 0).cast::<i32>().write_unaligned(sum.wrapping_shr(sh));
                }
                local_c = local_c.wrapping_add(1);
            }
        }
        let idx = DAT_100baa08.wrapping_mul(4);
        if ri(c8908, idx) != 0 {
            let tbl = ri(c8dc0, idx);
            let f = (tbl.wrapping_add(0x18) as usize as *const unsafe extern "C" fn(*mut i16))
                .read_unaligned();
            f(local_8.as_mut_ptr());
        }
        if local_8[0] == 0x40d {
            FUN_10081fdc();
        } else {
            DAT_100ba96c = local_8[0];
        }
    }
}
