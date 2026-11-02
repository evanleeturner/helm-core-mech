// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bd4: u8;
    static mut DAT_100a9678: u8;
    static mut DAT_100af0a8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003eb02(param_1: u32, param_2: *mut i32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006346b() {
    let mut local_8: i32 = 0;
    // SAFETY: the globals and the callee are the game's own; the table
    // addresses are read at the original's byte offsets.
    unsafe {
        while local_8 < 0x42 {
            let v = (&raw mut DAT_100a7bd4)
                .byte_add(local_8.wrapping_mul(0x68) as usize)
                .cast::<i32>()
                .read_unaligned();
            if 0 < v {
                FUN_1003eb02(v as u32, DAT_100af0a8 as *mut i32);
            }
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = 0;
        while local_8 < 0x24 {
            let v = (&raw mut DAT_100a9678)
                .byte_add(local_8.wrapping_mul(0x2c) as usize)
                .cast::<i32>()
                .read_unaligned();
            if 0 < v {
                FUN_1003eb02(v as u32, DAT_100af0a8 as *mut i32);
            }
            local_8 = local_8.wrapping_add(1);
        }
    }
}
