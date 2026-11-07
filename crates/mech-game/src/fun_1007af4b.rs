// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10076068: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007af4b(param_1: *mut u32, param_2: *mut u32) -> u32 {
    mech_trace::enter!("1007af4b", param_1, param_2);
    let in_cs: u16;
    // SAFETY: reading the CS segment register has no side effects.
    unsafe {
        asm!("mov {0:x}, cs", out(reg) in_cs, options(nomem, nostack, preserves_flags));
        mech_trace::log::debug!(
            f = "1007af4b", b = "reg", r = "cs", v:% = mech_trace::__shown!(in_cs); "asm.out"
        );
    }
    // SAFETY: param_1 and param_2 point at the caller's writable u32 slots;
    // the global's address is the game's own.
    unsafe {
        mech_trace::wr!("1007af4b", param_2, write_unaligned, u32::from(in_cs));
        mech_trace::wr!(
            "1007af4b",
            param_1,
            write_unaligned,
            (&raw mut DAT_10076068) as usize as u32
        );
    }
    mech_trace::rt!("1007af4b", 0x4ec6)
}
