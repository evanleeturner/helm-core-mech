// SPDX-License-Identifier: MIT

use core::mem::transmute;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a1c: *mut i32;
    static mut DAT_100b9a2c: *mut i32;
    static mut DAT_100b9f74: *mut i32;
    static mut DAT_100b9f78: *mut i32;
    static mut DAT_100b9f7c: *mut i32;
    static mut DAT_100b9f8c: i32;
    static mut DAT_100c9070: i32;
    static mut DAT_100c91f4: i32;
    static mut DAT_100ba4b0: u8;
    static mut DAT_100ba4d0: u8;
    static mut DAT_100ba4f0: u8;
    static mut DAT_100ba510: u8;
    static mut DAT_100ba52c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *const i8, ...);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10084708() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10084f85(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
) -> i32 {
    mech_trace::enter!("10084f85", param_1, param_2, param_3, param_4);
    // SAFETY: globals and COM interface pointers belong to the game; vtable
    // slots are read and called through as __stdcall exactly where the
    // original does.
    unsafe {
        if mech_trace::rd!(
            "10084f85",
            mech_trace::gr!("10084f85", "100b9a1c", DAT_100b9a1c),
            read
        ) == 0
        {
            return mech_trace::rt!(
                "10084f85",
                mech_trace::cr!("10084f85", "10084708", FUN_10084708())
            );
        }
        if mech_trace::gr!("10084f85", "100b9f8c", DAT_100b9f8c) == 0 {
            mech_trace::gw!("10084f85", "100c9070", DAT_100c9070, 0);
        } else {
            let vt = mech_trace::rd!(
                "10084f85",
                mech_trace::gr!("10084f85", "100b9f7c", DAT_100b9f7c),
                read
            ) as usize;
            let f: extern "stdcall" fn(*mut i32, i32) -> i32 = transmute(mech_trace::rd!(
                "10084f85",
                ((vt + 0x80) as *const usize),
                read_unaligned
            ));
            mech_trace::gw!(
                "10084f85",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "10084f85",
                    "ind",
                    f(
                        mech_trace::gr!("10084f85", "100b9f7c", DAT_100b9f7c),
                        mech_trace::gr!("10084f85", "100c91f4", DAT_100c91f4)
                    )
                )
            );
            mech_trace::wr!(
                "10084f85",
                mech_trace::gr!("10084f85", "100b9a2c", DAT_100b9a2c),
                write,
                0
            );
            mech_trace::gw!("10084f85", "100b9f8c", DAT_100b9f8c, 0);
        }
        if mech_trace::gr!("10084f85", "100c9070", DAT_100c9070) != 0 {
            FUN_10002168(
                (&raw mut DAT_100ba4b0).cast::<i8>(),
                mech_trace::gr!("10084f85", "100c9070", DAT_100c9070),
            );
        }
        let vt = mech_trace::rd!(
            "10084f85",
            mech_trace::gr!("10084f85", "100b9f74", DAT_100b9f74),
            read
        ) as usize;
        let f: extern "stdcall" fn(*mut i32) -> i32 = transmute(mech_trace::rd!(
            "10084f85",
            ((vt + 0x60) as *const usize),
            read_unaligned
        ));
        let i_var2 = mech_trace::cr!(
            "10084f85",
            "ind",
            f(mech_trace::gr!("10084f85", "100b9f74", DAT_100b9f74))
        );
        if i_var2 == -0x7789fe3e_i32 {
            let vt = mech_trace::rd!(
                "10084f85",
                mech_trace::gr!("10084f85", "100b9f74", DAT_100b9f74),
                read
            ) as usize;
            let f: extern "stdcall" fn(*mut i32) -> i32 = transmute(mech_trace::rd!(
                "10084f85",
                ((vt + 0x6c) as *const usize),
                read_unaligned
            ));
            mech_trace::gw!(
                "10084f85",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "10084f85",
                    "ind",
                    f(mech_trace::gr!("10084f85", "100b9f74", DAT_100b9f74))
                )
            );
            let vt = mech_trace::rd!(
                "10084f85",
                mech_trace::gr!("10084f85", "100b9f7c", DAT_100b9f7c),
                read
            ) as usize;
            let f: extern "stdcall" fn(*mut i32) -> i32 = transmute(mech_trace::rd!(
                "10084f85",
                ((vt + 0x60) as *const usize),
                read_unaligned
            ));
            let i_var2 = mech_trace::cr!(
                "10084f85",
                "ind",
                f(mech_trace::gr!("10084f85", "100b9f7c", DAT_100b9f7c))
            );
            if i_var2 == -0x7789fe3e_i32 {
                let vt = mech_trace::rd!(
                    "10084f85",
                    mech_trace::gr!("10084f85", "100b9f7c", DAT_100b9f7c),
                    read
                ) as usize;
                let f: extern "stdcall" fn(*mut i32) -> i32 = transmute(mech_trace::rd!(
                    "10084f85",
                    ((vt + 0x6c) as *const usize),
                    read_unaligned
                ));
                mech_trace::gw!(
                    "10084f85",
                    "100c9070",
                    DAT_100c9070,
                    mech_trace::cr!(
                        "10084f85",
                        "ind",
                        f(mech_trace::gr!("10084f85", "100b9f7c", DAT_100b9f7c))
                    )
                );
            }
        } else {
            mech_trace::gw!("10084f85", "100c9070", DAT_100c9070, 0);
        }
        if mech_trace::gr!("10084f85", "100c9070", DAT_100c9070) != 0 {
            FUN_10002168(
                (&raw mut DAT_100ba4d0).cast::<i8>(),
                mech_trace::gr!("10084f85", "100c9070", DAT_100c9070),
            );
        }
        let mut local_14: [i32; 4] = [param_1, param_2, param_3, param_4];
        if mech_trace::rd!(
            "10084f85",
            mech_trace::gr!("10084f85", "100b9a1c", DAT_100b9a1c),
            read
        ) == 1
        {
            let vt = mech_trace::rd!(
                "10084f85",
                mech_trace::gr!("10084f85", "100b9f78", DAT_100b9f78),
                read
            ) as usize;
            let f: extern "stdcall" fn(*mut i32, i32, i32, *mut i32, *mut i32, i32) -> i32 =
                transmute(mech_trace::rd!(
                    "10084f85",
                    ((vt + 0x1c) as *const usize),
                    read_unaligned
                ));
            mech_trace::gw!(
                "10084f85",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "10084f85",
                    "ind",
                    f(
                        mech_trace::gr!("10084f85", "100b9f78", DAT_100b9f78),
                        0,
                        0,
                        mech_trace::gr!("10084f85", "100b9f7c", DAT_100b9f7c),
                        local_14.as_mut_ptr(),
                        0x10,
                    )
                )
            );
            if mech_trace::gr!("10084f85", "100c9070", DAT_100c9070) != 0 {
                FUN_10002168(
                    (&raw mut DAT_100ba4f0).cast::<i8>(),
                    mech_trace::gr!("10084f85", "100c9070", DAT_100c9070),
                );
            }
            let vt = mech_trace::rd!(
                "10084f85",
                mech_trace::gr!("10084f85", "100b9f74", DAT_100b9f74),
                read
            ) as usize;
            let f: extern "stdcall" fn(*mut i32, i32, i32) -> i32 = transmute(mech_trace::rd!(
                "10084f85",
                ((vt + 0x2c) as *const usize),
                read_unaligned
            ));
            mech_trace::gw!(
                "10084f85",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "10084f85",
                    "ind",
                    f(mech_trace::gr!("10084f85", "100b9f74", DAT_100b9f74), 0, 1)
                )
            );
            if mech_trace::gr!("10084f85", "100c9070", DAT_100c9070) != 0 {
                FUN_10002168(
                    (&raw mut DAT_100ba510).cast::<i8>(),
                    mech_trace::gr!("10084f85", "100c9070", DAT_100c9070),
                );
            }
        } else {
            let vt = mech_trace::rd!(
                "10084f85",
                mech_trace::gr!("10084f85", "100b9f74", DAT_100b9f74),
                read
            ) as usize;
            let f: extern "stdcall" fn(*mut i32, i32, i32, *mut i32, *mut i32, i32) -> i32 =
                transmute(mech_trace::rd!(
                    "10084f85",
                    ((vt + 0x1c) as *const usize),
                    read_unaligned
                ));
            mech_trace::gw!(
                "10084f85",
                "100c9070",
                DAT_100c9070,
                mech_trace::cr!(
                    "10084f85",
                    "ind",
                    f(
                        mech_trace::gr!("10084f85", "100b9f74", DAT_100b9f74),
                        0,
                        0,
                        mech_trace::gr!("10084f85", "100b9f7c", DAT_100b9f7c),
                        local_14.as_mut_ptr(),
                        0x10,
                    )
                )
            );
            if mech_trace::gr!("10084f85", "100c9070", DAT_100c9070) != 0 {
                FUN_10002168(
                    (&raw mut DAT_100ba52c).cast::<i8>(),
                    mech_trace::gr!("10084f85", "100c9070", DAT_100c9070),
                );
            }
        }
        mech_trace::rt!(
            "10084f85",
            if mech_trace::gr!("10084f85", "100c9070", DAT_100c9070) == 0 {
                0
            } else {
                -1
            }
        )
    }
}
