// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10114db0: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100086f7();
    fn FUN_1000a73a();
    fn FUN_1000898c();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000895c() {
    // SAFETY: the callees and the global are the game's own; the global holds
    // an address whose field at offset 6 is read as a 16-bit value.
    unsafe {
        FUN_100086f7();
        let v = ((DAT_10114db0.wrapping_add(6)) as *const i16).read_unaligned();
        if v == 0 {
            FUN_1000898c();
        } else {
            FUN_1000a73a();
        }
    }
}
