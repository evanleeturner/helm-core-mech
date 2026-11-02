// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100480d9(param_1: *mut i8) -> *mut u32;
    fn FUN_10047b36(
        param_1: *mut i8,
        param_2: *mut u32,
        param_3: *mut i32,
        param_4: *mut i32,
    ) -> i32;
    fn __close(_FileHandle: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10047c3f(param_1: *mut i8, param_2: *mut i32) -> i32 {
    let mut local_10: i32 = 0;
    let mut local_8: u32 = 0;
    // SAFETY: the callees are the game's own; param_2 is the caller's valid
    // out pointer, and the locals outlive the calls that receive their addresses.
    unsafe {
        param_2.write_unaligned(0);
        let p_var1 = FUN_100480d9(param_1) as *mut i8;
        let local_c = FUN_10047b36(
            p_var1,
            &raw mut local_8,
            &raw mut local_10,
            core::ptr::null_mut(),
        );
        if local_c != -1 {
            __close(local_c);
            param_2.write_unaligned(local_10);
        }
        local_c
    }
}
