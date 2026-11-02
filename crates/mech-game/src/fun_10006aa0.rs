// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4110: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_1002f783(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10006aa0() {
    mech_trace::enter!("10006aa0");
    // SAFETY: the globals and callees are the game's own memory and code; the
    // table entry points at a structure whose field at +0x40 is read as an int.
    unsafe {
        let base = (&raw mut DAT_100eef40).cast::<u32>();
        let entry = mech_trace::rd!(
            "10006aa0",
            base.add(mech_trace::gr!("10006aa0", "100adf38", DAT_100adf38) as usize),
            read_unaligned
        );
        let arg = mech_trace::rd!(
            "10006aa0",
            (entry.wrapping_add(0x40) as *const i32),
            read_unaligned
        );
        if mech_trace::gr!("10006aa0", "100a4110", DAT_100a4110) == 0 {
            mech_trace::cr!("10006aa0", "1002f727", FUN_1002f727(arg));
        } else {
            mech_trace::cr!("10006aa0", "1002f783", FUN_1002f783(arg));
        }
        mech_trace::gw!(
            "10006aa0",
            "100a4110",
            DAT_100a4110,
            (mech_trace::gr!("10006aa0", "100a4110", DAT_100a4110) == 0) as u32
        );
    }
}
