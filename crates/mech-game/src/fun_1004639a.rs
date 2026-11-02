// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b04b0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn mciSendCommandA(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut core::ffi::c_void,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004639a() {
    mech_trace::enter!("1004639a");
    let mut resume_params: [u8; 4] = [0; 4];
    // SAFETY: the global and mciSendCommandA are the game's own; the params
    // buffer is a local that outlives the call.
    unsafe {
        mech_trace::cr!(
            "1004639a",
            "mciSendCommandA",
            mciSendCommandA(
                mech_trace::gr!("1004639a", "100b04b0", DAT_100b04b0),
                0x855,
                0,
                resume_params.as_mut_ptr().cast::<core::ffi::c_void>(),
            )
        );
    }
}
