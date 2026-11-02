// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fbff(param_1: *mut i32) -> *mut i32 {
    let mut p = param_1;
    // SAFETY: param_1 points at a chain of linked nodes whose first word is
    // the next pointer, terminated by a zero word; each is read in turn.
    unsafe {
        while p.read_unaligned() != 0 {
            p = p.read_unaligned() as usize as *mut i32;
        }
    }
    p
}
