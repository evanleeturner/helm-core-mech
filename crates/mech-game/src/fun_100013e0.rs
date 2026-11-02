// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123130: u8;
    static mut DAT_10123230: u32;
    static mut DAT_10123240: u8;
    static mut DAT_10123244: u8;
    static mut DAT_10123248: u8;
    static mut DAT_1012324c: u8;
    static mut DAT_10123250: u8;
    static mut DAT_10123254: u8;
    static mut DAT_10123258: u8;
    static mut DAT_10123268: u8;
    static mut DAT_10123279: u8;
    static mut DAT_1012327a: u8;
    static mut DAT_1012327b: u8;
    static mut DAT_1012327c: u8;
    static mut DAT_10123280: u8;
    static mut DAT_10123281: u8;
    static mut DAT_10123285: u8;
    static mut DAT_10123289: u8;
    static mut DAT_101232e1: u8;
    static mut DAT_101232e5: u8;
    static mut DAT_101232e9: u8;
    static mut DAT_101232ee: u8;
    static mut DAT_101232ef: u8;
    static mut DAT_101232f3: u8;
    static mut DAT_101232f7: u8;
    static mut DAT_101232fb: u8;
    static mut DAT_101232ff: u8;
    static mut DAT_10123303: u8;
    static mut DAT_10123313: u8;
    static mut DAT_10123323: u8;
    static mut DAT_10123325: u8;
    static mut DAT_10123327: u8;
    static mut DAT_10123368: u8;
    static mut DAT_10123369: u8;
    static mut DAT_101600ec: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
    fn FUN_1007b9d2(param_1: u32, param_2: *mut i8) -> i32;
}

