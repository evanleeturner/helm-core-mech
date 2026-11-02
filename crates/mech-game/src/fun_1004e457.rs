// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0710: i32;
    static mut DAT_100b07dc: i32;
    static mut DAT_100aa1ac: i32;
    static mut DAT_10123250: i32;
    static mut DAT_10123254: i32;
    static mut DAT_10123258: i32;
    static mut DAT_10123268: i32;
    static mut DAT_10165922: i32;
    static mut DAT_100adf38: i32;
    static mut DAT_100b0800: u8;
    static mut DAT_100b0814: u8;
    static mut DAT_100b0824: u8;
    static mut DAT_100b083c: u8;
    static mut DAT_100b0844: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(param_1: *mut i8, param_2: *const i8) -> i32;
    fn FUN_1001028b(param_1: *mut u8) -> i32;
    fn FUN_1001399a();
    fn FUN_1007b9d2(param_1: i32, param_2: *mut i8) -> i32;
    fn FUN_10008351(param_1: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
#[allow(unused_assignments, unused_variables)]
pub unsafe extern "C" fn FUN_1004e457(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("1004e457", param_1, param_2);
    let mut local_74 = [0i8; 100];
    let mut local_10: i32 = 0;
    let mut local_c: *mut i8 = core::ptr::null_mut();
    let mut local_8: i32 = 0;

    // SAFETY: the globals and callees are the game's own memory and code;
    // offsets are computed as the original does and local buffers are valid.
    unsafe {
        if param_1 == mech_trace::gr!("1004e457", "100b0710", DAT_100b0710)
            && (mech_trace::gr!("1004e457", "100b07dc", DAT_100b07dc) == 0
                || mech_trace::gr!("1004e457", "100aa1ac", DAT_100aa1ac) == 0)
        {
            let off = param_1.wrapping_mul(0x3c0a) as isize;
            mech_trace::gw!("1004e457", "100b07dc", DAT_100b07dc, 1);
            if param_2 == 2 {
                mech_trace::cr!(
                    "1004e457",
                    "_sprintf",
                    _sprintf(
                        local_74.as_mut_ptr(),
                        (&raw mut DAT_100b0800).cast::<i8>() as *const i8,
                    )
                );
                local_10 = mech_trace::rd!(
                    "1004e457",
                    (&raw mut DAT_10123250)
                        .cast::<u8>()
                        .byte_offset(off)
                        .cast::<i32>(),
                    read_unaligned
                );
                local_8 = mech_trace::cr!(
                    "1004e457",
                    "1001028b",
                    FUN_1001028b((&raw mut DAT_10123258).cast::<u8>().byte_offset(off))
                );
                if mech_trace::gr!("1004e457", "100aa1ac", DAT_100aa1ac) != 0 {
                    mech_trace::gw!(
                        "1004e457",
                        "10165922",
                        DAT_10165922,
                        mech_trace::gr!("1004e457", "100adf38", DAT_100adf38)
                    );
                    FUN_1001399a();
                }
            } else if param_2 == 3 {
                mech_trace::cr!(
                    "1004e457",
                    "_sprintf",
                    _sprintf(
                        local_74.as_mut_ptr(),
                        (&raw mut DAT_100b0814).cast::<i8>() as *const i8,
                    )
                );
                local_10 = mech_trace::rd!(
                    "1004e457",
                    (&raw mut DAT_10123254)
                        .cast::<u8>()
                        .byte_offset(off)
                        .cast::<i32>(),
                    read_unaligned
                );
                local_8 = mech_trace::cr!(
                    "1004e457",
                    "1001028b",
                    FUN_1001028b((&raw mut DAT_10123268).cast::<u8>().byte_offset(off))
                );
            } else if param_2 == 4 {
                mech_trace::cr!(
                    "1004e457",
                    "_sprintf",
                    _sprintf(
                        local_74.as_mut_ptr(),
                        (&raw mut DAT_100b0824).cast::<i8>() as *const i8,
                    )
                );
                local_10 = mech_trace::cr!(
                    "1004e457",
                    "1007b9d2",
                    FUN_1007b9d2(10, (&raw mut DAT_100b083c).cast::<i8>())
                );
                local_8 =
                    mech_trace::cr!("1004e457", "1001028b", FUN_1001028b(&raw mut DAT_100b0844));
            }
            local_c = local_74.as_mut_ptr();
            FUN_10008351(&raw mut local_10);
        }
    }
    mech_trace::rt!("1004e457", 1)
}
