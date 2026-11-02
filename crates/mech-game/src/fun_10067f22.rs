// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adcdc: u32;
    static mut DAT_100b2ce8: u32;
    static mut DAT_100b3958: u32;
    static mut DAT_100eeee0: u32;
    static mut DAT_100eef0c: u32;
    static mut DAT_100eef10: u32;
    static mut DAT_100eef20: u32;
    static mut DAT_100eef24: u32;
    static mut DAT_100eef28: u32;
    static mut DAT_100eef2c: u32;
    static mut DAT_100eef30: u32;
    static mut DAT_100eef34: u32;
    static mut DAT_100eef38: u32;
    static mut DAT_10114db8: u32;
    static mut DAT_10114df4: u32;
    static mut DAT_10114e08: u32;
    static mut DAT_10114e18: u32;
    static mut DAT_10114e2c: u32;
    static mut DAT_10114e30: u32;
    static mut DAT_10114e34: u32;
    static mut DAT_10215660: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10009c1b(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10067f22() -> i32 {
    mech_trace::enter!("10067f22");
    // SAFETY: the globals are the game's own memory; each object global holds
    // a pointer to an object whose field at offset 0xd4 is a function pointer
    // called with the object and the value, as the original does.
    unsafe {
        if mech_trace::gr!("10067f22", "100b3958", DAT_100b3958) != 0 {
            let o = mech_trace::gr!("10067f22", "10114e08", DAT_10114e08);
            let f = mech_trace::rd!(
                "10067f22",
                ((o as usize).wrapping_add(0xd4) as *const u32),
                read_unaligned
            );
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            mech_trace::cr!(
                "10067f22",
                "ind",
                f(o, mech_trace::gr!("10067f22", "100eef28", DAT_100eef28))
            );

            let o = mech_trace::gr!("10067f22", "10114e18", DAT_10114e18);
            let f = mech_trace::rd!(
                "10067f22",
                ((o as usize).wrapping_add(0xd4) as *const u32),
                read_unaligned
            );
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            mech_trace::cr!(
                "10067f22",
                "ind",
                f(o, mech_trace::gr!("10067f22", "100eef2c", DAT_100eef2c))
            );

            let o = mech_trace::gr!("10067f22", "10114df4", DAT_10114df4);
            let f = mech_trace::rd!(
                "10067f22",
                ((o as usize).wrapping_add(0xd4) as *const u32),
                read_unaligned
            );
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            mech_trace::cr!(
                "10067f22",
                "ind",
                f(o, mech_trace::gr!("10067f22", "100eef24", DAT_100eef24))
            );

            let o = mech_trace::gr!("10067f22", "10114db8", DAT_10114db8);
            let f = mech_trace::rd!(
                "10067f22",
                ((o as usize).wrapping_add(0xd4) as *const u32),
                read_unaligned
            );
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            mech_trace::cr!(
                "10067f22",
                "ind",
                f(o, mech_trace::gr!("10067f22", "100eef20", DAT_100eef20))
            );

            let o = mech_trace::gr!("10067f22", "10114e2c", DAT_10114e2c);
            let f = mech_trace::rd!(
                "10067f22",
                ((o as usize).wrapping_add(0xd4) as *const u32),
                read_unaligned
            );
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            mech_trace::cr!(
                "10067f22",
                "ind",
                f(o, mech_trace::gr!("10067f22", "100eef30", DAT_100eef30))
            );

            let o = mech_trace::gr!("10067f22", "10114e30", DAT_10114e30);
            let f = mech_trace::rd!(
                "10067f22",
                ((o as usize).wrapping_add(0xd4) as *const u32),
                read_unaligned
            );
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            mech_trace::cr!(
                "10067f22",
                "ind",
                f(o, mech_trace::gr!("10067f22", "100eef34", DAT_100eef34))
            );

            let o = mech_trace::gr!("10067f22", "10114e34", DAT_10114e34);
            let f = mech_trace::rd!(
                "10067f22",
                ((o as usize).wrapping_add(0xd4) as *const u32),
                read_unaligned
            );
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            mech_trace::cr!(
                "10067f22",
                "ind",
                f(o, mech_trace::gr!("10067f22", "100eef38", DAT_100eef38))
            );

            mech_trace::gw!(
                "10067f22",
                "100b2ce8",
                DAT_100b2ce8,
                mech_trace::gr!("10067f22", "100eeee0", DAT_100eeee0)
            );
            mech_trace::gw!(
                "10067f22",
                "100adcdc",
                DAT_100adcdc,
                mech_trace::gr!("10067f22", "100eef0c", DAT_100eef0c)
            );
            mech_trace::gw!(
                "10067f22",
                "10215660",
                DAT_10215660,
                mech_trace::gr!("10067f22", "100eef10", DAT_100eef10)
            );
            mech_trace::cr!(
                "10067f22",
                "10009c1b",
                FUN_10009c1b(mech_trace::gr!("10067f22", "100eef10", DAT_100eef10) as i32)
            );
        }
        mech_trace::rt!(
            "10067f22",
            mech_trace::gr!("10067f22", "100b3958", DAT_100b3958) as i32
        )
    }
}
