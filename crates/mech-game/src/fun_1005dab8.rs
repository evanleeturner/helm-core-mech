// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005dab8(param_1: u32, param_2: *mut u32) {
    mech_trace::enter!("1005dab8", param_1, param_2);
    let _ = param_1;
    // SAFETY: param_2 points at the caller's table whose first entry is a
    // cdecl function pointer and whose third entry is passed as an argument;
    // the global is the game's own.
    unsafe {
        let f = mech_trace::rd!("1005dab8", param_2, read_unaligned) as usize;
        let f: unsafe extern "C" fn(u32, u32, u32, u32) = core::mem::transmute(f);
        let a = mech_trace::gr!("1005dab8", "100b38fc", DAT_100b38fc);
        let b = mech_trace::rd!("1005dab8", param_2.add(2), read_unaligned);
        mech_trace::cr!("1005dab8", "ind", f(0xffffffff, 0, a, b));
    }
}
