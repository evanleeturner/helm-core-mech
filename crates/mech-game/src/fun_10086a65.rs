// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100bf7d0: u32;
    static mut DAT_100c8908: u8;
    static mut DAT_100c8920: u8;
    static mut DAT_100c8cf8: u8;
    static mut DAT_100c8dc0: u8;
    static mut DAT_100bf7d8: u32;
    static mut DAT_100c238c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
    fn __strcmpi(_str1: *mut i8, _str2: *mut i8) -> i32;
    fn FUN_10088ec7(param_1: i32, param_2: u32, param_3: *mut i8) -> u8;
}

unsafe fn copy_cstr(dst: *mut u8, src: *const u8) {
    // SAFETY: src is a NUL-terminated string and dst is the game's buffer
    // large enough for it, as in the original.
    unsafe {
        let mut count: u32 = 0xffff_ffff;
        let mut p = src;
        loop {
            if count == 0 {
                break;
            }
            count = count.wrapping_sub(1);
            let c = mech_trace::rd!("10086a65", p, read);
            p = p.add(1);
            if c == 0 {
                break;
            }
        }
        let n = !count;
        let mut s = src;
        let mut d = dst;
        let mut q = n >> 2;
        while q != 0 {
            mech_trace::wr!(
                "10086a65",
                d.cast::<u32>(),
                write_unaligned,
                mech_trace::rd!("10086a65", s.cast::<u32>(), read_unaligned)
            );
            s = s.add(4);
            d = d.add(4);
            q = q.wrapping_sub(1);
        }
        let mut r = n & 3;
        while r != 0 {
            mech_trace::wr!("10086a65", d, write, mech_trace::rd!("10086a65", s, read));
            s = s.add(1);
            d = d.add(1);
            r = r.wrapping_sub(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10086a65(param_1: *mut i8) -> i32 {
    mech_trace::enter!("10086a65", param_1);
    // SAFETY: all globals, device tables and callbacks are the game's own
    // memory; param_1 is the caller's NUL-terminated string.
    unsafe {
        let mut local_c: i32 = 0;
        let mut i_var2: i32;
        let mut i_var4: i32 = 0;
        let mut found = false;

        'outer: loop {
            i_var2 = mech_trace::gr!("10086a65", "100bf7d0", DAT_100bf7d0) as i32;
            if !(local_c < 3) {
                break;
            }
            let tbl = mech_trace::rd!(
                "10086a65",
                (&raw const DAT_100bf7d8)
                    .cast::<u32>()
                    .add(local_c as usize),
                read_unaligned
            );
            if tbl != 0 {
                let tp = tbl as usize as *const u8;
                let f: unsafe extern "C" fn() -> i32 = mech_trace::rd!(
                    "10086a65",
                    tp.cast::<unsafe extern "C" fn() -> i32>(),
                    read_unaligned
                );
                i_var2 = mech_trace::cr!("10086a65", "ind", f());
                if i_var2 != 0 {
                    let mut local_10: i32 = 0;
                    loop {
                        i_var4 = mech_trace::gr!("10086a65", "100bf7d0", DAT_100bf7d0) as i32;
                        if !(local_10 < i_var2) {
                            break;
                        }
                        if 4 < mech_trace::gr!("10086a65", "100bf7d0", DAT_100bf7d0) {
                            mech_trace::cr!(
                                "10086a65",
                                "100108a0",
                                FUN_100108a0(0x11, (&raw mut DAT_100c238c).cast::<i8>())
                            );
                            return mech_trace::rt!("10086a65", -1);
                        }
                        let g: unsafe extern "C" fn(i32, *mut u8) -> i32 = mech_trace::rd!(
                            "10086a65",
                            tp.byte_add(4)
                                .cast::<unsafe extern "C" fn(i32, *mut u8) -> i32>(),
                            read_unaligned
                        );
                        let buf = (&raw mut DAT_100c8920).wrapping_byte_add(
                            mech_trace::gr!("10086a65", "100bf7d0", DAT_100bf7d0).wrapping_mul(0x74)
                                as usize,
                        );
                        let i_var3 = mech_trace::cr!("10086a65", "ind", g(local_10, buf));
                        if i_var3 == 0 {
                            let buf4 = (&raw mut DAT_100c8920)
                                .wrapping_byte_add((i_var4 as u32).wrapping_mul(0x74) as usize);
                            let i_var3 = mech_trace::cr!(
                                "10086a65",
                                "__strcmpi",
                                __strcmpi(buf4.cast::<i8>(), param_1)
                            );
                            if i_var3 == 0 {
                                found = true;
                                break 'outer;
                            }
                            let h: unsafe extern "C" fn(*mut u8) = mech_trace::rd!(
                                "10086a65",
                                tp.byte_add(0xc).cast::<unsafe extern "C" fn(*mut u8)>(),
                                read_unaligned
                            );
                            mech_trace::cr!("10086a65", "ind", h(buf4));
                        }
                        local_10 = local_10.wrapping_add(1);
                    }
                }
            }
            local_c = local_c.wrapping_add(1);
        }

        if !found {
            mech_trace::gw!(
                "10086a65",
                "100bf7d0",
                DAT_100bf7d0,
                mech_trace::gr!("10086a65", "100bf7d0", DAT_100bf7d0).wrapping_add(1)
            );
            let dst = (&raw mut DAT_100c8cf8)
                .wrapping_byte_add((i_var2 as u32).wrapping_mul(0x28) as usize);
            copy_cstr(dst, param_1.cast::<u8>());
            mech_trace::wr!(
                "10086a65",
                (&raw mut DAT_100c8908)
                    .wrapping_byte_add((i_var2 as u32).wrapping_mul(4) as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::cr!("10086a65", "10088ec7", FUN_10088ec7(0x6e, 0, param_1));
            return mech_trace::rt!("10086a65", -1);
        }

        let dst =
            (&raw mut DAT_100c8cf8).wrapping_byte_add((i_var4 as u32).wrapping_mul(0x28) as usize);
        copy_cstr(dst, param_1.cast::<u8>());
        let tbl = mech_trace::rd!(
            "10086a65",
            (&raw const DAT_100bf7d8)
                .cast::<u32>()
                .add(local_c as usize),
            read_unaligned
        );
        let slot = (&raw mut DAT_100c8dc0)
            .wrapping_byte_add((i_var4 as u32).wrapping_mul(4) as usize)
            .cast::<u32>();
        mech_trace::wr!("10086a65", slot, write_unaligned, tbl);
        mech_trace::gw!(
            "10086a65",
            "100bf7d0",
            DAT_100bf7d0,
            mech_trace::gr!("10086a65", "100bf7d0", DAT_100bf7d0).wrapping_add(1)
        );
        let tp = mech_trace::rd!("10086a65", slot, read_unaligned) as usize as *const u8;
        let k: unsafe extern "C" fn(*mut u8) -> i32 = mech_trace::rd!(
            "10086a65",
            tp.byte_add(8)
                .cast::<unsafe extern "C" fn(*mut u8) -> i32>(),
            read_unaligned
        );
        let r = mech_trace::cr!(
            "10086a65",
            "ind",
            k((&raw mut DAT_100c8920)
                .wrapping_byte_add((i_var4 as u32).wrapping_mul(0x74) as usize))
        );
        let flag = (&raw mut DAT_100c8908)
            .wrapping_byte_add((i_var4 as u32).wrapping_mul(4) as usize)
            .cast::<u32>();
        if r == 0 {
            mech_trace::wr!("10086a65", flag, write_unaligned, 1);
        } else {
            mech_trace::cr!(
                "10086a65",
                "10088ec7",
                FUN_10088ec7(
                    0x70,
                    0,
                    (i_var4 as u32).wrapping_mul(0x74).wrapping_add(0x100c892c) as usize as *mut i8,
                )
            );
            mech_trace::wr!("10086a65", flag, write_unaligned, 0);
            i_var4 = -1;
        }
        mech_trace::rt!("10086a65", i_var4)
    }
}
