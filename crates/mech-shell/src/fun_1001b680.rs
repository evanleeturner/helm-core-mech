// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008f988: [u8; 0];
    static mut DAT_1008fd10: [i8; 0];
    static mut DAT_100b1290: [u32; 0];
    static mut DAT_100b12a8: [u32; 0];
    static mut DAT_100b12c0: [u8; 0];
    static mut DAT_100b13e8: [u8; 0];
    static mut DAT_100b50d8: [u32; 0];
    static mut DAT_100b50f0: u32;
    static mut DAT_100c41c0: [u8; 0];
    static mut PTR_DAT_10083d8c: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001baa0(param_1: u32, param_2: i32);
    fn FUN_100361e0(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: i32);
    fn FUN_10035f30(param_1: i32, param_2: *mut i8, param_3: *mut i8, param_4: *mut i8) -> u32;
    fn FUN_100361a0(param_1: i32);
    fn strncpy(dest: *mut i8, source: *mut i8, count: usize) -> *mut i8;
    fn FUN_1006e790(param_1: *mut c_void);
    fn FUN_1001ace0(param_1: *mut i8);
    fn FUN_10063de0(param_1: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10036260();
    fn FUN_10036280();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001b680(param_1: i32, param_2: i32, param_3: i32) {
    mech_trace::enter!("1001b680", param_1, param_2, param_3);
    // SAFETY: all globals and callees are the game's own memory and code;
    // param_1 points at the caller's structure, accessed at the original's offsets.
    unsafe {
        let a1290 = (&raw mut DAT_100b1290).cast::<u32>();
        let a12a8 = (&raw mut DAT_100b12a8).cast::<u32>();
        let b12c0 = (&raw mut DAT_100b12c0).cast::<u8>();
        let b13e8 = (&raw mut DAT_100b13e8).cast::<u8>();
        let a50d8 = (&raw mut DAT_100b50d8).cast::<u32>();
        let c41c0 = (&raw mut DAT_100c41c0).cast::<u8>();
        let ptrs = (&raw mut PTR_DAT_10083d8c).cast::<u32>();
        let f24 = param_1 as usize + 0x24;
        let f28 = param_1 as usize + 0x28;
        let mut ac_stack: [i8; 16] = [0; 16];

        let mut i_var3: i32 = mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32;
        if param_3 != 0 {
            if mech_trace::dr!(
                "1001b680",
                b12c0.offset(mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32 as isize)
            ) == 0
            {
                if -1 < mech_trace::rd!("1001b680", (f28 as *const i32), read_unaligned) {
                    FUN_1001baa0(
                        mech_trace::dr!(
                            "1001b680",
                            ptrs.offset(
                                (mech_trace::rd!("1001b680", (f28 as *const i32), read_unaligned)
                                    .wrapping_mul(10)) as isize,
                            )
                        ),
                        mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32,
                    );
                }
                i_var3 = mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32;
                if mech_trace::dr!(
                    "1001b680",
                    a12a8.offset(
                        mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32 as isize
                    )
                ) == 0
                {
                    mech_trace::dw!(
                        "1001b680",
                        b12c0
                            .offset(mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32
                                as isize),
                        0
                    );
                    mech_trace::dw!("1001b680", a50d8.offset(i_var3 as isize), 0);
                    mech_trace::dw!("1001b680", a12a8.offset(i_var3 as isize), 0);
                    mech_trace::dw!("1001b680", a1290.offset(i_var3 as isize), 0);
                    return;
                }
                mech_trace::dw!(
                    "1001b680",
                    b12c0.offset(
                        mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32 as isize
                    ),
                    1
                );
                mech_trace::dw!("1001b680", a50d8.offset(i_var3 as isize), 0);
            } else {
                let i_var2 = mech_trace::dr!(
                    "1001b680",
                    a50d8.offset(
                        mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32 as isize
                    )
                ) as i32;
                mech_trace::dw!(
                    "1001b680",
                    a50d8.offset(
                        mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32 as isize
                    ),
                    i_var2.wrapping_add(param_3) as u32
                );
                if i_var2.wrapping_add(param_3) < 0 {
                    mech_trace::dw!("1001b680", a50d8.offset(i_var3 as isize), 0);
                }
                if (mech_trace::dr!("1001b680", a12a8.offset(i_var3 as isize)) as i32)
                    <= (mech_trace::dr!("1001b680", a50d8.offset(i_var3 as isize)) as i32)
                {
                    mech_trace::dw!(
                        "1001b680",
                        a50d8.offset(i_var3 as isize),
                        (mech_trace::dr!("1001b680", a12a8.offset(i_var3 as isize)) as i32)
                            .wrapping_sub(1) as u32
                    );
                }
            }
            match mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned) {
                0 | 1 | 2 => {
                    FUN_100361e0(0, -1, -1, -1, -1);
                    if mech_trace::rd!("1001b680", (f28 as *const i32), read_unaligned) < 0 {
                        FUN_100361a0(mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32);
                    } else {
                        let idx = (mech_trace::dr!(
                            "1001b680",
                            a50d8.offset(mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0)
                                as i32 as isize)
                        ) as i32)
                            .wrapping_add(
                                (mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32)
                                    .wrapping_mul(200),
                            );
                        mech_trace::cr!(
                            "1001b680",
                            "10035f30",
                            FUN_10035f30(
                                mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32,
                                b13e8.offset(idx.wrapping_mul(0xd) as isize).cast::<i8>(),
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                            )
                        );
                    }
                    FUN_10036260();
                }
                3 | 4 | 5 => {
                    FUN_100361e0(1, -1, -1, -1, -1);
                    if mech_trace::rd!("1001b680", (f28 as *const i32), read_unaligned) < 0 {
                        let mut done = false;
                        if mech_trace::rd!("1001b680", (f28 as *const i32), read_unaligned) != -1 {
                            mech_trace::cr!(
                                "1001b680",
                                "strncpy",
                                strncpy(
                                    ac_stack.as_mut_ptr(),
                                    c41c0
                                        .offset(
                                            mech_trace::rd!(
                                                "1001b680",
                                                (f24 as *const i32),
                                                read_unaligned
                                            )
                                            .wrapping_mul(0x10)
                                                as isize
                                        )
                                        .cast::<i8>(),
                                    3,
                                )
                            );
                            ac_stack[3] = 0;
                            if ac_stack[0] != 0 {
                                mech_trace::cr!(
                                    "1001b680",
                                    "10035f30",
                                    FUN_10035f30(
                                        mech_trace::rd!(
                                            "1001b680",
                                            (f24 as *const i32),
                                            read_unaligned
                                        )
                                        .wrapping_add(-3),
                                        ac_stack.as_mut_ptr(),
                                        core::ptr::null_mut(),
                                        core::ptr::null_mut(),
                                    )
                                );
                                done = true;
                            }
                        }
                        if !done {
                            FUN_100361a0(
                                mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned)
                                    .wrapping_add(-3),
                            );
                        }
                    } else {
                        let idx = (mech_trace::dr!(
                            "1001b680",
                            a50d8.offset(mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0)
                                as i32 as isize)
                        ) as i32)
                            .wrapping_add(
                                (mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32)
                                    .wrapping_mul(200),
                            );
                        mech_trace::cr!(
                            "1001b680",
                            "10035f30",
                            FUN_10035f30(
                                mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned)
                                    .wrapping_add(-3),
                                b13e8.offset(idx.wrapping_mul(0xd) as isize).cast::<i8>(),
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                            )
                        );
                    }
                    FUN_10036280();
                }
                _ => {}
            }
        }
        i_var3 = mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32;
        if param_2 != 0 {
            mech_trace::dw!(
                "1001b680",
                b12c0.offset(mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32 as isize),
                0
            );
            mech_trace::dw!("1001b680", a50d8.offset(i_var3 as isize), 0);
            mech_trace::dw!("1001b680", a12a8.offset(i_var3 as isize), 0);
            if mech_trace::dr!("1001b680", a1290.offset(i_var3 as isize)) != 0 {
                FUN_1006e790(
                    mech_trace::dr!("1001b680", a1290.offset(i_var3 as isize)) as *mut c_void
                );
                mech_trace::dw!(
                    "1001b680",
                    a1290.offset(
                        mech_trace::gr!("1001b680", "100b50f0", DAT_100b50f0) as i32 as isize
                    ),
                    0
                );
            }
            let mut u_var4: u32 = (mech_trace::rd!("1001b680", (f28 as *const i32), read_unaligned)
                .wrapping_add(param_2)) as u32;
            if mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned) == 3
                && u_var4 == 0xffff_ffff
            {
                u_var4 = ((-1 < param_2) as u32).wrapping_sub(1) & 0x3d;
            }
            if mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned) < 3 {
                if (u_var4 as i32) < -1 {
                    u_var4 = 0x3d;
                } else if 0x3d < (u_var4 as i32) {
                    u_var4 = 0xffff_ffff;
                }
            } else if (u_var4 as i32) < -2 {
                u_var4 = 0x3d;
            } else if 0x3d < (u_var4 as i32) {
                u_var4 = 0xffff_fffe;
            }
            mech_trace::wr!("1001b680", (f28 as *mut u32), write_unaligned, u_var4);
            let pc_var5: *mut i8 = if (u_var4 as i32) < 0 {
                (&raw mut DAT_1008fd10).cast::<i8>()
            } else {
                mech_trace::dr!(
                    "1001b680",
                    ptrs.offset(u_var4.wrapping_mul(10) as i32 as isize)
                ) as *mut i8
            };
            FUN_1001ace0(pc_var5);
            match mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned) {
                0 | 1 | 2 => {
                    FUN_100361e0(0, -1, -1, -1, -1);
                    if mech_trace::rd!("1001b680", (f28 as *const i32), read_unaligned) < 0 {
                        FUN_100361a0(mech_trace::rd!(
                            "1001b680",
                            (f24 as *const i32),
                            read_unaligned
                        ));
                    } else {
                        mech_trace::cr!(
                            "1001b680",
                            "10035f30",
                            FUN_10035f30(
                                mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned),
                                mech_trace::dr!(
                                    "1001b680",
                                    ptrs.offset(
                                        (mech_trace::rd!(
                                            "1001b680",
                                            (f28 as *const i32),
                                            read_unaligned
                                        )
                                        .wrapping_mul(10))
                                            as isize,
                                    )
                                ) as *mut i8,
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                            )
                        );
                    }
                    FUN_10036260();
                }
                3 | 4 | 5 => {
                    FUN_100361e0(1, -1, -1, -1, -1);
                    i_var3 = mech_trace::rd!("1001b680", (f28 as *const i32), read_unaligned);
                    if i_var3 < 0 {
                        let mut done = false;
                        if i_var3 != -1 {
                            mech_trace::cr!(
                                "1001b680",
                                "strncpy",
                                strncpy(
                                    ac_stack.as_mut_ptr(),
                                    c41c0
                                        .offset(
                                            mech_trace::rd!(
                                                "1001b680",
                                                (f24 as *const i32),
                                                read_unaligned
                                            )
                                            .wrapping_mul(0x10)
                                                as isize
                                        )
                                        .cast::<i8>(),
                                    3,
                                )
                            );
                            ac_stack[3] = 0;
                            if ac_stack[0] != 0 {
                                mech_trace::cr!(
                                    "1001b680",
                                    "10035f30",
                                    FUN_10035f30(
                                        mech_trace::rd!(
                                            "1001b680",
                                            (f24 as *const i32),
                                            read_unaligned
                                        )
                                        .wrapping_add(-3),
                                        ac_stack.as_mut_ptr(),
                                        core::ptr::null_mut(),
                                        core::ptr::null_mut(),
                                    )
                                );
                                done = true;
                            }
                        }
                        if !done {
                            FUN_100361a0(
                                mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned)
                                    .wrapping_add(-3),
                            );
                        }
                    } else {
                        mech_trace::cr!(
                            "1001b680",
                            "10035f30",
                            FUN_10035f30(
                                mech_trace::rd!("1001b680", (f24 as *const i32), read_unaligned)
                                    .wrapping_add(-3),
                                mech_trace::dr!(
                                    "1001b680",
                                    ptrs.offset(i_var3.wrapping_mul(10) as isize)
                                ) as *mut i8,
                                core::ptr::null_mut(),
                                core::ptr::null_mut(),
                            )
                        );
                    }
                    FUN_10036280();
                }
                _ => {}
            }
        }
        FUN_10063de0((&raw mut DAT_1008f988).cast::<i32>());
    }
}
