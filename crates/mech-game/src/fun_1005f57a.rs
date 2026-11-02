// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3530: u8;
    static mut DAT_100af0ac: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003eb02(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005f57a() {
    let mut local_8: i32 = 0;
    // SAFETY: the table and the pointer global are the game's own memory;
    // the table is read until its -1 terminator, as the original does.
    unsafe {
        loop {
            let v = (&raw mut DAT_100b3530)
                .byte_add(local_8.wrapping_mul(4) as isize as usize)
                .cast::<i32>()
                .read_unaligned();
            if v == -1 {
                break;
            }
            let v2 = (&raw mut DAT_100b3530)
                .byte_add(local_8.wrapping_mul(4) as isize as usize)
                .cast::<u32>()
                .read_unaligned();
            FUN_1003eb02(v2, DAT_100af0ac as *mut i32);
            local_8 = local_8.wrapping_add(1);
        }
    }
}
