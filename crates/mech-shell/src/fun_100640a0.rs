// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c5940: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __vsnprintf(dst: *mut i8, size: i32, fmt: *mut i8, args: *mut i8) -> i32;
    fn FUN_1004f2c0(format: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100640a0(mut format: *mut i8) {
    mech_trace::enter!("100640a0", format);
    // SAFETY: the callees and the buffer are the game's own; the variadic
    // arguments follow `format` on the cdecl stack, so the va_list is the
    // address just past the parameter slot.
    unsafe {
        let args = (&raw mut format).add(1).cast::<i8>();
        mech_trace::cr!(
            "100640a0",
            "__vsnprintf",
            __vsnprintf((&raw mut DAT_100c5940).cast::<i8>(), 0x100, format, args)
        );
        FUN_1004f2c0((&raw mut DAT_100c5940).cast::<i8>());
    }
}
