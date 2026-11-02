// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a5160: [u8; 0];
    static mut DAT_100a5188: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10007440(param_1: *mut i32, param_2: *mut i32, param_3: i32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100082c4(param_1: i32, param_2: i32) {
    mech_trace::enter!("100082c4", param_1, param_2);
    if param_2 != 0 {
        let local_8: *mut i32 = if param_2 == -1 {
            core::ptr::null_mut()
        } else {
            // SAFETY: only the address of the game's own table is taken.
            unsafe {
                (&raw mut DAT_100a5160)
                    .cast::<u8>()
                    .wrapping_byte_offset(param_2.wrapping_mul(0xc) as isize)
                    .cast::<i32>()
            }
        };
        // SAFETY: the callee and the tables are the game's own; only the
        // address of the table is taken here.
        unsafe {
            let p = (&raw mut DAT_100a5188)
                .cast::<u8>()
                .wrapping_byte_offset(param_1.wrapping_mul(0xc) as isize)
                .cast::<i32>();
            mech_trace::cr!("100082c4", "10007440", FUN_10007440(p, local_8, -1));
        }
    }
}
