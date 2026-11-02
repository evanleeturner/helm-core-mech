// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab374: i32;
    static mut DAT_1016acb4: u32;
    static mut DAT_1016bc60: [u8; 0];
    static mut DAT_1016cc60: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100239a4(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10023a8c(param_1: i32) -> *mut i32 {
    mech_trace::enter!("10023a8c", param_1);
    let mut local_8: *mut i32 = core::ptr::null_mut();
    // SAFETY: the globals and the callee are the game's own memory; the
    // pointers are computed and written exactly where the original does.
    unsafe {
        if (mech_trace::gr!("10023a8c", "100ab374", DAT_100ab374).wrapping_sub(1) as u32)
            != mech_trace::gr!("10023a8c", "1016acb4", DAT_1016acb4)
            && -1 < param_1
            && param_1 < mech_trace::gr!("10023a8c", "100ab374", DAT_100ab374)
        {
            local_8 = (&raw mut DAT_1016cc60)
                .cast::<u8>()
                .byte_add(param_1.wrapping_mul(0x1fa) as usize)
                .cast::<i32>();
            mech_trace::gw!(
                "10023a8c",
                "1016acb4",
                DAT_1016acb4,
                mech_trace::gr!("10023a8c", "1016acb4", DAT_1016acb4).wrapping_add(1)
            );
            mech_trace::wr!(
                "10023a8c",
                (&raw mut DAT_1016bc60)
                    .cast::<u8>()
                    .byte_add(
                        mech_trace::gr!("10023a8c", "1016acb4", DAT_1016acb4).wrapping_mul(4)
                            as usize
                    )
                    .cast::<i32>(),
                write_unaligned,
                param_1
            );
            mech_trace::cr!("10023a8c", "100239a4", FUN_100239a4(local_8));
        }
    }
    mech_trace::rt!("10023a8c", local_8)
}
