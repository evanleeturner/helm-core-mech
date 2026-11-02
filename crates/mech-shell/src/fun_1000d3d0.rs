// SPDX-License-Identifier: MIT

use core::arch::asm;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10050b20();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d3d0(
    text: *mut i8,
    width_measurer: *mut u8,
    max_width: i32,
) -> *mut i8 {
    let width: i32;
    // SAFETY: calls the game's thiscall function: this in ECX, text pushed on
    // the stack, and the callee pops it.
    unsafe {
        asm!(
            "push {t}",
            "call {f}",
            t = in(reg) text,
            f = sym FUN_10050b20,
            in("ecx") width_measurer,
            lateout("eax") width,
            clobber_abi("C"),
        );
    }
    if max_width < width {
        let mut i: usize = 0;
        // SAFETY: text points at a NUL-terminated string; reads stay within it.
        unsafe {
            let mut c = *text;
            while c != 0 && *text.add(i) != b' ' as i8 {
                i = i.wrapping_add(1);
                c = *text.add(i);
            }
            c = *text.add(i);
            while c != 0 && *text.add(i) == b' ' as i8 {
                i = i.wrapping_add(1);
                c = *text.add(i);
            }
            let mut result = text.add(i);
            if *text.add(i) == 0 {
                result = text;
            }
            return result;
        }
    }
    text
}
