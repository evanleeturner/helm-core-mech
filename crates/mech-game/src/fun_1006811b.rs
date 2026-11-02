// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10114db8: u32;
    static mut DAT_10114df4: u32;
    static mut DAT_10114e08: u32;
    static mut DAT_10114e18: u32;
    static mut DAT_10114e2c: u32;
    static mut DAT_10114e30: u32;
    static mut DAT_10114e34: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006811b() {
    // SAFETY: the globals are the game's own; each holds an address whose
    // slot at offset 0xd4 holds a function pointer, called as the original does.
    unsafe {
        let a = DAT_10114e08;
        let f = ((a.wrapping_add(0xd4)) as *const u32).read_unaligned();
        let f: unsafe extern "C" fn(u32, i32) = core::mem::transmute(f as usize);
        f(a, 0);

        let a = DAT_10114e18;
        let f = ((a.wrapping_add(0xd4)) as *const u32).read_unaligned();
        let f: unsafe extern "C" fn(u32, i32) = core::mem::transmute(f as usize);
        f(a, 0);

        let a = DAT_10114df4;
        let f = ((a.wrapping_add(0xd4)) as *const u32).read_unaligned();
        let f: unsafe extern "C" fn(u32, i32) = core::mem::transmute(f as usize);
        f(a, 0);

        let a = DAT_10114db8;
        let f = ((a.wrapping_add(0xd4)) as *const u32).read_unaligned();
        let f: unsafe extern "C" fn(u32, i32) = core::mem::transmute(f as usize);
        f(a, 0);

        let a = DAT_10114e2c;
        let f = ((a.wrapping_add(0xd4)) as *const u32).read_unaligned();
        let f: unsafe extern "C" fn(u32, i32) = core::mem::transmute(f as usize);
        f(a, 0);

        let a = DAT_10114e30;
        let f = ((a.wrapping_add(0xd4)) as *const u32).read_unaligned();
        let f: unsafe extern "C" fn(u32, i32) = core::mem::transmute(f as usize);
        f(a, 0);

        let a = DAT_10114e34;
        let f = ((a.wrapping_add(0xd4)) as *const u32).read_unaligned();
        let f: unsafe extern "C" fn(u32, i32) = core::mem::transmute(f as usize);
        f(a, 0);
    }
}
