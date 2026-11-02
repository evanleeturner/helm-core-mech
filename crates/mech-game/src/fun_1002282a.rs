// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad6bc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100814bb(param_1: i32, param_2: i32) -> i32;
    fn FUN_100228e4(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10022aaa() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002282a(param_1: u32, param_2: i32) -> u32 {
    let _ = param_1;
    // SAFETY: the callees and the global are the game's own; param_2 points at
    // the caller's unit record, and the allocated block is written where the
    // original does.
    unsafe {
        (param_2.wrapping_add(0x20) as *mut u32).write_unaligned(0);
        let i_var1 = FUN_10022aaa();
        let pi_var2 = FUN_100814bb(i_var1 as i32, DAT_100ad6bc as i32) as *mut i32;
        if pi_var2.is_null() {
            return 0;
        }
        pi_var2.write_unaligned(param_2);
        let mut local_c: i32 = 0;
        while local_c < 8 {
            pi_var2
                .byte_add((local_c.wrapping_add(0xe6) as usize).wrapping_mul(4))
                .write_unaligned(0);
            local_c = local_c.wrapping_add(1);
        }
        (param_2.wrapping_add(0x20) as *mut i32).write_unaligned(pi_var2 as usize as i32);
        (param_2.wrapping_add(0x24) as *mut u32).write_unaligned(0x4d3);
        FUN_100228e4(pi_var2 as usize as i32);
        1
    }
}
