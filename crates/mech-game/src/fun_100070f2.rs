// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100070f2() {
    let mut local_8: *mut u32 = 0xb0f00 as *mut u32;
    while (local_8 as i32) < 0xb0fa0 {
        // SAFETY: the address range is the game's own memory, masked in place
        // exactly as the original does.
        unsafe {
            let v = local_8.read_unaligned();
            local_8.write_unaligned(v & 0xff00ff00);
        }
        local_8 = local_8.wrapping_add(1);
    }
}
