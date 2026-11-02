// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10123230: i32;
    static mut DAT_10168410: [u8; 0];
    static mut DAT_10123240: [u8; 0];
    static mut DAT_1012327a: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10001d0a(str1: *mut i8) -> u32 {
    mech_trace::enter!("10001d0a", str1);
    let mut local_18: u32 = 0;
    // SAFETY: the globals are the game's own memory; str1 points at the
    // caller's NUL-terminated string; offsets mirror the original's layout.
    unsafe {
        if mech_trace::dr!("10001d0a", str1) == 0 {
            local_18 = 0;
        } else {
            let mut local_14: i32 = 0;
            while local_14 < mech_trace::gr!("10001d0a", "10123230", DAT_10123230) {
                let tbl = (&raw mut DAT_10168410)
                    .cast::<u8>()
                    .byte_offset(local_14.wrapping_mul(4) as isize);
                if mech_trace::rd!("10001d0a", tbl.cast::<i32>(), read_unaligned) != 0 {
                    let mut local_c: i32 = 0;
                    loop {
                        let cnt = mech_trace::rd!(
                            "10001d0a",
                            (&raw mut DAT_10123240)
                                .cast::<u8>()
                                .byte_offset(local_14.wrapping_mul(0x3c0a) as isize)
                                .cast::<i32>(),
                            read_unaligned
                        );
                        if !(local_c < cnt) {
                            break;
                        }
                        let off = local_14
                            .wrapping_mul(0x3c0a)
                            .wrapping_add(local_c.wrapping_mul(0x13f));
                        let p = (&raw mut DAT_1012327a)
                            .cast::<i8>()
                            .byte_offset(off as isize);
                        let c_var1 = mech_trace::rd!("10001d0a", p, read);
                        if c_var1 == 0 || c_var1 == 1 || c_var1 == 7 {
                            let base = mech_trace::rd!(
                                "10001d0a",
                                (&raw mut DAT_10168410)
                                    .cast::<u8>()
                                    .byte_offset(local_14.wrapping_mul(4) as isize)
                                    .cast::<i32>(),
                                read_unaligned
                            );
                            let s = (base
                                .wrapping_add(local_c.wrapping_mul(0x97))
                                .wrapping_add(0x70))
                                as *const i8;
                            let i_var2 =
                                mech_trace::cr!("10001d0a", "__strcmpi", __strcmpi(str1, s));
                            if i_var2 == 0 {
                                local_18 = local_18
                                    | 1u32.wrapping_shl(((local_c as u8) & 0x1f) as u32)
                                    | 1u32
                                        .wrapping_shl(((local_14 as u8) & 0x1f) as u32)
                                        .wrapping_shl(0x10);
                                mech_trace::wr!("10001d0a", p, write, 7);
                            }
                        }
                        local_c = local_c.wrapping_add(1);
                    }
                }
                local_14 = local_14.wrapping_add(1);
            }
        }
    }
    mech_trace::rt!("10001d0a", local_18)
}
