// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52c8: [i32; 2];
    static mut DAT_100eee64: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...);
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1006a2a5(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006a4bb(param_1: *mut i8, param_2: i32) {
    mech_trace::enter!("1006a4bb", param_1, param_2);
    let mut local_54: [i8; 80] = [0; 80];
    let buf = local_54.as_mut_ptr();
    // SAFETY: the callees and globals are the game's own; buf points at an
    // 80-byte local buffer and the format strings are NUL-terminated.
    unsafe {
        _sprintf(
            buf,
            b"%s  Res:%dx%d  Mem:%ld\0".as_ptr().cast(),
            param_1,
            mech_trace::gr!("1006a4bb", "101f52c8", DAT_101f52c8[0]),
            mech_trace::gr!("1006a4bb", "101f52c8", DAT_101f52c8[1]),
            param_2,
        );
        FUN_1005be24(buf, 0, 0x4f3, 0x50);
        if mech_trace::gr!("1006a4bb", "100eee64", DAT_100eee64) != 0 {
            _sprintf(
                buf,
                b"WarThink / MechWarrior II %s %ld\0".as_ptr().cast(),
                param_1,
                param_2,
            );
            FUN_1006a2a5(buf);
        }
    }
}
