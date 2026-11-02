// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7558: u32;
    static mut DAT_100aae48: u32;
    static mut DAT_100aae50: u32;
    static mut DAT_100aae58: u32;
    static mut DAT_100aae5c: u32;
    static mut DAT_100aae60: u32;
    static mut DAT_100aae64: u32;
    static mut DAT_100aae68: u32;
    static mut DAT_100aae6c: [u8; 1];
    static mut DAT_100aae70: u32;
    static mut DAT_100aae74: u32;
    static mut DAT_100aae78: u32;
    static mut DAT_100aae7c: u32;
    static mut DAT_100aae80: u32;
    static mut DAT_100aafe4: [i8; 1];
    static mut DAT_100af17c: u32;
    static mut DAT_100b351c: u32;
    static mut DAT_100b352c: *mut u16;
    static mut PTR_DAT_100af100: u32;
    static mut s_mission__4s_globalintensity__d_t_100aaff4: [i8; 1];
    static mut s_skygnd_par_100aafe8: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1001767b(param_1: i32, param_2: *mut i8) -> u8;
    fn fopen(filename: *mut i8, mode: *mut i8) -> *mut c_void;
    fn FUN_100175e7(param_1: *mut c_void, param_2: i32, param_3: i32) -> u32;
    fn sscanf(src: *mut i8, format: *mut i8, ...) -> i32;
    fn strncmp(a: *mut i8, b: *mut i8, n: usize) -> i32;
    fn FUN_1007d001(param_1: i32) -> u8;
    fn FUN_10031035(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017750(param_1: *mut i8) {
    mech_trace::enter!("10017750", param_1);
    let mut local_26c: i32 = 0;
    let mut local_268: i32 = 0x80;
    let mut local_264: [i8; 32] = [0; 32];
    let mut local_244: [i8; 32] = [0; 32];
    let mut local_224: [i8; 32] = [0; 32];
    let mut local_204: [i8; 512] = [0; 512];

    // SAFETY: globals and callees are the game's own; all pointers passed
    // refer to live locals or the game's own data.
    unsafe {
        if mech_trace::gr!("10017750", "100b352c", DAT_100b352c).is_null() {
            mech_trace::gw!(
                "10017750",
                "100b352c",
                DAT_100b352c,
                mech_trace::cr!(
                    "10017750",
                    "1008668b",
                    FUN_1008668b(
                        mech_trace::gr!("10017750", "100af17c", DAT_100af17c) as i32,
                        mech_trace::gr!("10017750", "100b351c", DAT_100b351c),
                        mech_trace::gr!("10017750", "100af100", PTR_DAT_100af100) as usize
                            as *mut i32,
                    )
                )
            );
        }
        mech_trace::gw!(
            "10017750",
            "100aae80",
            DAT_100aae80,
            (mech_trace::rd!(
                "10017750",
                mech_trace::gr!("10017750", "100b352c", DAT_100b352c),
                read_unaligned
            ) as u8 as i8) as i32 as u32
        );
        mech_trace::cr!(
            "10017750",
            "1001767b",
            FUN_1001767b(local_224.as_mut_ptr() as usize as i32, param_1)
        );
        let p_fvar1 = mech_trace::cr!(
            "10017750",
            "fopen",
            fopen(
                (&raw mut s_skygnd_par_100aafe8).cast::<i8>(),
                (&raw mut DAT_100aafe4).cast::<i8>(),
            )
        );
        if p_fvar1.is_null() {
            mech_trace::gw!("10017750", "100aae48", DAT_100aae48, 320000);
            mech_trace::gw!("10017750", "100aae74", DAT_100aae74, 0);
            mech_trace::gw!("10017750", "100aae78", DAT_100aae78, 320000);
            mech_trace::gw!("10017750", "100aae7c", DAT_100aae7c, 0);
            mech_trace::gw!("10017750", "100aae50", DAT_100aae50, 160000);
            mech_trace::gw!("10017750", "100aae58", DAT_100aae58, 5000);
            mech_trace::gw!("10017750", "100aae5c", DAT_100aae5c, 320000);
            mech_trace::gw!("10017750", "100aae64", DAT_100aae64, 0);
            mech_trace::gw!("10017750", "100aae68", DAT_100aae68, 0);
        } else {
            loop {
                let u_var2 = mech_trace::cr!(
                    "10017750",
                    "100175e7",
                    FUN_100175e7(p_fvar1, local_204.as_mut_ptr() as usize as i32, 0x100)
                );
                if u_var2 == 0 {
                    return;
                }
                mech_trace::cr!(
                    "10017750",
                    "sscanf",
                    sscanf(
                        local_204.as_mut_ptr(),
                        (&raw mut s_mission__4s_globalintensity__d_t_100aaff4).cast::<i8>(),
                        local_244.as_mut_ptr(),
                        &raw mut local_268,
                        &raw mut DAT_100aae58,
                        &raw mut DAT_100aae64,
                        &raw mut DAT_100aae5c,
                        &raw mut local_26c,
                        &raw mut DAT_100aae68,
                        (&raw mut DAT_100aae6c).cast::<u8>(),
                        &raw mut DAT_100aae70,
                    )
                );
                mech_trace::cr!(
                    "10017750",
                    "1001767b",
                    FUN_1001767b(
                        local_264.as_mut_ptr() as usize as i32,
                        local_244.as_mut_ptr(),
                    )
                );
                let i_var3 = mech_trace::cr!(
                    "10017750",
                    "strncmp",
                    strncmp(local_224.as_mut_ptr(), local_264.as_mut_ptr(), 4)
                );
                if i_var3 == 0 {
                    break;
                }
            }
            mech_trace::gw!("10017750", "100aae74", DAT_100aae74, 0);
            mech_trace::gw!(
                "10017750",
                "100aae78",
                DAT_100aae78,
                mech_trace::gr!("10017750", "100aae48", DAT_100aae48)
            );
            mech_trace::cr!("10017750", "1007d001", FUN_1007d001(local_268));
            mech_trace::cr!("10017750", "10031035", FUN_10031035(local_26c));
            mech_trace::gw!(
                "10017750",
                "100aae60",
                DAT_100aae60,
                mech_trace::gr!("10017750", "100aae5c", DAT_100aae5c)
            );
            mech_trace::gw!(
                "10017750",
                "100aae5c",
                DAT_100aae5c,
                mech_trace::gr!("10017750", "100aae5c", DAT_100aae5c).wrapping_shl(1)
            );
            mech_trace::gw!(
                "10017750",
                "100a7558",
                DAT_100a7558,
                mech_trace::gr!("10017750", "100aae70", DAT_100aae70)
            );
        }
    }
}
