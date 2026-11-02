// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_10092138: i32;
    static mut _DAT_10092170: i32;
    static mut _DAT_100921a8: i32;
    static mut _DAT_100921e0: i32;
    static mut DAT_10092000: i32;
    static mut DAT_10092004: i32;
    static mut DAT_10092040: *mut u8;
    static mut DAT_10092044: *mut u8;
    static mut DAT_10083508: *mut u8;
    static mut DAT_100920a0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fea0(param_1: i32, param_2: i32) -> i32;
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10030090() {
    mech_trace::enter!("10030090");
    // SAFETY: the globals and callees are the game's own; the pointers held
    // in the globals point at live game structures read and written at the
    // same offsets as the original.
    unsafe {
        mech_trace::gw!("10030090", "_DAT_10092138", _DAT_10092138, 1);
        mech_trace::gw!("10030090", "_DAT_10092170", _DAT_10092170, 1);
        mech_trace::gw!("10030090", "_DAT_100921a8", _DAT_100921a8, 1);
        mech_trace::gw!("10030090", "_DAT_100921e0", _DAT_100921e0, 1);
        let i_var1 = mech_trace::cr!(
            "10030090",
            "1002fea0",
            FUN_1002fea0(
                mech_trace::gr!("10030090", "10092000", DAT_10092000),
                mech_trace::gr!("10030090", "10092004", DAT_10092004)
            )
        );
        mech_trace::wr!(
            "10030090",
            mech_trace::gr!("10030090", "10092040", DAT_10092040)
                .byte_add(0x24)
                .cast::<i32>(),
            write_unaligned,
            i_var1
        );
        let unaligned_val = mech_trace::rd!(
            "10030090",
            mech_trace::gr!("10030090", "10083508", DAT_10083508)
                .byte_add(0x12a)
                .cast::<i32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "10030090",
            mech_trace::gr!("10030090", "10092044", DAT_10092044)
                .byte_add(0x24)
                .cast::<i32>(),
            write_unaligned,
            unaligned_val.wrapping_sub(i_var1)
        );
        FUN_10063de0(&raw mut DAT_100920a0);
    }
}
