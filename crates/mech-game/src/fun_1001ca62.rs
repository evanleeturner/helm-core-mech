// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001ca62(param_1: i32, param_2: u16) {
    // SAFETY: param_1 is an address of a caller-owned object; the u16 is
    // written at offset 0x14 as the original does.
    unsafe {
        (param_1 as usize as *mut u8)
            .byte_add(0x14)
            .cast::<u16>()
            .write_unaligned(param_2);
    }
}
