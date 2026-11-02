// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: i32;
    static mut DAT_100eef40: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008276d(param_1: i32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10083bdd(param_1: i32) {
    mech_trace::enter!("10083bdd", param_1);
    // SAFETY: the globals and the callee are the game's own; DAT_100eef40 is
    // the base of an i32 array indexed by DAT_100adf38.
    unsafe {
        let idx = mech_trace::gr!("10083bdd", "100adf38", DAT_100adf38);
        let base = (&raw mut DAT_100eef40).wrapping_offset(idx as isize);
        let v = mech_trace::rd!("10083bdd", base, read_unaligned);
        FUN_1008276d(v, param_1, 2);
    }
}
