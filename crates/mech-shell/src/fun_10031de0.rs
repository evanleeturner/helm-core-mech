// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_10092020: u8;
    static mut DAT_10092024: u8;
    static mut DAT_10092618: [u8; 0];
    static mut DAT_10092dc0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063ea0(param_1: *mut i32) -> u8;
    fn FUN_10063d30(param_1: *mut i32) -> u8;
    fn FUN_10031e40(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031de0(param_1: i32) {
    mech_trace::enter!("10031de0", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 and the
    // pointer held in DAT_10083508 point at the game's live objects.
    unsafe {
        mech_trace::gw!("10031de0", "10092020", DAT_10092020, 0);
        mech_trace::gw!("10031de0", "10092024", DAT_10092024, 0);
        mech_trace::cr!(
            "10031de0",
            "10063ea0",
            FUN_10063ea0((&raw mut DAT_10092618).cast::<i32>())
        );
        mech_trace::cr!(
            "10031de0",
            "10063ea0",
            FUN_10063ea0((&raw mut DAT_10092dc0).cast::<i32>())
        );
        mech_trace::cr!(
            "10031de0",
            "10063d30",
            FUN_10063d30((&raw mut DAT_10092dc0).cast::<i32>())
        );
        let u_var1 = mech_trace::cr!("10031de0", "10031e40", FUN_10031e40(param_1));
        if (u_var1 as u8) == 1
            && mech_trace::rd!(
                "10031de0",
                ((mech_trace::gr!("10031de0", "10083508", DAT_10083508).wrapping_add(0x23bb))
                    as *const i32),
                read_unaligned
            ) == 1
        {
            let p = (param_1.wrapping_add(0x28)) as *mut i32;
            mech_trace::wr!(
                "10031de0",
                p,
                write_unaligned,
                mech_trace::rd!("10031de0", p, read_unaligned).wrapping_sub(1)
            );
        }
    }
}
