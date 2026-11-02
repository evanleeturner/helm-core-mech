// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52c8: *mut i32;
    static mut DAT_101f52e0: i32;
    static mut DAT_101f52e4: i32;
    static mut DAT_101f52e8: i32;
    static mut DAT_101f52ec: i32;
    static mut DAT_101f52f0: i32;
    static mut DAT_101f5300: i32;
    static mut DAT_100c4f38: [i32; 5];
    static mut DAT_100a7b64: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000e530() -> i32;
    fn FUN_10031cbe(param_1: *mut i32);
    fn FUN_10031c80(param_1: *mut i32);
    fn FUN_10060390(param_1: *mut i32);
    fn FUN_10070b60();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000e612() -> i32 {
    mech_trace::enter!("1000e612");
    // SAFETY: the callees and globals are the game's own memory; DAT_101f52c8
    // points at at least two ints, read where the original reads them.
    unsafe {
        if mech_trace::cr!("1000e612", "1000e530", FUN_1000e530()) == 0 {
            return mech_trace::rt!("1000e612", 0);
        }
        FUN_10031cbe(mech_trace::gr!("1000e612", "101f52c8", DAT_101f52c8));
        FUN_10031c80(mech_trace::gr!("1000e612", "101f52c8", DAT_101f52c8));
        mech_trace::gw!(
            "1000e612",
            "101f52e0",
            DAT_101f52e0,
            (&raw mut DAT_101f5300) as usize as i32
        );
        mech_trace::gw!("1000e612", "101f52e4", DAT_101f52e4, 0);
        mech_trace::gw!("1000e612", "101f52e8", DAT_101f52e8, 0);
        mech_trace::gw!(
            "1000e612",
            "101f52ec",
            DAT_101f52ec,
            (mech_trace::rd!(
                "1000e612",
                mech_trace::gr!("1000e612", "101f52c8", DAT_101f52c8),
                read_unaligned
            ) as u32)
                .wrapping_sub(1) as i32
        );
        mech_trace::gw!(
            "1000e612",
            "101f52f0",
            DAT_101f52f0,
            (mech_trace::rd!(
                "1000e612",
                mech_trace::gr!("1000e612", "101f52c8", DAT_101f52c8).add(1),
                read_unaligned
            ) as u32)
                .wrapping_sub(1) as i32
        );
        let p = &raw mut DAT_100c4f38 as *mut i32;
        mech_trace::wr!(
            "1000e612",
            p,
            write_unaligned,
            mech_trace::gr!("1000e612", "101f52e0", DAT_101f52e0)
        );
        mech_trace::wr!(
            "1000e612",
            p.add(1),
            write_unaligned,
            mech_trace::gr!("1000e612", "101f52e4", DAT_101f52e4)
        );
        mech_trace::wr!(
            "1000e612",
            p.add(2),
            write_unaligned,
            mech_trace::gr!("1000e612", "101f52e8", DAT_101f52e8)
        );
        mech_trace::wr!(
            "1000e612",
            p.add(3),
            write_unaligned,
            mech_trace::gr!("1000e612", "101f52ec", DAT_101f52ec)
        );
        mech_trace::wr!(
            "1000e612",
            p.add(4),
            write_unaligned,
            mech_trace::gr!("1000e612", "101f52f0", DAT_101f52f0)
        );
        FUN_10060390(&raw mut DAT_101f52e0);
        FUN_10070b60();
        mech_trace::gw!("1000e612", "100a7b64", DAT_100a7b64, 1);
        mech_trace::rt!("1000e612", 1)
    }
}
