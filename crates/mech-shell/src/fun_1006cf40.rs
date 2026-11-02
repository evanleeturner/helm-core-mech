// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_100aa780: i32;
    static mut _DAT_100aa7b8: i32;
    static mut _DAT_100aa7f0: i32;
    static mut _DAT_100aa828: i32;
    static mut PTR_DAT_100aa870: *mut u8;
    static mut PTR_DAT_100aa874: *mut u8;
    static mut DAT_10083508: *mut u8;
    static mut DAT_100aa6e8: i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1006cf90() -> i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063de0(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006cf40() {
    mech_trace::enter!("1006cf40");
    // SAFETY: the globals and callees are the game's own memory; the pointer
    // globals point at game structures read and written where the original does.
    unsafe {
        mech_trace::gw!("1006cf40", "100aa780", _DAT_100aa780, 1);
        mech_trace::gw!("1006cf40", "100aa7b8", _DAT_100aa7b8, 1);
        mech_trace::gw!("1006cf40", "100aa7f0", _DAT_100aa7f0, 1);
        mech_trace::gw!("1006cf40", "100aa828", _DAT_100aa828, 1);
        let i_var1 = mech_trace::cr!("1006cf40", "1006cf90", FUN_1006cf90());
        mech_trace::wr!(
            "1006cf40",
            mech_trace::gr!("1006cf40", "100aa870", PTR_DAT_100aa870)
                .byte_add(0x24)
                .cast::<i32>(),
            write_unaligned,
            i_var1
        );
        let unaligned_val = mech_trace::rd!(
            "1006cf40",
            mech_trace::gr!("1006cf40", "10083508", DAT_10083508)
                .byte_add(0x12a)
                .cast::<i32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1006cf40",
            mech_trace::gr!("1006cf40", "100aa874", PTR_DAT_100aa874)
                .byte_add(0x24)
                .cast::<i32>(),
            write_unaligned,
            unaligned_val.wrapping_sub(i_var1)
        );
        FUN_10063de0(&raw mut DAT_100aa6e8);
    }
}
