// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: i32;
    static mut DAT_100aa1b0: *mut i8;
    static mut DAT_100aa1b8: i16;
    static mut DAT_100aa1f0: i32;
    static mut DAT_100aa1f4: i32;
    static mut DAT_100aa1f8: u32;
    static mut DAT_100aa1fc: u32;
    static mut DAT_100aa200: u32;
    static mut DAT_100aa204: u32;
    static mut DAT_100aa208: *mut i32;
    static mut DAT_100aa20c: i32;
    static mut DAT_100aa210: i32;
    static mut DAT_100aa214: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_100b3900: i32;
    static mut DAT_101eb518: i32;

    static mut s_StartExternalIO____No_DirectPlay_100aa300: [i8; 1];
    static mut s_StartExternalIO___No_DPlay_servi_100aa32c: [i8; 1];
    static mut s_StartExternalIO___DirectPlayCrea_100aa35c: [i8; 1];
    static mut s_StartExternalIO___DPlay_Open_CRE_100aa38c: [i8; 1];
    static mut s_Master_Mech_Player_100aa3c0: [i8; 1];
    static mut s_StartExternalIO___Searching_for_s_100aa3d4: [i8; 1];
    static mut s_StartExternalIO___DPlay_Open_OPE_100aa400: [i8; 1];
    static mut s_Slave_Mech2_Player_100aa434: [i8; 1];
    static mut s_StartExternalIO___DPlay_CreatePl_100aa448: [i8; 1];
    static mut s_StartExternalIO___Located__d_of___100aa478: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *const i8, ...);
    fn FUN_100140da();
    fn FUN_10065d78() -> i32;
    fn FUN_10065e16() -> i32;
    fn FUN_10013b3d(param_1: *mut u32) -> i32;
    fn FUN_1007069f() -> i16;
    fn FUN_10013b09() -> i32;
    fn FUN_10013a57();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn Ordinal_2(param_1: *mut c_void, param_2: i32) -> i32;
    fn Ordinal_1(param_1: i32, param_2: *mut *mut i32, param_3: i32) -> i32;
}

type DPOpenFn = unsafe extern "stdcall" fn(*mut i32, *mut u32) -> i32;
type DPCreatePlayerFn =
    unsafe extern "stdcall" fn(*mut i32, *mut i32, *mut i8, *mut i8, i32) -> i32;
