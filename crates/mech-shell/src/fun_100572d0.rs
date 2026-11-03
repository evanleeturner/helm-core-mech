// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut PTR_s_Glen_Garry_100a1b58: *const u8;
    static mut DAT_100a1b5c: u32;
    static mut PTR_s_Defend_100a1b4c: *const u8;
    static mut PTR_DAT_100a1a00: *const u8;
    static mut s__splt1_100a3630: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056be0(a: i32, b: i32) -> i32;
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

unsafe fn str_eq(mut a: *const u8, mut b: *const u8) -> bool {
    // SAFETY: both pointers reference NUL-terminated strings.
    unsafe {
        loop {
            let x = mech_trace::dr!("100572d0", a);
            if x != mech_trace::dr!("100572d0", b) {
                return false;
            }
            if x == 0 {
                return true;
            }
            a = a.add(1);
            b = b.add(1);
        }
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100572d0(
    param_1: *const u8,
    param_2: *mut u8,
    param_3: *mut i16,
    param_4: *mut i8,
    param_5: *mut i16,
    param_6: i32,
) {
    mech_trace::enter!(
        "100572d0", param_1, param_2, param_3, param_4, param_5, param_6
    );
    // SAFETY: the globals, tables and callees are the game's own; the
    // pointer parameters reference the caller's buffers as the original uses them.
    unsafe {
        let mut local: u32 = 0;
        let bp: i16;
        let mut len = 0usize;
        while mech_trace::dr!("100572d0", param_2.add(len)) != 0 {
            len += 1;
        }
        if len == 0 {
            local = ((mech_trace::cr!("100572d0", "10056be0", FUN_10056be0(0, 0x36)) as u16)
                as u32)
                << 16;
            bp = mech_trace::cr!("100572d0", "10056be0", FUN_10056be0(0, 2)) as i16;
            let idx = mech_trace::cr!("100572d0", "10056be0", FUN_10056be0(0, 0xe));
            let src = mech_trace::rd!(
                "100572d0",
                (&raw const PTR_s_Glen_Garry_100a1b58)
                    .cast::<u8>()
                    .byte_add((idx as usize).wrapping_mul(8))
                    .cast::<*const u8>(),
                read_unaligned
            );
            let mut n = 0usize;
            while mech_trace::dr!("100572d0", src.add(n)) != 0 {
                n += 1;
            }
            n += 1;
            let mut i = 0usize;
            while i < n {
                mech_trace::dw!(
                    "100572d0",
                    param_2.add(i),
                    mech_trace::dr!("100572d0", src.add(i))
                );
                i += 1;
            }
        } else {
            let mut i: i16 = 0;
            loop {
                let s = mech_trace::rd!(
                    "100572d0",
                    (&raw const PTR_s_Glen_Garry_100a1b58)
                        .cast::<u8>()
                        .byte_add((i as isize as usize).wrapping_mul(8))
                        .cast::<*const u8>(),
                    read_unaligned
                );
                let eq = str_eq(s, param_2);
                if eq {
                    break;
                }
                i = i.wrapping_add(1);
                if !(i < 0xf) {
                    break;
                }
            }
            local = mech_trace::rd!(
                "100572d0",
                (&raw const DAT_100a1b5c)
                    .cast::<u8>()
                    .byte_add((i as isize as usize).wrapping_mul(8))
                    .cast::<u32>(),
                read_unaligned
            );
            bp = local as i16;
        }
        let ax: i16;
        loop {
            let r = mech_trace::cr!("100572d0", "10056be0", FUN_10056be0(0, 2)) as i16;
            if r != bp {
                ax = r;
                break;
            }
        }
        let defend = mech_trace::gr!("100572d0", "PTR_s_Defend_100a1b4c", PTR_s_Defend_100a1b4c);
        if str_eq(param_1, defend) {
            mech_trace::wr!("100572d0", param_3, write_unaligned, bp);
            mech_trace::wr!("100572d0", param_5, write_unaligned, ax);
        } else {
            mech_trace::wr!("100572d0", param_5, write_unaligned, bp);
            mech_trace::wr!("100572d0", param_3, write_unaligned, ax);
        }
        if param_6 >= 0xbea {
            mech_trace::wr!(
                "100572d0",
                param_5,
                write_unaligned,
                mech_trace::cr!("100572d0", "10056be0", FUN_10056be0(3, 6)) as i16
            );
        }
        let hi = (local >> 16) as u16 as i16;
        let s = mech_trace::rd!(
            "100572d0",
            (&raw const PTR_DAT_100a1a00)
                .cast::<u8>()
                .byte_add((hi as isize as usize).wrapping_mul(4))
                .cast::<*const u8>(),
            read_unaligned
        );
        mech_trace::cr!(
            "100572d0",
            "_sprintf",
            _sprintf(param_4, (&raw const s__splt1_100a3630).cast::<i8>(), s)
        );
    }
}
