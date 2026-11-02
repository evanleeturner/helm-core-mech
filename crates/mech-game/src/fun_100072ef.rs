// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a42f0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __vsnprintf(param_1: *mut i8, param_2: u32, param_3: *const i8, param_4: *mut i8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100072ef(mut format: *mut i8) {
    mech_trace::enter!("100072ef", format);
    // SAFETY: the buffer and callee are the game's own; the va_list is the
    // address of the argument slot just after `format` on the cdecl stack.
    unsafe {
        let va = (&raw mut format).byte_add(4).cast::<i8>();
        mech_trace::cr!(
            "100072ef",
            "__vsnprintf",
            __vsnprintf(&raw mut DAT_100a42f0, 0x50, format, va)
        );
    }
}
