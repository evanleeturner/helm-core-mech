// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101686ec: u32;
    static mut PTR_DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031c80(param_1: *mut i32) {
    mech_trace::enter!("10031c80", param_1);
    // SAFETY: param_1 points at the caller's two ints; the globals and the
    // callee are the game's own, and PTR_DAT_100aaf68 holds a valid address
    // whose +0x44 slot is written where the original does.
    unsafe {
        let a = ((mech_trace::rd!("10031c80", param_1.add(1), read_unaligned) as u32)
            .wrapping_shl(0x10)) as i32;
        let b = ((mech_trace::rd!("10031c80", param_1, read_unaligned) as u32).wrapping_shl(0x10))
            as i32;
        mech_trace::gw!(
            "10031c80",
            "101686ec",
            DAT_101686ec,
            mech_trace::cr!("10031c80", "10011e60", FUN_10011e60(a, 0x15555, b))
        );
        mech_trace::wr!(
            "10031c80",
            ((mech_trace::gr!("10031c80", "100aaf68", PTR_DAT_100aaf68).wrapping_add(0x44))
                as *mut u32),
            write_unaligned,
            mech_trace::gr!("10031c80", "101686ec", DAT_101686ec)
        );
    }
}
