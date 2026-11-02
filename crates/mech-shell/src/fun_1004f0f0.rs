// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__memcpy(param_1: *mut u8, param_2: *const u8, param_3: u32) -> *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004f0f0() {
    mech_trace::enter!("1004f0f0");
    // SAFETY: the callee is the game's own memcpy; the fixed addresses are
    // the original's video memory regions, copied as the original does.
    unsafe {
        mech_trace::cr!(
            "1004f0f0",
            "FID_conflict__memcpy",
            FID_conflict__memcpy(0xb0000 as *mut u8, 0xb00a0 as *const u8, 0xf00)
        );
    }
}
