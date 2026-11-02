// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10038a52(param_1: i32) -> u32 {
    mech_trace::enter!("10038a52", param_1);
    if param_1 != 0 {
        // SAFETY: param_1 is the address of the caller's object; its fields hold
        // heap pointers freed with the game's own heap handle, read in the original's order.
        unsafe {
            let p = param_1 as *const u8;
            if mech_trace::rd!("10038a52", p.byte_add(0x60).cast::<i32>(), read_unaligned) != 0 {
                mech_trace::cr!(
                    "10038a52",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10038a52", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::rd!("10038a52", p.byte_add(0x60).cast::<i32>(), read_unaligned)
                            as *mut core::ffi::c_void,
                    )
                );
            }
            if mech_trace::rd!("10038a52", p.byte_add(100).cast::<i32>(), read_unaligned) != 0 {
                mech_trace::cr!(
                    "10038a52",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10038a52", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::rd!("10038a52", p.byte_add(100).cast::<i32>(), read_unaligned)
                            as *mut core::ffi::c_void,
                    )
                );
            }
            if mech_trace::rd!("10038a52", p.byte_add(0x68).cast::<i32>(), read_unaligned) != 0 {
                let q = mech_trace::rd!("10038a52", p.byte_add(0x68).cast::<i32>(), read_unaligned)
                    as *const i32;
                if mech_trace::rd!("10038a52", q, read_unaligned) != 0 {
                    mech_trace::cr!(
                        "10038a52",
                        "HeapFree",
                        HeapFree(
                            mech_trace::gr!("10038a52", "100adf74", DAT_100adf74),
                            1,
                            mech_trace::rd!(
                                "10038a52",
                                (mech_trace::rd!(
                                    "10038a52",
                                    p.byte_add(0x68).cast::<i32>(),
                                    read_unaligned
                                ) as *const i32),
                                read_unaligned
                            ) as *mut core::ffi::c_void,
                        )
                    );
                }
                mech_trace::cr!(
                    "10038a52",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10038a52", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::rd!("10038a52", p.byte_add(0x68).cast::<i32>(), read_unaligned)
                            as *mut core::ffi::c_void,
                    )
                );
            }
            if mech_trace::rd!("10038a52", p.byte_add(0x6c).cast::<i32>(), read_unaligned) != 0 {
                let q = mech_trace::rd!("10038a52", p.byte_add(0x6c).cast::<i32>(), read_unaligned)
                    as *const i32;
                if mech_trace::rd!("10038a52", q, read_unaligned) != 0 {
                    mech_trace::cr!(
                        "10038a52",
                        "HeapFree",
                        HeapFree(
                            mech_trace::gr!("10038a52", "100adf74", DAT_100adf74),
                            1,
                            mech_trace::rd!(
                                "10038a52",
                                (mech_trace::rd!(
                                    "10038a52",
                                    p.byte_add(0x6c).cast::<i32>(),
                                    read_unaligned
                                ) as *const i32),
                                read_unaligned
                            ) as *mut core::ffi::c_void,
                        )
                    );
                }
                mech_trace::cr!(
                    "10038a52",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10038a52", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::rd!("10038a52", p.byte_add(0x6c).cast::<i32>(), read_unaligned)
                            as *mut core::ffi::c_void,
                    )
                );
            }
            if mech_trace::rd!("10038a52", p.byte_add(0x70).cast::<i32>(), read_unaligned) != 0 {
                mech_trace::cr!(
                    "10038a52",
                    "HeapFree",
                    HeapFree(
                        mech_trace::gr!("10038a52", "100adf74", DAT_100adf74),
                        1,
                        mech_trace::rd!("10038a52", p.byte_add(0x70).cast::<i32>(), read_unaligned)
                            as *mut core::ffi::c_void,
                    )
                );
            }
        }
    }
    mech_trace::rt!("10038a52", 0)
}
