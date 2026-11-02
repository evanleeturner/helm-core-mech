// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_10098fa8: u32;
    static mut DAT_10098fac: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10044940();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10044970(param_1: i32) {
    // SAFETY: the globals and the callee are the game's own; the globals hold
    // addresses of game structures, written and read at the offsets the
    // original uses.
    unsafe {
        ((DAT_10098fa8 as usize + 0x24) as *mut i32).write_unaligned(param_1);
        let base = ((DAT_10083508 as usize + 0x12a) as *const i32).read_unaligned();
        ((DAT_10098fac as usize + 0x24) as *mut i32).write_unaligned(base.wrapping_add(param_1));
        FUN_10044940();
    }
}
