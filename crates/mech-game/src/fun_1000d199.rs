// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7584: i32;
    static mut DAT_101f64a4: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d875(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000d199() {
    let mut local_8: i32 = 0;
    // SAFETY: the globals and the callee are the game's own; the array is
    // read at the same offsets the original uses.
    unsafe {
        while local_8 < DAT_100a7584 {
            let v = (&raw mut DAT_101f64a4)
                .cast::<u8>()
                .byte_add(local_8.wrapping_mul(0x7c) as usize)
                .cast::<i32>()
                .read_unaligned();
            FUN_1000d875(local_8, v);
            local_8 = local_8.wrapping_add(1);
        }
    }
}
