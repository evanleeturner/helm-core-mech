// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab238: u32;
    static mut DAT_100b9a18: u32;
    static mut DAT_101e86b0: [i8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001d916(param_1: u32) {
    // SAFETY: the globals are the game's own memory; DAT_100b9a18 holds the
    // address of a table whose slot at offset 0x18 is a cdecl function pointer.
    unsafe {
        let u_var1 = DAT_100ab238;
        DAT_100ab238 = param_1;
        let f = ((DAT_100b9a18.wrapping_add(0x18)) as *const u32).read_unaligned();
        let f: unsafe extern "C" fn(*mut core::ffi::c_void) = core::mem::transmute(f as usize);
        f((&raw mut DAT_101e86b0).cast());
        DAT_100ab238 = u_var1;
    }
}
