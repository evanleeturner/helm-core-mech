// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fc82(param_1: i32, param_2: *mut u32) {
    let mut src = param_2;
    let mut dst = param_1.wrapping_add(0xc) as *mut u32;
    let mut count: i32 = 0xc;
    while count != 0 {
        // SAFETY: the caller supplies a destination at param_1 + 0xc and a
        // source of twelve dwords; each is read and written in order.
        unsafe {
            dst.write_unaligned(src.read_unaligned());
            src = src.add(1);
            dst = dst.add(1);
        }
        count -= 1;
    }
}
