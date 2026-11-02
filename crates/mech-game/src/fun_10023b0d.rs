// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab384: u32;
    static mut DAT_1016acb4: i32;
    static mut DAT_1016bc60: [u8; 0];
    static mut DAT_1016cc60: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10023920(param_1: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10023b0d() -> *mut u32 {
    mech_trace::enter!("10023b0d");
    let mut local_8: *mut u32 = core::ptr::null_mut();
    // SAFETY: the globals and the callee are the game's own memory; the index
    // table and record array are read at the offsets the original uses.
    unsafe {
        if -1 < mech_trace::gr!("10023b0d", "1016acb4", DAT_1016acb4)
            && mech_trace::gr!("10023b0d", "100ab384", DAT_100ab384) != 0
        {
            let idx = mech_trace::rd!(
                "10023b0d",
                (&raw mut DAT_1016bc60)
                    .cast::<u8>()
                    .byte_add(
                        (mech_trace::gr!("10023b0d", "1016acb4", DAT_1016acb4) as usize)
                            .wrapping_mul(4)
                    )
                    .cast::<i32>(),
                read_unaligned
            );
            local_8 = (&raw mut DAT_1016cc60)
                .cast::<u8>()
                .byte_offset((idx.wrapping_mul(0x1fa)) as isize)
                .cast::<u32>();
            mech_trace::cr!("10023b0d", "10023920", FUN_10023920(local_8));
            mech_trace::gw!(
                "10023b0d",
                "1016acb4",
                DAT_1016acb4,
                mech_trace::gr!("10023b0d", "1016acb4", DAT_1016acb4).wrapping_sub(1)
            );
        }
    }
    mech_trace::rt!("10023b0d", local_8)
}
