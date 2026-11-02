// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10027502() {
    let mut local_8: i32 = 3;
    while local_8 < 0x19 {
        local_8 = ((local_8 as u32).wrapping_add(1)) as i32;
    }
}