type DPEnumFn = unsafe extern "stdcall" fn(*mut i32, i32, *mut c_void, i32, i32) -> i32;

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10013be9(param_1: *mut i32) -> u32 {
    mech_trace::enter!("10013be9", param_1);
    // SAFETY: all globals, callees and the DirectPlay vtable belong to the
    // game; param_1 and the string buffers are read and written exactly where
    // the original does.
    unsafe {
        let mut local_94: [u32; 31] = [0; 31];
        let local_98: i32;

        mech_trace::rt!(
            "10013be9",
            if param_1.is_null() {
                mech_trace::cr!(
                    "10013be9",
                    "Ordinal_2",
                    Ordinal_2(FUN_10013b09 as usize as *mut c_void, 0)
                );
                if mech_trace::gr!("10013be9", "100aa214", DAT_100aa214) == 0 {
                    FUN_10002168(
                        (&raw mut s_StartExternalIO___No_DPlay_servi_100aa32c).cast::<i8>(),
                    );
                    return mech_trace::rt!("10013be9", 0);
                }
                mech_trace::cr!(
                    "10013be9",
                    "Ordinal_1",
                    Ordinal_1(
                        mech_trace::gr!("10013be9", "100aa214", DAT_100aa214),
                        &raw mut DAT_100aa208,
                        0
                    )
                );
                if mech_trace::gr!("10013be9", "100aa208", DAT_100aa208).is_null() {
                    FUN_10002168(
                        (&raw mut s_StartExternalIO___DirectPlayCrea_100aa35c).cast::<i8>(),
                    );
                    return mech_trace::rt!("10013be9", 0);
                }
                local_94[0] = 0x7c;
                local_94[6] = 8;
                local_94[1] = mech_trace::gr!("10013be9", "100aa1f8", DAT_100aa1f8);
                local_94[2] = mech_trace::gr!("10013be9", "100aa1fc", DAT_100aa1fc);
                local_94[3] = mech_trace::gr!("10013be9", "100aa200", DAT_100aa200);
                local_94[4] = mech_trace::gr!("10013be9", "100aa204", DAT_100aa204);

                // copy the string including its terminator
                let src = mech_trace::gr!("10013be9", "100aa1b0", DAT_100aa1b0);
                let mut len: usize = 0;
                while mech_trace::dr!("10013be9", src.add(len)) != 0 {
                    len += 1;
                }
                let count = len + 1;
                let dst = (&raw mut local_94).cast::<u8>().add(36);
                let mut i = 0usize;
                while i < count {
                    mech_trace::dw!(
                        "10013be9",
                        dst.add(i),
                        mech_trace::dr!("10013be9", src.add(i)) as u8
                    );
                    i += 1;
                }

                if mech_trace::gr!("10013be9", "100aa1f4", DAT_100aa1f4) == 1 {
                    local_94[8] = 2;
                    let pfn_open = (mech_trace::dr!(
                        "10013be9",
                        mech_trace::gr!("10013be9", "100aa208", DAT_100aa208)
                    ) as usize
                        + 0x50) as *const DPOpenFn;
                    let pfn_open = mech_trace::rd!("10013be9", pfn_open, read_unaligned);
                    let r = mech_trace::cr!(
                        "10013be9",
                        "ind",
                        pfn_open(
                            mech_trace::gr!("10013be9", "100aa208", DAT_100aa208),
                            (&raw mut local_94).cast::<u32>()
                        )
                    );
                    if r != 0 {
                        FUN_10002168(
                            (&raw mut s_StartExternalIO___DPlay_Open_CRE_100aa38c).cast::<i8>(),
                        );
                        return mech_trace::rt!("10013be9", 0);
                    }
                    let pfn_create = (mech_trace::dr!(
                        "10013be9",
                        mech_trace::gr!("10013be9", "100aa208", DAT_100aa208)
                    ) as usize
                        + 0x14) as *const DPCreatePlayerFn;
                    let pfn_create = mech_trace::rd!("10013be9", pfn_create, read_unaligned);
                    local_98 = mech_trace::cr!(
                        "10013be9",
                        "ind",
                        pfn_create(
                            mech_trace::gr!("10013be9", "100aa208", DAT_100aa208),
                            &raw mut DAT_100aa20c,
                            mech_trace::gr!("10013be9", "100aa1b0", DAT_100aa1b0),
                            (&raw mut s_Master_Mech_Player_100aa3c0).cast::<i8>(),
                            0,
                        )
                    );
                } else {
                    FUN_10002168(
                        (&raw mut s_StartExternalIO___Searching_for_s_100aa3d4).cast::<i8>(),
                    );
                    loop {
                        let r = mech_trace::cr!(
                            "10013be9",
                            "10013b3d",
                            FUN_10013b3d((&raw mut local_94).cast::<u32>())
                        );
                        if r != 0 {
                            local_94[8] = 1;
                            let pfn_open = (mech_trace::dr!(
                                "10013be9",
                                mech_trace::gr!("10013be9", "100aa208", DAT_100aa208)
                            ) as usize
                                + 0x50)
                                as *const DPOpenFn;
                            let pfn_open = mech_trace::rd!("10013be9", pfn_open, read_unaligned);
                            let r2 = mech_trace::cr!(
                                "10013be9",
                                "ind",
                                pfn_open(
                                    mech_trace::gr!("10013be9", "100aa208", DAT_100aa208),
                                    (&raw mut local_94).cast::<u32>()
                                )
                            );
                            if r2 != 0 {
                                FUN_10002168(
                                    (&raw mut s_StartExternalIO___DPlay_Open_OPE_100aa400)
                                        .cast::<i8>(),
                                );
                                return mech_trace::rt!("10013be9", 0);
                            }
                            let pfn_create = (mech_trace::dr!(
                                "10013be9",
                                mech_trace::gr!("10013be9", "100aa208", DAT_100aa208)
                            ) as usize
                                + 0x14)
                                as *const DPCreatePlayerFn;
                            let pfn_create =
                                mech_trace::rd!("10013be9", pfn_create, read_unaligned);
                            local_98 = mech_trace::cr!(
                                "10013be9",
                                "ind",
                                pfn_create(
                                    mech_trace::gr!("10013be9", "100aa208", DAT_100aa208),
                                    &raw mut DAT_100aa20c,
                                    mech_trace::gr!("10013be9", "100aa1b0", DAT_100aa1b0),
                                    (&raw mut s_Slave_Mech2_Player_100aa434).cast::<i8>(),
                                    0,
                                )
                            );
                            break;
                        }
                        if mech_trace::cr!("10013be9", "1007069f", FUN_1007069f()) == 0x1b {
                            return mech_trace::rt!("10013be9", 0);
                        }
                    }
                }

                if local_98 != 0 {
                    FUN_10002168(
                        (&raw mut s_StartExternalIO___DPlay_CreatePl_100aa448).cast::<i8>(),
                    );
                    return mech_trace::rt!("10013be9", 0);
                }
                loop {
                    if mech_trace::gr!("10013be9", "100aa1ac", DAT_100aa1ac)
                        <= mech_trace::gr!("10013be9", "101eb518", DAT_101eb518)
                    {
                        mech_trace::gw!("10013be9", "100aa1f0", DAT_100aa1f0, 1);
                        FUN_100140da();
                        mech_trace::gw!(
                            "10013be9",
                            "100b38fc",
                            DAT_100b38fc,
                            mech_trace::cr!("10013be9", "10065d78", FUN_10065d78())
                        );
                        mech_trace::gw!(
                            "10013be9",
                            "100b3900",
                            DAT_100b3900,
                            mech_trace::cr!("10013be9", "10065e16", FUN_10065e16())
                        );
                        return mech_trace::rt!("10013be9", 1);
                    }
                    if !mech_trace::gr!("10013be9", "100aa208", DAT_100aa208).is_null() {
                        mech_trace::gw!(
                            "10013be9",
                            "100aa210",
                            DAT_100aa210,
                            mech_trace::gr!("10013be9", "100aa20c", DAT_100aa20c)
                        );
                        mech_trace::gw!("10013be9", "101eb518", DAT_101eb518, 0);
                        let pfn_enum = (mech_trace::dr!(
                            "10013be9",
                            mech_trace::gr!("10013be9", "100aa208", DAT_100aa208)
                        ) as usize
                            + 0x34) as *const DPEnumFn;
                        let pfn_enum = mech_trace::rd!("10013be9", pfn_enum, read_unaligned);
                        mech_trace::cr!(
                            "10013be9",
                            "ind",
                            pfn_enum(
                                mech_trace::gr!("10013be9", "100aa208", DAT_100aa208),
                                500,
                                FUN_10013a57 as usize as *mut c_void,
                                0,
                                0,
                            )
                        );
                    }
                    FUN_10002168(
                        (&raw mut s_StartExternalIO___Located__d_of___100aa478).cast::<i8>(),
                    );
                    if mech_trace::cr!("10013be9", "1007069f", FUN_1007069f()) == 0x1b {
                        return mech_trace::rt!("10013be9", 0);
                    }
                }
            } else {
                mech_trace::gw!("10013be9", "100aa1b8", DAT_100aa1b8, 1);
                mech_trace::gw!(
                    "10013be9",
                    "100aa208",
                    DAT_100aa208,
                    mech_trace::rd!("10013be9", param_1, read_unaligned) as usize as *mut i32
                );
                if mech_trace::gr!("10013be9", "100aa208", DAT_100aa208).is_null() {
                    FUN_10002168(
                        (&raw mut s_StartExternalIO____No_DirectPlay_100aa300).cast::<i8>(),
                    );
                    return mech_trace::rt!("10013be9", 0);
                }
                mech_trace::gw!(
                    "10013be9",
                    "100aa20c",
                    DAT_100aa20c,
                    mech_trace::rd!("10013be9", param_1.byte_add(4), read_unaligned)
                );
                mech_trace::gw!("10013be9", "100aa1f0", DAT_100aa1f0, 1);
                mech_trace::gw!(
                    "10013be9",
                    "100aa210",
                    DAT_100aa210,
                    mech_trace::gr!("10013be9", "100aa20c", DAT_100aa20c)
                );
                FUN_100140da();
                mech_trace::gw!(
                    "10013be9",
                    "100b38fc",
                    DAT_100b38fc,
                    mech_trace::cr!("10013be9", "10065d78", FUN_10065d78())
                );
                mech_trace::gw!(
                    "10013be9",
                    "100b3900",
                    DAT_100b3900,
                    mech_trace::cr!("10013be9", "10065e16", FUN_10065e16())
                );
                1
            }
        )
    }
}
