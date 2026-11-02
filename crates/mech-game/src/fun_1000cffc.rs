// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7598: u32;
}

#[unsafe(naked)]
#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000cffc() {
    core::arch::naked_asm!(
        "lea eax, [esp + 4]",
        "push ebx",
        "mov edx, offset {d}",
        "mov ecx, 9",
        ".Lcopy_loop:",
        "mov ebx, [eax]",
        "mov [edx], ebx",
        "add eax, 4",
        "add edx, 4",
        "dec ecx",
        "jnz .Lcopy_loop",
        "pop ebx",
        "ret",
        d = sym DAT_100a7598,
    )
}
