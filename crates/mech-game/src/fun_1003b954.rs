// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10089cc9(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
    fn FUN_10089643(param_1: *mut i32, param_2: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003b954(param_1: *mut i32) {
    // local_34[9] followed by local_10, local_c, local_8 laid out contiguously on the stack.
    let mut local: [i32; 12] = [0; 12];
    // SAFETY: param_1 points at the caller's int array; the callees are the
    // game's own and receive pointers into the local frame and the caller's array.
    unsafe {
        FUN_10089cc9(
            local.as_mut_ptr(),
            param_1.add(2).read_unaligned(),
            param_1.add(1).read_unaligned(),
            param_1.read_unaligned(),
            param_1.add(5).read_unaligned(),
            param_1.add(3).read_unaligned(),
            param_1.add(4).read_unaligned(),
        );
        FUN_10089643(local.as_mut_ptr(), param_1.add(0x15));
        param_1.add(0x1e).write_unaligned(local[9]);
        param_1.add(0x1f).write_unaligned(local[10]);
        param_1.add(0x20).write_unaligned(local[11]);
    }
}
