// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7248: u32;
    static mut DAT_100a724c: u32;
    static mut DAT_100a7580: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005fb90() {
    // SAFETY: the globals are the game's own; the non-zero ones hold pointers
    // to objects whose function-pointer slots at 0x14 and 0x10 are called
    // exactly as the original does.
    unsafe {
        if DAT_100a724c != 0 {
            let f = ((DAT_100a724c.wrapping_add(0x14)) as *const u32).read_unaligned();
            let f: unsafe extern "stdcall" fn() = core::mem::transmute(f as usize);
            f();
        }
        if DAT_100a7248 != 0 {
            let f = ((DAT_100a7248.wrapping_add(0x10)) as *const u32).read_unaligned();
            let f: unsafe extern "stdcall" fn() = core::mem::transmute(f as usize);
            f();
        }
        DAT_100a7580 = 1;
    }
}
