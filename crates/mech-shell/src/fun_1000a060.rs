// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d008: i8;
    static mut DAT_1008d00c: u8;
    static mut DAT_100a6738: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __strnicmp(_str1: *const i8, _str2: *const i8, _max_count: u32) -> i32;
    fn FUN_10036510();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a060(text: *mut i8) {
    // SAFETY: the globals and callees are the game's own; text points at the
    // caller's string. The thiscall callee takes `this` in ecx, pops its three
    // stack arguments itself, and is called with them pushed right to left.
    unsafe {
        let compare_result = __strnicmp(
            text.byte_add(5) as *const i8,
            &raw mut DAT_1008d008 as *const i8,
            3,
        );
        if compare_result == 0 {
            let this = DAT_100a6738;
            let p3 = &raw mut DAT_1008d00c as *mut i32;
            core::arch::asm!(
                "push {c}",
                "push 8",
                "push {t}",
                "call {f}",
                c = in(reg) p3,
                t = in(reg) text,
                f = sym FUN_10036510,
                in("ecx") this,
                clobber_abi("C"),
            );
        }
    }
}
