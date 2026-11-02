// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ba9fc: i32;
    static mut DAT_100bc310: [u8; 0];
    static mut DAT_100c243c: [i8; 0];
    static mut DAT_100c2444: [i8; 0];
    static mut DAT_100c244c: [i8; 0];
    static mut DAT_100c2454: [i8; 0];
    static mut DAT_100c245c: [i8; 0];
    static mut DAT_100c2464: [i8; 0];
    static mut DAT_100c2468: i8;
    static mut DAT_100c246c: [i8; 0];
    static mut DAT_100c241c: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strstr(a: *const i8, b: *const i8) -> *mut i8;
    fn FUN_10086a00(param_1: *mut i8) -> *mut u32;
    fn FUN_100108a0(param_1: i32, param_2: *const i8);
}

unsafe fn copy_name(src: *const i8, dst: *mut i8) {
    // SAFETY: src is a NUL-terminated string; dst is the caller's buffer.
    unsafe {
        let mut i = 0usize;
        loop {
            let c = mech_trace::dr!("1008716b", src.add(i));
            mech_trace::dw!("1008716b", dst.add(i), c);
            i += 1;
            if c == 0 {
                break;
            }
        }
    }
}

unsafe fn end_of(buf: *mut i8) -> usize {
    // SAFETY: buf holds a NUL-terminated string.
    unsafe {
        let mut n = 0usize;
        while mech_trace::dr!("1008716b", buf.add(n)) != 0 {
            n += 1;
        }
        n
    }
}

unsafe fn put(buf: *mut i8, off: usize, src: *const i8, len: usize) {
    // SAFETY: copies len bytes from src into buf at off.
    unsafe {
        let mut i = 0usize;
        while i < len {
            mech_trace::dw!(
                "1008716b",
                buf.add(off + i),
                mech_trace::dr!("1008716b", src.add(i))
            );
            i += 1;
        }
    }
}

