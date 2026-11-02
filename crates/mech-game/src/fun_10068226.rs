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
        let code_ptr = mech_trace::rd!("10068226", slot, read_unaligned) as *const u32;
        let code = mech_trace::rd!("10068226", code_ptr, read_unaligned) as usize;
        let f: unsafe extern "C" fn(u32, u32) = core::mem::transmute(code);
        mech_trace::cr!("10068226", "ind", f(obj, arg));
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10068226() {
    mech_trace::enter!("10068226");
    // SAFETY: the globals are the game's own memory; each object is called
    // through its slot in the original order.
    unsafe {
        call_d4(
            mech_trace::gr!("10068226", "10114e08", DAT_10114e08),
            mech_trace::gr!("10068226", "100eeef8", DAT_100eeef8),
        );
        call_d4(
            mech_trace::gr!("10068226", "10114e18", DAT_10114e18),
            mech_trace::gr!("10068226", "100eeefc", DAT_100eeefc),
        );
        call_d4(
            mech_trace::gr!("10068226", "10114df4", DAT_10114df4),
            mech_trace::gr!("10068226", "100eeef4", DAT_100eeef4),
        );
        call_d4(
            mech_trace::gr!("10068226", "10114db8", DAT_10114db8),
            mech_trace::gr!("10068226", "100eeef0", DAT_100eeef0),
        );
        call_d4(
            mech_trace::gr!("10068226", "10114e2c", DAT_10114e2c),
            mech_trace::gr!("10068226", "100eef00", DAT_100eef00),
        );
        call_d4(
            mech_trace::gr!("10068226", "10114e30", DAT_10114e30),
            mech_trace::gr!("10068226", "100eef04", DAT_100eef04),
        );
        call_d4(
            mech_trace::gr!("10068226", "10114e34", DAT_10114e34),
            mech_trace::gr!("10068226", "100eef08", DAT_100eef08),
        );
    }
}
