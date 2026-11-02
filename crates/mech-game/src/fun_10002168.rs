// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10218080: [i8; 0x100];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008a920(buffer: *mut i8, count: u32, format: *const i8, argptr: *mut i8) -> i32;
    fn FUN_10007342(param_1: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10002168(param_1: *mut i8) {
    // SAFETY: the callees and the buffer are the game's own; under cdecl on
    // i386 the variadic arguments sit on the stack directly after param_1,
    // so the address one pointer past it is the MSVC va_list.
    unsafe {
        let argptr = (&raw const param_1).add(1) as *mut i8;
        FUN_1008a920(
            (&raw mut DAT_10218080).cast::<i8>(),
            0x100,
            param_1 as *const i8,
            argptr,
        );
        FUN_10007342((&raw mut DAT_10218080).cast::<i8>());
    }
}