unsafe fn lookup(buf: *mut i8) -> u32 {
    // SAFETY: the callee is the game's own.
    unsafe {
        let p = mech_trace::cr!("1008716b", "10086a00", FUN_10086a00(buf));
        if p.is_null() {
            0
        } else {
            mech_trace::rd!("1008716b", p.add(1), read_unaligned)
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1008716b(param_1: *mut u32) -> *mut u8 {
    mech_trace::enter!("1008716b", param_1);
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's record; the table and local buffer are accessed as the original does.
    unsafe {
        let base = &raw mut DAT_100bc310 as *mut u8;
        let mut local_8: i32 = 0;
        while local_8 < mech_trace::gr!("1008716b", "100ba9fc", DAT_100ba9fc) {
            let p = base.byte_add(local_8.wrapping_mul(0x32) as usize);
            let k = mech_trace::rd!("1008716b", p.cast::<u32>(), read_unaligned);
            if k == mech_trace::rd!("1008716b", param_1.add(1), read_unaligned) {
                return mech_trace::rt!("1008716b", p);
            }
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = mech_trace::gr!("1008716b", "100ba9fc", DAT_100ba9fc);
        mech_trace::gw!(
            "1008716b",
            "100ba9fc",
            DAT_100ba9fc,
            mech_trace::gr!("1008716b", "100ba9fc", DAT_100ba9fc).wrapping_add(1)
        );
        mech_trace::rt!(
            "1008716b",
            if mech_trace::gr!("1008716b", "100ba9fc", DAT_100ba9fc) < 0x33 {
                let e = base.byte_add(local_8.wrapping_mul(0x32) as usize);
                let name =
                    mech_trace::rd!("1008716b", param_1, read_unaligned) as usize as *const i8;
                let pc = mech_trace::cr!(
                    "1008716b",
                    "_strstr",
                    _strstr(name, &raw const DAT_100c243c as *const i8)
                );
                if pc.is_null() {
                    mech_trace::wr!(
                        "1008716b",
                        e.byte_add(4).cast::<u32>(),
                        write_unaligned,
                        e.byte_add(0x16) as usize as u32
                    );
                } else {
                    mech_trace::wr!(
                        "1008716b",
                        e.byte_add(4).cast::<u32>(),
                        write_unaligned,
                        e.byte_add(0x1a) as usize as u32
                    );
                }
                let p1 = mech_trace::rd!("1008716b", param_1.add(1), read_unaligned);
                mech_trace::wr!("1008716b", e.cast::<u32>(), write_unaligned, p1);
                let p4 = mech_trace::rd!("1008716b", param_1.add(4), read_unaligned);
                let p5 = mech_trace::rd!("1008716b", param_1.add(5), read_unaligned);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(8).cast::<u32>(),
                    write_unaligned,
                    p5.wrapping_sub(p4)
                );
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0xc).cast::<u32>(),
                    write_unaligned,
                    p4.wrapping_shl(16)
                );
                mech_trace::dw!(
                    "1008716b",
                    e.byte_add(0x14),
                    mech_trace::rd!("1008716b", param_1.add(7), read_unaligned) as u8
                );
                let p3 = mech_trace::rd!("1008716b", param_1.add(3), read_unaligned);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    p3.wrapping_shl(16)
                );
                let a = p3.wrapping_shl(16).wrapping_sub(p4.wrapping_shl(16));
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    a
                );
                let a = a.wrapping_shl(1);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    a
                );
                let b = mech_trace::rd!("1008716b", e.byte_add(8).cast::<u32>(), read_unaligned);
                let q: i32;
                // SAFETY: the machine's own idiv, faulting as the original does.
                core::arch::asm!(
                    "cdq",
                    "idiv {d:e}",
                    d = in(reg) b,
                    inout("eax") a => q,
                    out("edx") _,
                );
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    q as u32
                );
                let r = (q as u32).wrapping_add(0xffff0000);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    r
                );
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x1e).cast::<u32>(),
                    write_unaligned,
                    mech_trace::rd!("1008716b", param_1.add(6), read_unaligned)
                );
                let t = mech_trace::rd!("1008716b", e.byte_add(0x10).cast::<u32>(), read_unaligned);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x1a).cast::<u32>(),
                    write_unaligned,
                    t
                );

                let mut buf = [0i8; 40];
                let lb = buf.as_mut_ptr();

                // _plus
                copy_name(name, lb);
                let lc = mech_trace::cr!(
                    "1008716b",
                    "_strstr",
                    _strstr(lb, &raw const DAT_100c2444 as *const i8)
                );
                if !lc.is_null() {
                    mech_trace::dw!("1008716b", lc, 0);
                }
                let n = end_of(lb);
                put(lb, n, &raw const DAT_100c244c as *const i8, 6);
                let v = lookup(lb);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x22).cast::<u32>(),
                    write_unaligned,
                    v
                );

                // _minus
                copy_name(name, lb);
                let lc = mech_trace::cr!(
                    "1008716b",
                    "_strstr",
                    _strstr(lb, &raw const DAT_100c2454 as *const i8)
                );
                if !lc.is_null() {
                    mech_trace::dw!("1008716b", lc, 0);
                }
                let n = end_of(lb);
                put(lb, n, &raw const DAT_100c245c as *const i8, 7);
                let v = lookup(lb);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x26).cast::<u32>(),
                    write_unaligned,
                    v
                );

                // suffix
                copy_name(name, lb);
                let n = end_of(lb);
                put(lb, n, &raw const DAT_100c2464 as *const i8, 4);
                mech_trace::dw!(
                    "1008716b",
                    lb.add(n + 4),
                    mech_trace::gr!("1008716b", "100c2468", DAT_100c2468)
                );
                let v = lookup(lb);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x2e).cast::<u32>(),
                    write_unaligned,
                    v
                );

                // _reset
                copy_name(name, lb);
                let n = end_of(lb);
                put(lb, n, &raw const DAT_100c246c as *const i8, 7);
                let v = lookup(lb);
                mech_trace::wr!(
                    "1008716b",
                    e.byte_add(0x2a).cast::<u32>(),
                    write_unaligned,
                    v
                );

                base.byte_add(local_8.wrapping_mul(0x32) as usize)
            } else {
                FUN_100108a0(0x11, &raw const DAT_100c241c as *const i8);
                core::ptr::null_mut()
            }
        )
    }
}
