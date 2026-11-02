// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a18: u32;
    static mut DAT_100b9a1c: u32;
    static mut DAT_100b9d50: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081cb8() {
    // SAFETY: the globals are the game's own memory; the non-zero values are
    // addresses of objects whose function pointers sit at the offsets read.
    unsafe {
        if DAT_100b9a1c != 0 {
            let f = ((DAT_100b9a1c.wrapping_add(0x14)) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn() = core::mem::transmute(f as usize);
            f();
        }
        if DAT_100b9a18 != 0 {
            let f = ((DAT_100b9a18.wrapping_add(0x10)) as *const u32).read_unaligned();
            let f: unsafe extern "C" fn() = core::mem::transmute(f as usize);
            f();
        }
        DAT_100b9d50 = 1;
    }
}
