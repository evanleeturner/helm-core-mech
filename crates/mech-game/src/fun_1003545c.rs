// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b60: u32;
    static mut DAT_100aaf70: u8;
    static mut DAT_100b36d8: u32;
    static mut DAT_10168290: u32;
    static mut DAT_101682a0: u8;
    static mut DAT_101683a0: u8;
    static mut PTR_DAT_100aaf68: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000ec6b() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003b59a(param_1: i32) -> u8;
    fn FUN_1003b954(param_1: *mut i32) -> u8;
    fn FUN_1003b2f0(param_1: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003545c() {
    mech_trace::enter!("1003545c");
    // SAFETY: the globals, buffers and callees are the game's own memory;
    // the copies read and write the same dword counts the original does.
    unsafe {
        let mut src = (&raw mut DAT_101683a0).cast::<u32>();
        let mut dst = (&raw mut DAT_100aaf70).cast::<u32>();
        let mut n: i32 = 0x1a;
        while n != 0 {
            mech_trace::wr!(
                "1003545c",
                dst,
                write_unaligned,
                mech_trace::rd!("1003545c", src, read_unaligned)
            );
            src = src.add(1);
            dst = dst.add(1);
            n = n.wrapping_sub(1);
        }
        let mut src = (&raw mut DAT_101682a0).cast::<u32>();
        let mut dst = mech_trace::gr!("1003545c", "100aaf68", PTR_DAT_100aaf68).cast::<u32>();
        let mut n: i32 = 0x38;
        while n != 0 {
            mech_trace::wr!(
                "1003545c",
                dst,
                write_unaligned,
                mech_trace::rd!("1003545c", src, read_unaligned)
            );
            src = src.add(1);
            dst = dst.add(1);
            n = n.wrapping_sub(1);
        }
        mech_trace::gw!(
            "1003545c",
            "100b36d8",
            DAT_100b36d8,
            mech_trace::gr!("1003545c", "10168290", DAT_10168290)
        );
        mech_trace::cr!("1003545c", "1000ec6b", FUN_1000ec6b());
        mech_trace::cr!(
            "1003545c",
            "1003b59a",
            FUN_1003b59a(mech_trace::gr!("1003545c", "100aaf68", PTR_DAT_100aaf68) as i32)
        );
        mech_trace::cr!(
            "1003545c",
            "1003b954",
            FUN_1003b954(mech_trace::gr!("1003545c", "100aaf68", PTR_DAT_100aaf68).cast::<i32>())
        );
        mech_trace::cr!(
            "1003545c",
            "1003b2f0",
            FUN_1003b2f0(mech_trace::gr!("1003545c", "100aaf68", PTR_DAT_100aaf68))
        );
        mech_trace::gw!("1003545c", "100a7b60", DAT_100a7b60, 0);
    }
}
