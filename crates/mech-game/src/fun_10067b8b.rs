// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067b8b(
    param_1: *mut u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u32,
) -> bool {
    let _ = param_4;
    let _ = param_5;
    // SAFETY: param_1 points at the caller's record of at least five u32s;
    // __ftol and the global are the game's own.
    unsafe {
        param_1.add(1).write_unaligned(param_2);
        param_1.add(2).write_unaligned(param_3);
        let l_var1 = __ftol();
        param_1.add(3).write_unaligned(l_var1 as i32 as u32);
        param_1.write_unaligned(DAT_100b38fc);
        param_1.add(4).write_unaligned(param_6);
        0 < (param_1.add(3).read_unaligned() as i32)
    }
}
