// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adcdc: u32;
    static mut DAT_100b2ce8: u32;
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
pub unsafe extern "stdcall" fn FUN_1006804e() {
    // SAFETY: the globals are the game's own; each holds the address of an
    // object whose slot at offset 0xd4 is a function pointer, called as the
    // original does.
    unsafe {
        let f = ((DAT_10114e08.wrapping_add(0xd4)) as *const unsafe extern "C" fn(u32, i32))
            .read_unaligned();
        f(DAT_10114e08, 9);
        let f = ((DAT_10114e18.wrapping_add(0xd4)) as *const unsafe extern "C" fn(u32, i32))
            .read_unaligned();
        f(DAT_10114e18, 10);
        let f = ((DAT_10114df4.wrapping_add(0xd4)) as *const unsafe extern "C" fn(u32, i32))
            .read_unaligned();
        f(DAT_10114df4, 8);
        let f = ((DAT_10114db8.wrapping_add(0xd4)) as *const unsafe extern "C" fn(u32, i32))
            .read_unaligned();
        f(DAT_10114db8, 1);
        let f = ((DAT_10114e2c.wrapping_add(0xd4)) as *const unsafe extern "C" fn(u32, i32))
            .read_unaligned();
        f(DAT_10114e2c, 0);
        let f = ((DAT_10114e30.wrapping_add(0xd4)) as *const unsafe extern "C" fn(u32, i32))
            .read_unaligned();
        f(DAT_10114e30, 0xc);
        let f = ((DAT_10114e34.wrapping_add(0xd4)) as *const unsafe extern "C" fn(u32, i32))
            .read_unaligned();
        f(DAT_10114e34, 0xd);
        DAT_100b2ce8 = 1;
        DAT_100adcdc = 1;
        DAT_10215660 = 1;
        FUN_10009c1b(1);
    }
}
