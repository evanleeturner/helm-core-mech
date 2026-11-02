// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eeef0: u32;
    static mut DAT_100eeef4: u32;
    static mut DAT_100eeef8: u32;
    static mut DAT_100eeefc: u32;
    static mut DAT_100eef00: u32;
    static mut DAT_100eef04: u32;
    static mut DAT_100eef08: u32;
    static mut DAT_10114db8: u32;
    static mut DAT_10114df4: u32;
    static mut DAT_10114e08: u32;
    static mut DAT_10114e18: u32;
    static mut DAT_10114e2c: u32;
    static mut DAT_10114e30: u32;
    static mut DAT_10114e34: u32;
}

#[inline(always)]
unsafe fn call_d4(obj: u32, arg: u32) {
    // SAFETY: obj is a game object whose slot at +0xd4 holds a pointer to a
    // code pointer; both are read and the code is called as the original does.
    unsafe {
        let slot = obj.wrapping_add(0xd4) as *const u32;
        let code_ptr = slot.read_unaligned() as *const u32;
        let code = code_ptr.read_unaligned() as usize;
        let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(code);
        f(obj, arg);
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10068226() {
    // SAFETY: the globals are the game's own memory; each object is called
    // through its slot in the original order.
    unsafe {
        call_d4(DAT_10114e08, DAT_100eeef8);
        call_d4(DAT_10114e18, DAT_100eeefc);
        call_d4(DAT_10114df4, DAT_100eeef4);
        call_d4(DAT_10114db8, DAT_100eeef0);
        call_d4(DAT_10114e2c, DAT_100eef00);
        call_d4(DAT_10114e30, DAT_100eef04);
        call_d4(DAT_10114e34, DAT_100eef08);
    }
}
