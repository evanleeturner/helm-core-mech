// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d000(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b380(param_1: i32, param_2: i32, param_3: i32, param_4: i8) {
    let i_var1 = param_3.wrapping_mul(0x12);
    let a2 = i_var1.wrapping_add(0x760).wrapping_add(param_2);
    let a1 = i_var1.wrapping_add(0x760).wrapping_add(param_1);
    // SAFETY: the addresses are the caller's game structures, read and
    // written where the original does; the callee and global are the game's own.
    unsafe {
        let i_var2 = (a2 as *const i32)
            .read_unaligned()
            .wrapping_sub((a1 as *const i32).read_unaligned());
        FUN_1000d000(i_var2);
        if param_4 != 1 {
            (a1 as *mut i32).write_unaligned((a2 as *const i32).read_unaligned());
            if 0 < i_var2 {
                (param_1.wrapping_add(0x84c) as *mut u8).write(1);
                DAT_10083508.write(1);
            }
        }
    }
}