unsafe fn copy_str(mut src: *const u8, mut dst: *mut u8) {
    // SAFETY: caller guarantees src is NUL-terminated and dst has room.
    unsafe {
        loop {
            let c = mech_trace::rd!("100013e0", src, read);
            mech_trace::wr!("100013e0", dst, write, c);
            src = src.add(1);
            dst = dst.add(1);
            if c == 0 {
                break;
            }
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100013e0(param_1: i32) {
    mech_trace::enter!("100013e0", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's record, read and written where the original does.
    unsafe {
        let e = param_1 as usize as *mut u8;
        let rd32 = |off: i32| -> i32 {
            mech_trace::rd!(
                "100013e0",
                e.wrapping_add(off as usize).cast::<i32>(),
                read_unaligned
            )
        };
        let ptr = |off: i32| -> *mut u8 { e.wrapping_add(off as usize) };
        let at = |base: *mut u8, off: i32| -> *mut u8 { base.wrapping_add(off as usize) };
        let m = || -> i32 { rd32(8).wrapping_mul(0x3c0a) };

        let u_var2: u32 = (rd32(4) as u32).wrapping_sub(0x26) / 0x97;
        if rd32(0x26) != 0x10 {
            mech_trace::cr!(
                "100013e0",
                "100108a0",
                FUN_100108a0(0x4a, core::ptr::null_mut())
            );
        }
        let v = mech_trace::rd!(
            "100013e0",
            at(&raw mut DAT_101600ec, rd32(8).wrapping_mul(0x3c)),
            read
        );
        mech_trace::wr!("100013e0", at(&raw mut DAT_10123279, m()), write, v);
        mech_trace::wr!(
            "100013e0",
            at(&raw mut DAT_1012324c, m()).cast::<i32>(),
            write_unaligned,
            rd32(0xc)
        );
        mech_trace::wr!(
            "100013e0",
            at(&raw mut DAT_10123244, m()).cast::<u32>(),
            write_unaligned,
            0xffffffff
        );
        mech_trace::wr!(
            "100013e0",
            at(&raw mut DAT_10123248, m()).cast::<u32>(),
            write_unaligned,
            0xffffffff
        );
        let i_var3 = mech_trace::cr!(
            "100013e0",
            "1007b9d2",
            FUN_1007b9d2(10, ptr(0x12) as *mut i8)
        );
        mech_trace::wr!(
            "100013e0",
            at(&raw mut DAT_10123250, m()).cast::<i32>(),
            write_unaligned,
            i_var3
        );
        copy_str(ptr(0x12), at(&raw mut DAT_10123258, m()));
        let i_var3 = mech_trace::cr!(
            "100013e0",
            "1007b9d2",
            FUN_1007b9d2(10, ptr(0x1d) as *mut i8)
        );
        mech_trace::wr!(
            "100013e0",
            at(&raw mut DAT_10123254, m()).cast::<i32>(),
            write_unaligned,
            i_var3
        );
        copy_str(ptr(0x1d), at(&raw mut DAT_10123268, m()));

        let mut c: i32 = 0;
        loop {
            if (u_var2 as i32) <= c {
                mech_trace::wr!(
                    "100013e0",
                    at(&raw mut DAT_10123130, rd32(8).wrapping_mul(4)).cast::<u32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "100013e0",
                    at(&raw mut DAT_10123240, m()).cast::<u32>(),
                    write_unaligned,
                    u_var2
                );
                if (mech_trace::gr!("100013e0", "10123230", DAT_10123230) as i32)
                    < rd32(8).wrapping_add(1)
                {
                    mech_trace::gw!(
                        "100013e0",
                        "10123230",
                        DAT_10123230,
                        rd32(8).wrapping_add(1) as u32
                    );
                }
                return;
            }
            let s = c.wrapping_mul(0x97);
            let d = || -> i32 { m().wrapping_add(c.wrapping_mul(0x13f)) };

            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_1012327b, d()),
                write,
                mech_trace::rd!("100013e0", ptr(s.wrapping_add(0x54)), read)
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_1012327c, d()).cast::<i32>(),
                write_unaligned,
                rd32(s.wrapping_add(0x26))
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_101232e9, d()).cast::<i32>(),
                write_unaligned,
                rd32(s.wrapping_add(0x50))
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_101232e1, d()).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_101232e5, d()).cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_10123323, d()).cast::<u16>(),
                write_unaligned,
                mech_trace::rd!(
                    "100013e0",
                    ptr(s.wrapping_add(0x79)).cast::<u16>(),
                    read_unaligned
                )
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_10123325, d()).cast::<u16>(),
                write_unaligned,
                mech_trace::rd!(
                    "100013e0",
                    ptr(s.wrapping_add(0x7b)).cast::<u16>(),
                    read_unaligned
                )
            );
            if mech_trace::rd!("100013e0", ptr(s.wrapping_add(0x2a)), read) == b'V' {
                mech_trace::wr!("100013e0", at(&raw mut DAT_101232ee, d()), write, 1);
            } else {
                mech_trace::wr!("100013e0", at(&raw mut DAT_101232ee, d()), write, 0);
            }
            if mech_trace::rd!("100013e0", ptr(s.wrapping_add(0x55)), read) == b'M' {
                mech_trace::wr!(
                    "100013e0",
                    at(&raw mut DAT_101232ef, d()).cast::<u32>(),
                    write_unaligned,
                    1
                );
            } else {
                mech_trace::wr!(
                    "100013e0",
                    at(&raw mut DAT_101232ef, d()).cast::<u32>(),
                    write_unaligned,
                    0
                );
            }
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_101232f3, d()).cast::<u32>(),
                write_unaligned,
                u32::from(mech_trace::rd!("100013e0", ptr(s.wrapping_add(0x56)), read))
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_101232f7, d()).cast::<u32>(),
                write_unaligned,
                u32::from(mech_trace::rd!("100013e0", ptr(s.wrapping_add(0x57)), read))
            );

            let i_var3 = mech_trace::cr!(
                "100013e0",
                "1007b9d2",
                FUN_1007b9d2(10, ptr(s.wrapping_add(0x5a)) as *mut i8)
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_101232fb, d()).cast::<i32>(),
                write_unaligned,
                i_var3
            );
            copy_str(ptr(s.wrapping_add(0x5a)), at(&raw mut DAT_10123303, d()));

            let i_var3 = mech_trace::cr!(
                "100013e0",
                "1007b9d2",
                FUN_1007b9d2(10, ptr(s.wrapping_add(0x65)) as *mut i8)
            );
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_101232ff, d()).cast::<i32>(),
                write_unaligned,
                i_var3
            );
            copy_str(ptr(s.wrapping_add(0x65)), at(&raw mut DAT_10123313, d()));

            copy_str(ptr(s.wrapping_add(0x7d)), at(&raw mut DAT_10123327, d()));

            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_10123280, d()),
                write,
                mech_trace::rd!("100013e0", ptr(s.wrapping_add(0x2c)), read)
            );

            let mut k: i32 = 0;
            while k < 8 {
                let local_14: u32 = match mech_trace::rd!(
                    "100013e0",
                    ptr(s.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x30)),
                    read
                ) {
                    0x43 => 1,
                    0x46 => 3,
                    0x49 => 0,
                    0x53 => 2,
                    _ => 0,
                };
                let ko = k.wrapping_mul(0xc);
                mech_trace::wr!(
                    "100013e0",
                    at(&raw mut DAT_10123281, ko.wrapping_add(d())).cast::<u32>(),
                    write_unaligned,
                    local_14
                );
                mech_trace::wr!(
                    "100013e0",
                    at(&raw mut DAT_10123285, ko.wrapping_add(d())).cast::<u32>(),
                    write_unaligned,
                    u32::from(mech_trace::rd!(
                        "100013e0",
                        ptr(s.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x31)),
                        read
                    ),)
                );
                mech_trace::wr!(
                    "100013e0",
                    at(&raw mut DAT_10123289, ko.wrapping_add(d())).cast::<u32>(),
                    write_unaligned,
                    u32::from(mech_trace::rd!(
                        "100013e0",
                        ptr(s.wrapping_add(k.wrapping_mul(4)).wrapping_add(0x32)),
                        read
                    ),)
                );
                k += 1;
            }
            mech_trace::wr!("100013e0", at(&raw mut DAT_1012327a, d()), write, 0);
            mech_trace::wr!("100013e0", at(&raw mut DAT_10123368, d()), write, 0);
            mech_trace::wr!(
                "100013e0",
                at(&raw mut DAT_10123369, d()).cast::<u16>(),
                write_unaligned,
                0x800
            );
            c = c.wrapping_add(1);
        }
    }
}
