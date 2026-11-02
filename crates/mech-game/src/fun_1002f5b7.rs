// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008984b(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: u32,
    );
    fn FUN_1002f54b(param_1: *mut core::ffi::c_void, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f5b7(
    param_1: *mut core::ffi::c_void,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: i32,
    param_7: i32,
    param_8: i32,
) {
    let mut local_34: [i32; 12] = [0; 12];
    // SAFETY: param_1 points at the caller's object; the callees are the
    // game's own and local_34 is a valid scratch buffer for them.
    unsafe {
        let p = param_1.byte_add(0x74).cast::<u32>();
        p.write_unaligned(p.read_unaligned() | 1);
        FUN_1008984b(
            local_34.as_mut_ptr(),
            0,
            0,
            0,
            param_6.wrapping_neg(),
            param_7.wrapping_neg(),
            param_8.wrapping_neg(),
            param_5,
        );
        FUN_1002f54b(param_1, local_34.as_mut_ptr());
        FUN_1008984b(
            local_34.as_mut_ptr(),
            param_2,
            param_3,
            param_4,
            0,
            0,
            0,
            param_5,
        );
        FUN_1002f54b(param_1, local_34.as_mut_ptr());
        FUN_1008984b(
            local_34.as_mut_ptr(),
            0,
            0,
            0,
            param_6,
            param_7,
            param_8,
            param_5,
        );
        FUN_1002f54b(param_1, local_34.as_mut_ptr());
    }
}
