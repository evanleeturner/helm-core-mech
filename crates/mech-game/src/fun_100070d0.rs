// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "FID_conflict__memcpy"]
    fn FID_conflict__memcpy(dst: *mut u8, src: *const u8, n: u32) -> *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100070d0() {
    // SAFETY: the callee is the game's own memcpy; the fixed addresses are
    // the game's own memory regions, copied exactly as the original does.
    unsafe {
        FID_conflict__memcpy(0xb0000 as *mut u8, 0xb00a0 as *const u8, 0xf00);
    }
}
