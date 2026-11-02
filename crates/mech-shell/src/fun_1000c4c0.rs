// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10050b20();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c4c0(
    param_1: *mut u8,
    center_x_likely: i32,
    text: *mut i8,
) -> i32 {
    mech_trace::enter!("1000c4c0", param_1, center_x_likely, text);
    let text_width_likely: i32;
    // SAFETY: FUN_10050b20 is the game's __thiscall function: `this` goes in
    // ECX, the single argument is pushed, and the callee pops it. The result
    // comes back in EAX.
    unsafe {
        asm!(
            "push {text}",
            "call {f}",
            f = in(reg) FUN_10050b20 as usize,
            text = in(reg) text,
            inlateout("ecx") param_1 => _,
            lateout("eax") text_width_likely,
            clobber_abi("C"),
        );
    }
    mech_trace::rt!(
        "1000c4c0",
        center_x_likely.wrapping_sub(text_width_likely / 2)
    )
}
