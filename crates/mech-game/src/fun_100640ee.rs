// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f322(a: i32, b: *mut u32, c: *mut u32, d: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100640ee(
    param_1: i32,
    param_2: *mut u32,
    param_3: *mut u32,
    param_4: *mut u32,
) {
    // SAFETY: param_1 is an address of the game's own object; its field at
    // offset 0x48 is read and passed to the game's callee with the caller's pointers.
    unsafe {
        let a = ((param_1 as usize).wrapping_add(0x48) as *const i32).read_unaligned();
        FUN_1002f322(a, param_2, param_3, param_4);
    }
}
