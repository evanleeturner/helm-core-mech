// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adaa8: u32;
    static mut DAT_100adaac: u32;
    static mut DAT_10168490: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10050a68(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003324c() -> u32 {
    // SAFETY: the globals and the callee are the game's own; the table at
    // DAT_10168490 is read at the index the original uses.
    unsafe {
        if DAT_100adaac < DAT_100adaa8 {
            let p = (&raw mut DAT_10168490)
                .byte_add(DAT_100adaac.wrapping_mul(4) as usize)
                .cast::<i32>();
            let local_c = FUN_10050a68(p.read_unaligned());
            DAT_100adaac = DAT_100adaac.wrapping_add(1);
            local_c
        } else {
            0
        }
    }
}
