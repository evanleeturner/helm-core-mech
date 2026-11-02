// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068026(param_1: i32) {
    let p = param_1 as usize as *mut i32;
    // SAFETY: param_1 is the address of the caller's struct; the field at
    // offset 0x30 is read and written as the original does.
    unsafe {
        let f = p.byte_add(0x30);
        f.write_unaligned(f.read_unaligned().wrapping_add(1));
        if f.read_unaligned() == 0xe {
            f.write_unaligned(0);
        }
    }
}
