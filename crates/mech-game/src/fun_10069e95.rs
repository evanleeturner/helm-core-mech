// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3a50: u32;
    static mut s__100b39e8: [i8; 0];
    static mut s____d__ds_100b3b30: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069e95() {
    mech_trace::enter!("10069e95");
    let mut local_44: [i8; 32] = [0; 32];
    let mut local_24: [i8; 32] = [0; 32];
    // SAFETY: the globals and _sprintf are the game's own; the local buffers
    // are valid for the writes the original performs.
    unsafe {
        mech_trace::cr!(
            "10069e95",
            "_sprintf",
            _sprintf(
                local_24.as_mut_ptr(),
                (&raw const s____d__ds_100b3b30).cast::<i8>(),
                0x1e_i32,
                0x1e_i32,
            )
        );
        mech_trace::cr!(
            "10069e95",
            "_sprintf",
            _sprintf(
                local_44.as_mut_ptr(),
                local_24.as_ptr(),
                (&raw const s__100b39e8).cast::<i8>(),
            )
        );
        mech_trace::gw!("10069e95", "100b3a50", DAT_100b3a50, 0);
    }
}
