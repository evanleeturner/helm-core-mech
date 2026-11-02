// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c509(param_1: i32, param_2: u32) {
    // SAFETY: param_1 is an address of the game's own structure; the pointer
    // stored at offset 0x20 is read, and if non-null the value is written
    // through it, as the original does.
    unsafe {
        let p = ((param_1 as usize).wrapping_add(0x20) as *const *mut u32).read_unaligned();
        if !p.is_null() {
            p.write_unaligned(param_2);
        }
    }
}
