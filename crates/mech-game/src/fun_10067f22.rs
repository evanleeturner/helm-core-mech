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
    // SAFETY: the globals are the game's own memory; each object global holds
    // a pointer to an object whose field at offset 0xd4 is a function pointer
    // called with the object and the value, as the original does.
    unsafe {
        if DAT_100b3958 != 0 {
            let o = DAT_10114e08;
            let f = ((o as usize).wrapping_add(0xd4) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            f(o, DAT_100eef28);

            let o = DAT_10114e18;
            let f = ((o as usize).wrapping_add(0xd4) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            f(o, DAT_100eef2c);

            let o = DAT_10114df4;
            let f = ((o as usize).wrapping_add(0xd4) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            f(o, DAT_100eef24);

            let o = DAT_10114db8;
            let f = ((o as usize).wrapping_add(0xd4) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            f(o, DAT_100eef20);

            let o = DAT_10114e2c;
            let f = ((o as usize).wrapping_add(0xd4) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            f(o, DAT_100eef30);

            let o = DAT_10114e30;
            let f = ((o as usize).wrapping_add(0xd4) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            f(o, DAT_100eef34);

            let o = DAT_10114e34;
            let f = ((o as usize).wrapping_add(0xd4) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(f as usize);
            f(o, DAT_100eef38);

            DAT_100b2ce8 = DAT_100eeee0;
            DAT_100adcdc = DAT_100eef0c;
            DAT_10215660 = DAT_100eef10;
            FUN_10009c1b(DAT_100eef10 as i32);
        }
        DAT_100b3958 as i32
    }
}
