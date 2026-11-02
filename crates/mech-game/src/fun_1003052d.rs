// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada4c: u32;
    static mut DAT_100ada50: i32;
    static mut DAT_100ada58: [u8; 1];
    static mut DAT_100ada5c: [u8; 1];
    static mut DAT_10168800: [u8; 1];
    static mut DAT_10168900: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10030433(param_1: i32, param_2: *mut i32, param_3: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003052d(param_1: i32) -> *mut u8 {
    mech_trace::enter!("1003052d", param_1);
    let mut local_8: *mut u8 = core::ptr::null_mut();
    // SAFETY: the globals and the callee are the game's own memory; the
    // index is range-checked against DAT_100ada50 before it is used.
    unsafe {
        if (mech_trace::gr!("1003052d", "100ada50", DAT_100ada50).wrapping_sub(1) as u32)
            != mech_trace::gr!("1003052d", "100ada4c", DAT_100ada4c)
            && -1 < param_1
            && param_1 < mech_trace::gr!("1003052d", "100ada50", DAT_100ada50)
        {
            local_8 = (&raw mut DAT_10168900)
                .cast::<u8>()
                .byte_add(param_1.wrapping_mul(0x34) as u32 as usize);
            mech_trace::gw!(
                "1003052d",
                "100ada4c",
                DAT_100ada4c,
                mech_trace::gr!("1003052d", "100ada4c", DAT_100ada4c).wrapping_add(1)
            );
            mech_trace::wr!(
                "1003052d",
                (&raw mut DAT_10168800)
                    .cast::<u8>()
                    .byte_add(
                        mech_trace::gr!("1003052d", "100ada4c", DAT_100ada4c).wrapping_mul(4)
                            as usize
                    )
                    .cast::<i32>(),
                write_unaligned,
                param_1
            );
            mech_trace::cr!(
                "1003052d",
                "10030433",
                FUN_10030433(
                    local_8 as usize as i32,
                    (&raw mut DAT_100ada58).cast::<i32>(),
                    (&raw mut DAT_100ada5c).cast::<i32>(),
                )
            );
        }
    }
    mech_trace::rt!("1003052d", local_8)
}
