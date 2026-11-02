// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a5c: u32;
    static mut s__100b39e8: [i8; 0];
    static mut s____d__ds_100b3bac: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006a09b() {
    mech_trace::enter!("1006a09b");
    let mut local_6c: [i8; 52] = [0; 52];
    let mut local_38: [i8; 52] = [0; 52];
    // SAFETY: the callee and the globals are the game's own; the local buffers
    // are the sprintf destinations, as in the original.
    unsafe {
        mech_trace::cr!(
            "1006a09b",
            "_sprintf",
            _sprintf(
                local_38.as_mut_ptr(),
                (&raw const s____d__ds_100b3bac).cast::<i8>(),
                0x32i32,
                0x32i32,
            )
        );
        mech_trace::cr!(
            "1006a09b",
            "_sprintf",
            _sprintf(
                local_6c.as_mut_ptr(),
                local_38.as_ptr(),
                (&raw const s__100b39e8).cast::<i8>(),
            )
        );
        mech_trace::gw!("1006a09b", "100b3a5c", DAT_100b3a5c, 0);
    }
}
