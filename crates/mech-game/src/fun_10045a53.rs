// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a2c: *mut u32;
    static mut DAT_100b9a30: u8;
    static mut DAT_100b9d30: u32;
    static mut DAT_101e86b0: u8;
    static mut DAT_101e86b2: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d94c(param_1: *mut u8, param_2: *mut u8) -> u8;
    fn FUN_10045873(param_1: i32, param_2: i32, param_3: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100455b7() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10045a53(param_1: i32) -> u32 {
    mech_trace::enter!("10045a53", param_1);
    let mut local_8: i32 = 0;
    while local_8 < 0x100 {
        let off = local_8.wrapping_mul(3);
        let src = param_1.wrapping_add(off) as *mut u8;
        // SAFETY: param_1 points at the caller's palette bytes; the globals
        // and the callee are the game's own memory, accessed as the original does.
        unsafe {
            let w = mech_trace::rd!("10045a53", src.cast::<u16>(), read_unaligned);
            mech_trace::wr!(
                "10045a53",
                (&raw mut DAT_101e86b0).byte_add(off as usize).cast::<u16>(),
                write_unaligned,
                w
            );
            let b = mech_trace::rd!("10045a53", src.byte_add(2), read);
            mech_trace::wr!(
                "10045a53",
                (&raw mut DAT_101e86b2).byte_add(off as usize),
                write,
                b
            );
            mech_trace::cr!(
                "10045a53",
                "1001d94c",
                FUN_1001d94c(src, (&raw mut DAT_100b9a30).byte_add(off as usize))
            );
        }
        local_8 = local_8.wrapping_add(1);
    }
    // SAFETY: the callees and globals are the game's own; the pointer global
    // is read as the original does.
    unsafe {
        mech_trace::cr!("10045a53", "10045873", FUN_10045873(0, 0x100, 0x100b9a30));
        mech_trace::cr!("10045a53", "100455b7", FUN_100455b7());
        mech_trace::wr!(
            "10045a53",
            mech_trace::gr!("10045a53", "100b9a2c", DAT_100b9a2c),
            write_unaligned,
            mech_trace::gr!("10045a53", "100b9d30", DAT_100b9d30)
        );
    }
    mech_trace::rt!("10045a53", 0)
}
