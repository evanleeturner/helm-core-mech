// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eee64: i32;
    static mut DAT_100b3a4c: i32;
    static mut DAT_100c7fbc: i32;
    static mut DAT_100c7fc4: i32;
    static mut DAT_100c7fb8: i32;
    static mut DAT_100c7fc0: i32;
    static mut DAT_10160c00: i32;
    static mut DAT_100af09c: i32;
    static mut DAT_100b3a50: i32;
    static mut DAT_100a7b80: i32;

    static mut s_Objects___d_Polygons___d_Vertice_100b3ad8: [i8; 1];
    static mut s__4_4d_100b3b10: [i8; 1];
    static mut s__4_4d_100b3b18: [i8; 1];
    static mut s__4_4d_100b3b20: [i8; 1];
    static mut s__4_4d_100b3b28: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1003cc75();
    fn FUN_10069ef1();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001d37d(param_1: i32, param_2: *mut c_void);
    fn _sprintf(param_1: *mut i8, param_2: *const i8, ...) -> i32;
    fn FUN_1005be24(param_1: *mut i8, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069cb4() {
    mech_trace::enter!("10069cb4");
    let mut local_54: [i8; 80] = [0; 80];
    let mut local_5c: i32 = 0;
    // SAFETY: the globals and callees are the game's own; local_54 is a local
    // buffer passed to the original's sprintf calls; the table read mirrors the
    // C's int-pointer arithmetic (index * 4 ints, i.e. * 16 bytes).
    unsafe {
        if mech_trace::gr!("10069cb4", "100eee64", DAT_100eee64) != 0
            || mech_trace::gr!("10069cb4", "100b3a4c", DAT_100b3a4c) != 0
        {
            mech_trace::gw!("10069cb4", "100c7fbc", DAT_100c7fbc, 0);
            mech_trace::gw!("10069cb4", "100c7fc4", DAT_100c7fc4, 0);
            mech_trace::gw!("10069cb4", "100c7fb8", DAT_100c7fb8, 0);
            mech_trace::gw!("10069cb4", "100c7fc0", DAT_100c7fc0, 0);
            FUN_1003cc75();
            let mut local_58 = mech_trace::gr!("10069cb4", "10160c00", DAT_10160c00);
            while local_58 != 0 {
                FUN_1001d37d(local_58, FUN_10069ef1 as usize as *mut c_void);
                local_5c = local_5c.wrapping_add(1);
                local_58 = mech_trace::rd!(
                    "10069cb4",
                    (&raw mut DAT_10160c00)
                        .cast::<u8>()
                        .byte_offset(local_5c.wrapping_mul(16) as isize)
                        .cast::<i32>(),
                    read_unaligned
                );
            }
            FUN_1001d37d(
                mech_trace::gr!("10069cb4", "100af09c", DAT_100af09c),
                FUN_10069ef1 as usize as *mut c_void,
            );
            if mech_trace::gr!("10069cb4", "100eee64", DAT_100eee64) != 0 {
                if mech_trace::gr!("10069cb4", "100b3a50", DAT_100b3a50) == 0 {
                    if mech_trace::gr!("10069cb4", "100b3a4c", DAT_100b3a4c) != 0 {
                        mech_trace::cr!(
                            "10069cb4",
                            "_sprintf",
                            _sprintf(
                                local_54.as_mut_ptr(),
                                (&raw const s_Objects___d_Polygons___d_Vertice_100b3ad8)
                                    .cast::<i8>(),
                                mech_trace::gr!("10069cb4", "100c7fc0", DAT_100c7fc0),
                                mech_trace::gr!("10069cb4", "100c7fc4", DAT_100c7fc4),
                                mech_trace::gr!("10069cb4", "100c7fb8", DAT_100c7fb8),
                                mech_trace::gr!("10069cb4", "100a7b80", DAT_100a7b80),
                            )
                        );
                        FUN_1005be24(local_54.as_mut_ptr(), 1, 0xb5, 0x50);
                    }
                    mech_trace::gw!("10069cb4", "100b3a50", DAT_100b3a50, 1);
                }
                if mech_trace::gr!("10069cb4", "100eee64", DAT_100eee64) != 0 {
                    mech_trace::cr!(
                        "10069cb4",
                        "_sprintf",
                        _sprintf(
                            local_54.as_mut_ptr(),
                            (&raw const s__4_4d_100b3b10).cast::<i8>(),
                            mech_trace::gr!("10069cb4", "100c7fc0", DAT_100c7fc0),
                        )
                    );
                    mech_trace::cr!(
                        "10069cb4",
                        "_sprintf",
                        _sprintf(
                            local_54.as_mut_ptr(),
                            (&raw const s__4_4d_100b3b18).cast::<i8>(),
                            mech_trace::gr!("10069cb4", "100c7fb8", DAT_100c7fb8),
                        )
                    );
                    mech_trace::cr!(
                        "10069cb4",
                        "_sprintf",
                        _sprintf(
                            local_54.as_mut_ptr(),
                            (&raw const s__4_4d_100b3b20).cast::<i8>(),
                            mech_trace::gr!("10069cb4", "100c7fc4", DAT_100c7fc4),
                        )
                    );
                    mech_trace::cr!(
                        "10069cb4",
                        "_sprintf",
                        _sprintf(
                            local_54.as_mut_ptr(),
                            (&raw const s__4_4d_100b3b28).cast::<i8>(),
                            mech_trace::gr!("10069cb4", "100c7fbc", DAT_100c7fbc),
                        )
                    );
                }
            }
        }
    }
}
