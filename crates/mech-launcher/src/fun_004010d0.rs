// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _memcpy(dst: *mut u8, src: *const u8, len: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_004010d0() {
    // SAFETY: the callee is the game's own; the addresses are the fixed
    // video memory region the original copies within.
    unsafe {
        _memcpy(0xb0000 as *mut u8, 0xb00a0 as *const u8, 0xf00);
    }
}
