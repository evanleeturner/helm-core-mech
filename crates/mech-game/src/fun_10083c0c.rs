// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100adf38: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008276d(param_1: u32, param_2: i32, param_3: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10083c0c(param_1: i32) {
    mech_trace::enter!("10083c0c", param_1);
    // SAFETY: the globals and the callee are the game's own; the table is
    // indexed by the game's current index exactly as the original does.
    unsafe {
        let idx = mech_trace::gr!("10083c0c", "100adf38", DAT_100adf38);
        let p = (&raw mut DAT_100eef40).cast::<u32>();
        let v = mech_trace::rd!("10083c0c", p.offset(idx as isize), read_unaligned);
        FUN_1008276d(v, param_1, 0x20008);
    }
}
