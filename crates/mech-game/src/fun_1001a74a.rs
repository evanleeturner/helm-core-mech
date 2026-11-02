// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001a74a(param_1: *mut u32, param_2: *mut u32) -> u32 {
    if param_2.is_null() || param_1.is_null() {
        return 0;
    }
    // SAFETY: both pointers are non-null and point at the caller's buffers of
    // at least seven and six u32 values; reads and writes follow the original.
    unsafe {
        if param_2.add(6).read_unaligned() == 0 {
            return 0;
        }
        param_1.write_unaligned(param_2.read_unaligned());
        param_1
            .add(1)
            .write_unaligned(param_2.add(1).read_unaligned());
        param_1
            .add(2)
            .write_unaligned(param_2.add(2).read_unaligned());
        param_1
            .add(3)
            .write_unaligned(param_2.add(3).read_unaligned());
        param_1
            .add(4)
            .write_unaligned(param_2.add(4).read_unaligned());
        param_1
            .add(5)
            .write_unaligned(param_2.add(5).read_unaligned());
    }
    1
}
