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
    mech_trace::enter!("10044970", param_1);
    // SAFETY: the globals and the callee are the game's own; the globals hold
    // addresses of game structures, written and read at the offsets the
    // original uses.
    unsafe {
        mech_trace::wr!(
            "10044970",
            ((mech_trace::gr!("10044970", "10098fa8", DAT_10098fa8) as usize + 0x24) as *mut i32),
            write_unaligned,
            param_1
        );
        let base = mech_trace::rd!(
            "10044970",
            ((mech_trace::gr!("10044970", "10083508", DAT_10083508) as usize + 0x12a)
                as *const i32),
            read_unaligned
        );
        mech_trace::wr!(
            "10044970",
            ((mech_trace::gr!("10044970", "10098fac", DAT_10098fac) as usize + 0x24) as *mut i32),
            write_unaligned,
            base.wrapping_add(param_1)
        );
        FUN_10044940();
    }
}
