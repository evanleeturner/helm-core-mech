// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3bd4: [i8; 1];
    static mut DAT_100b3a70: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(param_1: *mut i8, param_2: *mut i8, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a133() {
    let mut local_20: [i8; 28] = [0; 28];
    // SAFETY: the callee and the globals are the game's own; local_20 is a
    // local buffer passed to sprintf as the original does.
    unsafe {
        _sprintf(
            local_20.as_mut_ptr(),
            (&raw mut DAT_100b3bd4).cast::<i8>(),
            0x19,
            0x19,
        );
        DAT_100b3a70 = 0;
    }
}
