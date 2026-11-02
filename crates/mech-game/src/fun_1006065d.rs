// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100af17c: u32;
    static mut DAT_100b36d4: u32;
    static mut DAT_100b36e4: u32;
    static mut DAT_100b36e8: u32;
    static mut DAT_100b36ec: u32;
    static mut DAT_100b36f0: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100c7e40: *mut u16;
    static mut DAT_100f3120: [u8; 0];
    static mut PTR_DAT_100af0c8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10071905(
        param_1: *mut u32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: u32,
    ) -> u32;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, p: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006065d(param_1: i32, param_2: u32, param_3: i32) -> u32 {
    mech_trace::enter!("1006065d", param_1, param_2, param_3);
    let mut param_2 = param_2;
    // SAFETY: all globals and callees are the game's own memory and code;
    // the table at DAT_100f3120 is indexed as the original does.
    unsafe {
        let mut local_14: *mut u16 = core::ptr::null_mut();
        let mut local_c: i32 = -1;
        let mut local_8: u32 = 0;
        let tbl = (&raw mut DAT_100f3120).cast::<u8>();

        if mech_trace::gr!("1006065d", "100b36f0", DAT_100b36f0) < 1 || param_3 != 0 {
            if mech_trace::gr!("1006065d", "100b36f0", DAT_100b36f0) < 1 {
                local_c = mech_trace::gr!("1006065d", "100b36d4", DAT_100b36d4) as i32;
                let v = mech_trace::rd!(
                    "1006065d",
                    tbl.byte_offset(
                        (mech_trace::gr!("1006065d", "100b36d4", DAT_100b36d4) as i32)
                            .wrapping_mul(4) as isize
                    )
                    .cast::<u32>(),
                    read_unaligned
                );
                local_14 = mech_trace::cr!(
                    "1006065d",
                    "1008668b",
                    FUN_1008668b(
                        mech_trace::gr!("1006065d", "100af17c", DAT_100af17c) as i32,
                        v,
                        mech_trace::gr!("1006065d", "PTR_DAT_100af0c8", PTR_DAT_100af0c8) as usize
                            as *mut i32,
                    )
                );
            }
        } else {
            local_14 = mech_trace::gr!("1006065d", "100c7e40", DAT_100c7e40);
        }
        if !local_14.is_null() {
            let v = mech_trace::rd!(
                "1006065d",
                tbl.byte_offset(param_1.wrapping_mul(4) as isize)
                    .cast::<u32>(),
                read_unaligned
            );
            let puvar1 = mech_trace::cr!(
                "1006065d",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("1006065d", "100af17c", DAT_100af17c) as i32,
                    v,
                    mech_trace::gr!("1006065d", "PTR_DAT_100af0c8", PTR_DAT_100af0c8) as usize
                        as *mut i32,
                )
            );
            if !puvar1.is_null() {
                mech_trace::gw!("1006065d", "100b36e4", DAT_100b36e4, param_1 as u32);
                mech_trace::gw!("1006065d", "100b36e8", DAT_100b36e8, local_c as u32);
                let mut local_10: i32;
                if mech_trace::gr!("1006065d", "100b3904", DAT_100b3904) == 0 {
                    local_10 = 0x14;
                } else {
                    if param_3 == 1 {
                        param_2 = ((param_2 as i32) >> 1) as u32;
                    }
                    if mech_trace::gr!("1006065d", "100b3904", DAT_100b3904) < 1 {
                        local_10 = 10;
                    } else {
                        local_10 = mech_trace::cr!(
                            "1006065d",
                            "10002570",
                            FUN_10002570(
                                param_2,
                                mech_trace::gr!("1006065d", "100b3904", DAT_100b3904) as i32
                            )
                        ) as i32;
                    }
                    if local_10 < 0x10000 {
                        local_10 = 1;
                    } else {
                        local_10 >>= 0x10;
                    }
                }
                if param_3 == 2 {
                    mech_trace::gw!("1006065d", "100b36f0", DAT_100b36f0, 1);
                } else {
                    mech_trace::gw!("1006065d", "100b36f0", DAT_100b36f0, local_10 as u32);
                }
                if param_3 == 1 || param_3 == 2 {
                    mech_trace::gw!("1006065d", "100b36ec", DAT_100b36ec, local_10 as u32);
                } else {
                    mech_trace::gw!("1006065d", "100b36ec", DAT_100b36ec, 0);
                }
                mech_trace::cr!(
                    "1006065d",
                    "10071905",
                    FUN_10071905(
                        (&raw mut DAT_100c7e40).cast::<u32>(),
                        local_14 as usize as i32,
                        puvar1 as usize as i32,
                        0,
                        0x100,
                        local_10 as u32,
                    )
                );
                local_8 = 1;
                let v = mech_trace::rd!(
                    "1006065d",
                    tbl.byte_offset(param_1.wrapping_mul(4) as isize)
                        .cast::<u32>(),
                    read_unaligned
                );
                mech_trace::cr!(
                    "1006065d",
                    "1008664f",
                    FUN_1008664f(
                        v,
                        mech_trace::gr!("1006065d", "PTR_DAT_100af0c8", PTR_DAT_100af0c8) as usize
                            as *mut i32
                    )
                );
            }
            if local_c == -1 {
                mech_trace::cr!(
                    "1006065d",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("1006065d", "100adf74", DAT_100adf74),
                        1,
                        local_14.cast::<c_void>()
                    )
                );
            } else {
                let v = mech_trace::rd!(
                    "1006065d",
                    tbl.byte_offset(local_c.wrapping_mul(4) as isize)
                        .cast::<u32>(),
                    read_unaligned
                );
                mech_trace::cr!(
                    "1006065d",
                    "1008664f",
                    FUN_1008664f(
                        v,
                        mech_trace::gr!("1006065d", "PTR_DAT_100af0c8", PTR_DAT_100af0c8) as usize
                            as *mut i32
                    )
                );
            }
        }
        mech_trace::rt!("1006065d", local_8)
    }
}
