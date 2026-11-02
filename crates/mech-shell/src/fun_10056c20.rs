// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10056c20(box_: *mut i32, param_2: i32, param_3: i32) {
    // SAFETY: box_ points at four consecutive i32 values owned by the caller.
    unsafe {
        let p1 = box_.byte_add(4);
        p1.write_unaligned(p1.read_unaligned().wrapping_add(param_2));
        let p3 = box_.byte_add(12);
        p3.write_unaligned(p3.read_unaligned().wrapping_add(param_2));
        box_.write_unaligned(box_.read_unaligned().wrapping_add(param_3));
        let p2 = box_.byte_add(8);
        p2.write_unaligned(p2.read_unaligned().wrapping_add(param_3));
    }
}
