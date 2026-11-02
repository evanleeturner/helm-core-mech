// SPDX-License-Identifier: MIT
use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100f3044: u8;
    static mut DAT_100f3048: u8;
    static mut DAT_100f304c: u8;
    static mut DAT_100f3050: u8;

    static mut DAT_10168398: i32;
    static mut _DAT_10168384: i32;
    static mut DAT_10168380: i32;
    static mut DAT_10168388: i32;
    static mut DAT_1016838c: i32;
    static mut _DAT_10168390: i32;
    static mut DAT_10168394: u32;
    static mut DAT_10168290: u32;
    static mut DAT_100b36d8: u32;
    static mut DAT_101682a0: u32;
    static mut DAT_101683a0: u32;
    static mut DAT_100aaf70: u32;
    static mut DAT_100aaf68: *mut u8;
    static mut DAT_100aafc8: Option<unsafe extern "C" fn()>;
    static mut DAT_100aafcc: Option<unsafe extern "C" fn()>;
    static mut DAT_100a7b60: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100354cd();
    fn FUN_1003567a();
    fn FUN_100603ff(param_1: i32);
    fn FUN_1003b59a(param_1: i32);
    fn FUN_1003b954(param_1: *mut i32);
    fn FUN_1003b2f0(param_1: *mut u8);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn RaiseException(a: u32, b: u32, c: u32, d: *const u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10035220(param_1: *mut u32, param_2: i32, param_3: i32, param_4: u32) {
    mech_trace::enter!("10035220", param_1, param_2, param_3, param_4);
    // SAFETY: all globals, callees and the pointers are the game's own memory;
    // reads and writes follow the original's order and offsets.
    unsafe {
        let off = param_2.wrapping_mul(0x14) as isize;
        let hi = mech_trace::rd!(
            "10035220",
            (&raw mut DAT_100f304c).byte_offset(off).cast::<i32>(),
            read_unaligned
        );
        let lo = mech_trace::rd!(
            "10035220",
            (&raw mut DAT_100f3044).byte_offset(off).cast::<i32>(),
            read_unaligned
        );
        let i_div = (hi as u32).wrapping_sub(lo as u32).wrapping_add(1) as i32;
        if i_div == 0 || (param_3 == i32::MIN && i_div == -1) {
            RaiseException(0xC0000094, 0, 0, core::ptr::null());
        }
        let q: i32;
        asm!(
            "cdq",
            "idiv {d:e}",
            d = in(reg) i_div,
            inout("eax") param_3 => q,
            out("edx") _,
        );
        mech_trace::gw!("10035220", "10168398", DAT_10168398, q);
        mech_trace::gw!("10035220", "10168384", _DAT_10168384, param_3 / 2);
        mech_trace::gw!(
            "10035220",
            "10168380",
            DAT_10168380,
            0u32.wrapping_sub(mech_trace::gr!("10035220", "10168384", _DAT_10168384) as u32) as i32
        );
        let h2 = mech_trace::rd!(
            "10035220",
            (&raw mut DAT_100f3050).byte_offset(off).cast::<i32>(),
            read_unaligned
        );
        let l2 = mech_trace::rd!(
            "10035220",
            (&raw mut DAT_100f3048).byte_offset(off).cast::<i32>(),
            read_unaligned
        );
        let span = (h2 as u32).wrapping_sub(l2 as u32).wrapping_add(1);
        mech_trace::gw!(
            "10035220",
            "10168388",
            DAT_10168388,
            (span.wrapping_mul(mech_trace::gr!("10035220", "10168398", DAT_10168398) as u32)
                as i32)
                / 2
        );
        mech_trace::gw!(
            "10035220",
            "1016838c",
            DAT_1016838c,
            0u32.wrapping_sub(mech_trace::gr!("10035220", "10168388", DAT_10168388) as u32) as i32
        );
        mech_trace::gw!("10035220", "10168390", _DAT_10168390, 0);
        mech_trace::gw!("10035220", "10168394", DAT_10168394, param_4);
        mech_trace::gw!(
            "10035220",
            "10168290",
            DAT_10168290,
            mech_trace::gr!("10035220", "100b36d8", DAT_100b36d8)
        );

        let mut src = mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68) as *const u32;
        let mut dst = &raw mut DAT_101682a0;
        let mut n: i32 = 0x38;
        while n != 0 {
            mech_trace::wr!(
                "10035220",
                dst,
                write_unaligned,
                mech_trace::rd!("10035220", src, read_unaligned)
            );
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }

        let p = mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68);
        mech_trace::wr!(
            "10035220",
            p.byte_add(0xc).cast::<u32>(),
            write_unaligned,
            mech_trace::rd!("10035220", param_1.add(3), read_unaligned)
        );
        let p = mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68);
        mech_trace::wr!(
            "10035220",
            p.byte_add(0x10).cast::<u32>(),
            write_unaligned,
            mech_trace::rd!("10035220", param_1.add(4), read_unaligned)
        );
        let p = mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68);
        mech_trace::wr!(
            "10035220",
            p.byte_add(0x14).cast::<u32>(),
            write_unaligned,
            mech_trace::rd!("10035220", param_1.add(5), read_unaligned)
        );
        let p = mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68);
        mech_trace::wr!(
            "10035220",
            p.cast::<u32>(),
            write_unaligned,
            mech_trace::rd!("10035220", param_1, read_unaligned)
        );
        let p = mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68);
        mech_trace::wr!(
            "10035220",
            p.byte_add(4).cast::<u32>(),
            write_unaligned,
            mech_trace::rd!("10035220", param_1.add(1), read_unaligned)
        );
        let p = mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68);
        mech_trace::wr!(
            "10035220",
            p.byte_add(8).cast::<u32>(),
            write_unaligned,
            mech_trace::rd!("10035220", param_1.add(2), read_unaligned)
        );

        FUN_100603ff(param_2);
        mech_trace::wr!(
            "10035220",
            mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68)
                .byte_add(0x40)
                .cast::<u32>(),
            write_unaligned,
            mech_trace::gr!("10035220", "10168394", DAT_10168394)
        );

        let mut src = &raw mut DAT_100aaf70 as *const u32;
        let mut dst = &raw mut DAT_101683a0;
        let mut n: i32 = 0x1a;
        while n != 0 {
            mech_trace::wr!(
                "10035220",
                dst,
                write_unaligned,
                mech_trace::rd!("10035220", src, read_unaligned)
            );
            src = src.add(1);
            dst = dst.add(1);
            n -= 1;
        }

        mech_trace::gw!("10035220", "100aafc8", DAT_100aafc8, Some(FUN_100354cd));
        mech_trace::gw!("10035220", "100aafcc", DAT_100aafcc, Some(FUN_1003567a));
        FUN_1003b59a(mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68) as usize as i32);
        mech_trace::wr!(
            "10035220",
            mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68)
                .byte_add(0x9c)
                .cast::<u32>(),
            write_unaligned,
            0x2000
        );
        mech_trace::wr!(
            "10035220",
            mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68)
                .byte_add(0xa4)
                .cast::<i16>(),
            write_unaligned,
            3
        );
        let v = mech_trace::rd!(
            "10035220",
            mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68)
                .byte_add(0x44)
                .cast::<i32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "10035220",
            mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68)
                .byte_add(0xa0)
                .cast::<i32>(),
            write_unaligned,
            v >> 3
        );
        mech_trace::wr!(
            "10035220",
            mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68)
                .byte_add(0xa6)
                .cast::<i16>(),
            write_unaligned,
            3
        );
        FUN_1003b954(mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68).cast::<i32>());
        FUN_1003b2f0(mech_trace::gr!("10035220", "100aaf68", DAT_100aaf68));
        mech_trace::gw!("10035220", "100a7b60", DAT_100a7b60, 0);
    }
}
