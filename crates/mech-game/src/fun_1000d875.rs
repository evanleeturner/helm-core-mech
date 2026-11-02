// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f64a0: u8;
    static mut DAT_101f64ad: u8;
    static mut DAT_101f64b8: u8;
    static mut DAT_101f64c0: u8;
    static mut DAT_100ef030: u8;
    static mut DAT_100a75e4: i32;
    static mut DAT_100af17c: i32;
    static mut DAT_100af0d0: *mut i32;
    static mut DAT_100eeed0: i32;
    static mut DAT_100eeed4: i32;
    static mut DAT_100eeed8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007bdc0() -> u8;
    fn FUN_1001ca17(p: *mut u16) -> u16;
    fn FUN_1008668b(a: i32, b: u32, c: *mut i32) -> *mut u16;
    fn FUN_1001e8a8(a: i32, b: *mut i32, c: u16) -> i32;
    fn FUN_10068d4e(a: u32, b: u32, c: u32);
    fn FUN_10068db6(a: u32);
    fn FUN_10068dc9(a: i32, b: *mut i32, c: i32, d: i32) -> *mut u16;
    fn FUN_1008664f(a: u32, b: *mut i32);
    fn FUN_100895ca(a: *mut i32, b: *mut i32, c: *mut i32);
    fn FUN_1001b7cd(a: *mut u16, b: *mut i32);
    fn FUN_1001ca37(a: u32, b: i16);
    fn FUN_1001ca62(a: u32, b: i16);
    fn FUN_100141c0(a: *mut u16, b: u32);
    fn FUN_1003da99(a: *mut u16);
    fn FUN_1002f272(a: u32, b: u32);
    fn FUN_1001d5e1(a: u32, b: u32);
    fn FUN_1002f3d4(a: u32, b: *mut i32);
    fn FUN_1002fbc0(a: *mut i32);
    fn FUN_1002f668(a: u32, b: u32, c: u32, d: u32, e: i32);
    fn FUN_1002f37a(a: u32, b: u32, c: u32, d: u32);
    fn FUN_1002f90b(a: u32, b: u32);
    fn FUN_1002f969(a: u32, b: u32);
    fn FUN_1002f9c7(a: u32, b: u32);
    fn FUN_10025270(a: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d875(param_1: i32, param_2: i32) -> u32 {
    mech_trace::enter!("1000d875", param_1, param_2);
    let mut local_8: i32 = 0;
    let mut local_4c: [i32; 12] = [0; 12];
    // SAFETY: all globals and callees are the game's own; the record pointers
    // address the game's 0x7c-byte entries, accessed where the original does.
    unsafe {
        let mut i_var4: i32 = (param_1 as u32).wrapping_mul(0x7c) as i32;
        let p: *mut u32 = (&raw mut DAT_101f64a0)
            .byte_add(i_var4 as usize)
            .cast::<u32>();
        let f = |i: usize| -> u32 { mech_trace::rd!("1000d875", p.add(i), read_unaligned) };
        let w = |i: usize, v: u32| mech_trace::wr!("1000d875", p.add(i), write_unaligned, v);

        if (mech_trace::rd!(
            "1000d875",
            (&raw mut DAT_101f64ad).byte_add(i_var4 as usize),
            read
        ) & 8)
            != 0
        {
            return mech_trace::rt!("1000d875", 0);
        }
        if f(0) == 0xffffffff {
            return mech_trace::rt!("1000d875", 0);
        }
        let local_c: i32 = mech_trace::rd!(
            "1000d875",
            (&raw mut DAT_101f64b8)
                .byte_add(i_var4 as usize)
                .cast::<i32>(),
            read_unaligned
        );
        let cond = local_c == -1
            || ((mech_trace::rd!(
                "1000d875",
                (&raw mut DAT_100ef030)
                    .byte_add((local_c as u32).wrapping_mul(0x40) as usize)
                    .cast::<i16>(),
                read_unaligned
            ) as i32 as u32)
                & 0x8000)
                == 0
            || (mech_trace::gr!("1000d875", "100a75e4", DAT_100a75e4) != 0
                && mech_trace::cr!("1000d875", "1007bdc0", FUN_1007bdc0()) == 0);
        if !cond {
            return mech_trace::rt!("1000d875", 0);
        }
        if f(7) != 0 {
            let u_var2 = mech_trace::cr!("1000d875", "1001ca17", FUN_1001ca17(f(7) as *mut u16));
            if (u_var2 & 0x400) == 0 {
                return mech_trace::rt!("1000d875", 1);
            }
        }
        let local_10 = mech_trace::cr!(
            "1000d875",
            "1008668b",
            FUN_1008668b(
                mech_trace::gr!("1000d875", "100af17c", DAT_100af17c),
                f(0),
                mech_trace::gr!("1000d875", "100af0d0", DAT_100af0d0)
            )
        );
        if local_10.is_null() {
            return mech_trace::rt!("1000d875", 0);
        }
        i_var4 = mech_trace::cr!(
            "1000d875",
            "1001e8a8",
            FUN_1001e8a8(
                mech_trace::gr!("1000d875", "100af17c", DAT_100af17c),
                mech_trace::gr!("1000d875", "100af0d0", DAT_100af0d0),
                f(0) as u16
            )
        );
        FUN_10068d4e(f(9), f(10), f(0xb));
        FUN_10068db6(f(3) & 0x1ff);
        let mut local_14: i32 = 0;
        if -1 < (f(2) as i32) {
            local_14 = mech_trace::rd!(
                "1000d875",
                (&raw mut DAT_101f64c0)
                    .byte_add(f(2).wrapping_mul(0x7c) as usize)
                    .cast::<i32>(),
                read_unaligned
            );
        }
        if f(6) != 0xffffffff {
            mech_trace::gw!("1000d875", "100eeed0", DAT_100eeed0, 1);
            mech_trace::gw!("1000d875", "100eeed4", DAT_100eeed4, 0x200);
            mech_trace::gw!("1000d875", "100eeed8", DAT_100eeed8, f(6));
        }
        let puvar5 = mech_trace::cr!(
            "1000d875",
            "10068dc9",
            FUN_10068dc9(local_10 as i32, &raw mut local_8, i_var4, local_14)
        );
        w(7, puvar5 as u32);
        mech_trace::gw!("1000d875", "100eeed0", DAT_100eeed0, 0);
        FUN_1008664f(f(0), mech_trace::gr!("1000d875", "100af0d0", DAT_100af0d0));
        if f(7) == 0 {
            return mech_trace::rt!("1000d875", 0);
        }
        let local_18: u32 = (f(3) & 0xf000) >> 0xc;
        let flag4 = (mech_trace::rd!("1000d875", p.byte_add(0xd).cast::<u8>(), read) & 4) == 0;
        if f(2) == 0xfffffffe {
            FUN_100895ca(
                (param_2 as u32).wrapping_mul(0x7c).wrapping_add(0x101f5568) as *mut i32,
                p.add(0x12).cast::<i32>(),
                local_4c.as_mut_ptr(),
            );
            FUN_1001b7cd(f(7) as *mut u16, local_4c.as_mut_ptr());
            w(8, 0);
            if (mech_trace::rd!("1000d875", p.byte_add(0xd).cast::<u8>(), read) & 4) == 0 {
                FUN_1001ca37(f(7), f(4) as i16);
                FUN_1001ca62(f(7), param_1 as i16);
            } else {
                FUN_1001ca37(f(7), f(4) as i16);
                FUN_1001ca62(f(7), f(6) as i16);
            }
            FUN_100141c0(f(7) as *mut u16, local_18);
            FUN_1003da99(f(7) as *mut u16);
        } else {
            let _ = flag4;
            if f(8) == 0 {
                return mech_trace::rt!("1000d875", 0);
            }
            FUN_1002f272(f(8), f(7));
            FUN_1001d5e1(f(7), f(8));
            FUN_1003da99(f(7) as *mut u16);
            if f(2) == 0xffffffff {
                FUN_100895ca(
                    (param_2 as u32).wrapping_mul(0x7c).wrapping_add(0x101f5568) as *mut i32,
                    p.add(0x12).cast::<i32>(),
                    local_4c.as_mut_ptr(),
                );
                FUN_1002f3d4(f(8), local_4c.as_mut_ptr());
                FUN_1002fbc0(f(8) as *mut i32);
            } else {
                FUN_1002f668(f(8), f(0xc), f(0xd), f(0xe), 0);
                FUN_1002f37a(f(8), f(0xf), f(0x10), f(0x11));
                FUN_1002fbc0(f(8) as *mut i32);
            }
            if (mech_trace::rd!("1000d875", p.byte_add(0xd).cast::<u8>(), read) & 4) == 0 {
                FUN_1001ca37(f(7), f(4) as i16);
                FUN_1001ca62(f(7), param_1 as i16);
            } else {
                FUN_1002f90b(f(8), f(4));
                FUN_1002f969(f(8), f(6));
            }
            FUN_1002f9c7(f(8), local_18);
        }
        if local_18 == 5 {
            FUN_10025270(f(7));
        }
        mech_trace::rt!("1000d875", 1)
    }
}
