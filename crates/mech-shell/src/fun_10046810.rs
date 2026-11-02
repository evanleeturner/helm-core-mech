// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098f7c: *mut u32;
    static mut DAT_10098f84: u32;
    static mut DAT_100ba320: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063f40(param_1: *mut i32) -> *mut i32;
    fn FUN_10063ea0(param_1: *mut i32);
    fn FUN_1006e790(param_1: *mut core::ffi::c_void);
    fn FUN_10045b60(param_1: i32, param_2: i32) -> *mut u32;
    fn FUN_10063d30(param_1: *mut i32);
    fn FUN_10063de0(param_1: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10044940();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10046810(param_1: i32, param_2: *mut i8) {
    mech_trace::enter!("10046810", param_1, param_2);
    // SAFETY: globals and callees are the game's own; param_2 is a
    // NUL-terminated string when non-null, copied into the game's buffer.
    unsafe {
        let mut i_var6: i32 = -1;
        if !param_2.is_null() {
            let mut len: u32 = 0;
            while mech_trace::dr!("10046810", param_2.add(len as usize)) != 0 {
                len = len.wrapping_add(1);
            }
            let total = len.wrapping_add(1);
            let mut src = param_2 as *const u8;
            let mut dst = (&raw mut DAT_100ba320).cast::<u8>();
            let mut n = total >> 2;
            while n != 0 {
                mech_trace::wr!(
                    "10046810",
                    (dst as *mut u32),
                    write_unaligned,
                    mech_trace::rd!("10046810", (src as *const u32), read_unaligned)
                );
                src = src.add(4);
                dst = dst.add(4);
                n -= 1;
            }
            let mut r = total & 3;
            while r != 0 {
                mech_trace::dw!("10046810", dst, mech_trace::dr!("10046810", src));
                src = src.add(1);
                dst = dst.add(1);
                r -= 1;
            }
        }
        let mut pi_var3 = mech_trace::cr!(
            "10046810",
            "10063f40",
            FUN_10063f40(mech_trace::gr!("10046810", "10098f7c", DAT_10098f7c) as *mut i32)
        );
        if !pi_var3.is_null() && 0 < mech_trace::rd!("10046810", pi_var3.add(10), read_unaligned) {
            i_var6 = mech_trace::rd!("10046810", pi_var3.add(9), read_unaligned);
        }
        FUN_10063ea0(mech_trace::gr!("10046810", "10098f7c", DAT_10098f7c) as *mut i32);
        if !mech_trace::gr!("10046810", "10098f7c", DAT_10098f7c).is_null() {
            FUN_1006e790(
                mech_trace::gr!("10046810", "10098f7c", DAT_10098f7c) as *mut core::ffi::c_void
            );
        }
        mech_trace::gw!(
            "10046810",
            "10098f7c",
            DAT_10098f7c,
            mech_trace::cr!("10046810", "10045b60", FUN_10045b60(param_1, 1))
        );
        mech_trace::gw!("10046810", "10098f84", DAT_10098f84, param_1 as u32);
        FUN_10063d30(mech_trace::gr!("10046810", "10098f7c", DAT_10098f7c) as *mut i32);
        if -1 < i_var6 {
            pi_var3 = mech_trace::gr!("10046810", "10098f7c", DAT_10098f7c) as *mut i32;
            if mech_trace::rd!(
                "10046810",
                mech_trace::gr!("10046810", "10098f7c", DAT_10098f7c),
                read_unaligned
            ) != 0xffff_ffff
            {
                while mech_trace::rd!("10046810", pi_var3.add(9), read_unaligned) != i_var6 {
                    let p = pi_var3.add(14);
                    pi_var3 = p;
                    if mech_trace::rd!("10046810", p, read_unaligned) == -1 {
                        FUN_10044940();
                        return;
                    }
                }
                mech_trace::wr!("10046810", pi_var3.add(0xb), write_unaligned, 2);
                mech_trace::wr!("10046810", pi_var3.add(0x19), write_unaligned, 2);
                mech_trace::wr!("10046810", pi_var3.add(0x27), write_unaligned, 2);
                FUN_10063de0(mech_trace::gr!("10046810", "10098f7c", DAT_10098f7c) as *mut i32);
            }
        }
        FUN_10044940();
    }
}
