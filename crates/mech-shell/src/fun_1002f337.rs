// SPDX-License-Identifier: MIT

use core::arch::asm;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002f337(param_1: *mut u32, param_2: *mut u32) -> u32 {
    let cs_value: u16;
    // SAFETY: reading the code segment register has no side effects; the
    // pointers are the caller's and are written once each, as in the original.
    unsafe {
        asm!("mov {0:x}, cs", out(reg) cs_value, options(nomem, nostack, preserves_flags));
        param_2.write_unaligned(cs_value as u32);
        param_1.write_unaligned(0x100206f0u32);
    }
    0xec2a
}
