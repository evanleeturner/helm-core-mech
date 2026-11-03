// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005ed70(this: *mut core::ffi::c_void, param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _SmackClose(param_1: i32);
    fn FUN_1006e790(param_1: *mut core::ffi::c_void);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005f900();
    fn MEM_free_lock(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_1005ef70(param_1: i32) {
    mech_trace::enter!("1005ef70", param_1);
    // SAFETY: param_1 is the address of the game's object; its fields are read
    // where the original reads them, and the callees are the game's own.
    unsafe {
        let base = param_1 as *const u8;
        FUN_1005ed70(
            mech_trace::rd!(
                "1005ef70",
                base.byte_add(0x13c).cast::<*mut core::ffi::c_void>(),
                read_unaligned
            ),
            param_1,
        );
        let a = mech_trace::rd!(
            "1005ef70",
            base.byte_add(0x100).cast::<i32>(),
            read_unaligned
        );
        if a != 0 {
            _SmackClose(a);
            FUN_1005f900();
        }
        let b = mech_trace::rd!(
            "1005ef70",
            base.byte_add(0x104).cast::<i32>(),
            read_unaligned
        );
        if b != 0 {
            MEM_free_lock(b);
        }
        let c = mech_trace::rd!(
            "1005ef70",
            base.byte_add(0x108).cast::<*mut core::ffi::c_void>(),
            read_unaligned
        );
        if !c.is_null() {
            FUN_1006e790(c);
        }
    }
}
