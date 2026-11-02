// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40d0: [u8; 0];
    static mut DAT_100b3c4c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002ba7(
        param_1: i32,
        param_2: u32,
        param_3: u32,
        param_4: *mut core::ffi::c_void,
        param_5: i32,
        param_6: i32,
        param_7: u32,
        param_8: u32,
        param_9: i32,
        param_10: u16,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10003742(
    param_1: u32,
    param_2: i32,
    param_3: u32,
    param_4: i32,
    param_5: u16,
) {
    mech_trace::enter!("10003742", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: the globals and the callee are the game's own; the table at
    // DAT_100a40d0 is indexed by param_4 as the original does.
    unsafe {
        let v8 = mech_trace::rd!(
            "10003742",
            (&raw mut DAT_100a40d0)
                .cast::<u8>()
                .byte_add((param_4 as usize).wrapping_mul(4))
                .cast::<u32>(),
            read_unaligned
        );
        mech_trace::cr!(
            "10003742",
            "10002ba7",
            FUN_10002ba7(
                0,
                0,
                param_1,
                core::ptr::null_mut(),
                param_2,
                mech_trace::gr!("10003742", "100b3c4c", DAT_100b3c4c) as i32,
                param_3,
                v8,
                -1,
                param_5,
            )
        );
    }
}
