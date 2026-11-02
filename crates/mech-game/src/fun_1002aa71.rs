// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_1016a6a0: i32;
    static mut DAT_1016a6a4: *mut *mut u16;
    static mut DAT_100af17c: i32;
    static mut PTR_DAT_100af0f8: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1002aa71() -> i32 {
    // SAFETY: the globals and the callee are the game's own; the table at
    // DAT_1016a6a4 holds at least nine pointer slots, written in order.
    unsafe {
        _DAT_1016a6a0 = 0;
        let mut local_8: i32 = 0;
        while local_8 < 9 {
            let p_var1 = FUN_1008668b(DAT_100af17c, local_8.wrapping_add(1), PTR_DAT_100af0f8);
            DAT_1016a6a4
                .offset(local_8 as isize)
                .write_unaligned(p_var1);
            local_8 = local_8.wrapping_add(1);
        }
    }
    1
}
