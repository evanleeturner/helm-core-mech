// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160e68: i32;
    static mut DAT_10160e60: i32;
    static mut DAT_10160c00: [i32; 25];
    static mut DAT_100c6d48: [u8; 0];
    static mut DAT_100c6d24: i32;
    static mut DAT_100c6d1c: i32;
    static mut DAT_100c6d28: i32;
    static mut DAT_100c6d20: i32;
    static mut DAT_100c6d18: i16;
    static mut DAT_100c6d0c: i32;
    static mut DAT_100c6d04: i32;
    static mut DAT_100c6d10: i32;
    static mut DAT_100c6d08: i32;
    static mut DAT_100c6d00: i16;
    static mut DAT_100c6d3c: i32;
    static mut DAT_100c6d34: i32;
    static mut DAT_100c6d40: i32;
    static mut DAT_100c6d38: i32;
    static mut DAT_100c6d30: i16;
    static mut DAT_100c7ad4: i32;
    static mut DAT_100c7acc: i32;
    static mut DAT_100c7ad8: i32;
    static mut DAT_100c7ad0: i32;
    static mut DAT_100c7ac8: i16;
    static mut DAT_100af098: *mut core::ffi::c_void;
    static mut DAT_100af09c: *mut core::ffi::c_void;
    static mut DAT_100aefc4: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003cb2a();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003d47c() {
    mech_trace::enter!("1003d47c");
    // SAFETY: all globals and the callee belong to the game; the table at
    // DAT_100c6d48 is a 12x12 grid of 0x18-byte entries, written in bounds.
    unsafe {
        mech_trace::gw!("1003d47c", "10160e68", DAT_10160e68, 0);
        mech_trace::gw!("1003d47c", "10160e60", DAT_10160e60, 0);
        FUN_1003cb2a();
        let arr = &raw mut DAT_10160c00 as *mut i32;
        let mut local_8: i32 = 0;
        while local_8 < 0x19 {
            mech_trace::wr!("1003d47c", arr.add(local_8 as usize), write, 0);
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = 0;
        while local_8 < 0xc {
            let mut local_c: i32 = 0;
            while local_c < 0xc {
                let base = &raw mut DAT_100c6d48 as *mut u8;
                let e = base
                    .byte_add(local_c.wrapping_mul(0x18) as usize)
                    .byte_add(local_8.wrapping_mul(0x120) as usize);
                mech_trace::wr!(
                    "1003d47c",
                    e.byte_add(0xc).cast::<i32>(),
                    write_unaligned,
                    0
                );
                let v = mech_trace::rd!("1003d47c", e.byte_add(0xc).cast::<i32>(), read_unaligned);
                mech_trace::wr!("1003d47c", e.byte_add(4).cast::<i32>(), write_unaligned, v);
                mech_trace::wr!(
                    "1003d47c",
                    e.byte_add(0x10).cast::<i32>(),
                    write_unaligned,
                    0
                );
                let v = mech_trace::rd!("1003d47c", e.byte_add(0x10).cast::<i32>(), read_unaligned);
                mech_trace::wr!("1003d47c", e.byte_add(8).cast::<i32>(), write_unaligned, v);
                mech_trace::wr!("1003d47c", e.cast::<i16>(), write_unaligned, 0x4000);
                let p = base
                    .byte_add(local_8.wrapping_mul(0x120) as usize)
                    .byte_add(local_c.wrapping_mul(0x18) as usize);
                mech_trace::wr!(
                    "1003d47c",
                    e.byte_add(0x14).cast::<*mut u8>(),
                    write_unaligned,
                    p
                );
                local_c = local_c.wrapping_add(1);
            }
            local_8 = local_8.wrapping_add(1);
        }
        mech_trace::gw!("1003d47c", "100c6d24", DAT_100c6d24, 0);
        mech_trace::gw!("1003d47c", "100c6d1c", DAT_100c6d1c, 0);
        mech_trace::gw!("1003d47c", "100c6d28", DAT_100c6d28, 0);
        mech_trace::gw!("1003d47c", "100c6d20", DAT_100c6d20, 0);
        mech_trace::gw!("1003d47c", "100c6d18", DAT_100c6d18, 0x4000);
        mech_trace::gw!("1003d47c", "100c6d0c", DAT_100c6d0c, 0);
        mech_trace::gw!("1003d47c", "100c6d04", DAT_100c6d04, 0);
        mech_trace::gw!("1003d47c", "100c6d10", DAT_100c6d10, 0);
        mech_trace::gw!("1003d47c", "100c6d08", DAT_100c6d08, 0);
        mech_trace::gw!("1003d47c", "100c6d00", DAT_100c6d00, 0x4000);
        mech_trace::gw!("1003d47c", "100c6d3c", DAT_100c6d3c, 0);
        mech_trace::gw!("1003d47c", "100c6d34", DAT_100c6d34, 0);
        mech_trace::gw!("1003d47c", "100c6d40", DAT_100c6d40, 0);
        mech_trace::gw!("1003d47c", "100c6d38", DAT_100c6d38, 0);
        mech_trace::gw!("1003d47c", "100c6d30", DAT_100c6d30, 0x4000);
        mech_trace::gw!("1003d47c", "100c7ad4", DAT_100c7ad4, 0);
        mech_trace::gw!("1003d47c", "100c7acc", DAT_100c7acc, 0);
        mech_trace::gw!("1003d47c", "100c7ad8", DAT_100c7ad8, 0);
        mech_trace::gw!("1003d47c", "100c7ad0", DAT_100c7ad0, 0);
        mech_trace::gw!("1003d47c", "100c7ac8", DAT_100c7ac8, 0x4000);
        mech_trace::gw!(
            "1003d47c",
            "100af098",
            DAT_100af098,
            (&raw mut DAT_100c6d00).cast()
        );
        mech_trace::gw!(
            "1003d47c",
            "100af09c",
            DAT_100af09c,
            (&raw mut DAT_100c6d30).cast()
        );
        mech_trace::gw!(
            "1003d47c",
            "100aefc4",
            DAT_100aefc4,
            (&raw mut DAT_100c6d18).cast()
        );
    }
}
