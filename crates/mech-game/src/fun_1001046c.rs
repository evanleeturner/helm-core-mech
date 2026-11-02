// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101ec450: [u8; 0];
    static mut DAT_100a9df0: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100104bb(param_1: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001046c() {
    mech_trace::enter!("1001046c");
    let mut local_c: i32 = 0;
    while local_c < 0x60 {
        // SAFETY: the callee and the globals are the game's own; the table
        // at DAT_101ec450 holds 0x60 entries of 8 bytes each.
        unsafe {
            FUN_100104bb(
                (&raw mut DAT_101ec450)
                    .cast::<u8>()
                    .byte_add((local_c as usize).wrapping_mul(8))
                    .cast::<u32>(),
            );
        }
        local_c = local_c.wrapping_add(1);
    }
    // SAFETY: DAT_100a9df0 is the game's own global.
    unsafe {
        mech_trace::gw!("1001046c", "100a9df0", DAT_100a9df0, 0);
    }
}
