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
    // SAFETY: the globals and callees are the game's own memory; the pointer
    // globals point at game structures read and written where the original does.
    unsafe {
        _DAT_100aa780 = 1;
        _DAT_100aa7b8 = 1;
        _DAT_100aa7f0 = 1;
        _DAT_100aa828 = 1;
        let i_var1 = FUN_1006cf90();
        PTR_DAT_100aa870
            .byte_add(0x24)
            .cast::<i32>()
            .write_unaligned(i_var1);
        let unaligned_val = DAT_10083508.byte_add(0x12a).cast::<i32>().read_unaligned();
        PTR_DAT_100aa874
            .byte_add(0x24)
            .cast::<i32>()
            .write_unaligned(unaligned_val.wrapping_sub(i_var1));
        FUN_10063de0(&raw mut DAT_100aa6e8);
    }
}
