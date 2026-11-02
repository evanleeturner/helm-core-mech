// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c599(param_1: i32) -> u16 {
    // SAFETY: param_1 is the address of the caller's object; the pointer at
    // offset 0x20 is checked for zero before the u16 at offset 0x14 is read.
    unsafe {
        let p = (param_1 as usize + 0x20) as *const i32;
        let inner = p.read_unaligned();
        if inner == 0 {
            0
        } else {
            ((inner as usize + 0x14) as *const u16).read_unaligned()
        }
    }
}
