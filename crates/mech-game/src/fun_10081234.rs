// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3c9c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081234() -> u32 {
    // SAFETY: the global is the game's own; its value is an address of a
    // structure whose field at offset 0x20 is read.
    unsafe { ((DAT_100b3c9c.wrapping_add(0x20)) as *const u32).read_unaligned() }
}
