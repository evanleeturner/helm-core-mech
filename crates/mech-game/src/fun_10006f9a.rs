// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa208: *mut i32;
    static mut DAT_100aa20c: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10006f9a(param_1: u32, param_2: u32, param_3: u32) {
    // SAFETY: the globals are the game's own; DAT_100aa208 points at an object
    // whose first field is a vtable pointer, with a stdcall method at offset 0x5c.
    unsafe {
        let obj = DAT_100aa208;
        let vtable = obj.read_unaligned() as *const u8;
        let f = vtable
            .byte_add(0x5c)
            .cast::<unsafe extern "stdcall" fn(*mut i32, u32, u32, i32, u32, u32)>()
            .read_unaligned();
        f(obj, DAT_100aa20c, param_1, 4, param_2, param_3);
    }
}
