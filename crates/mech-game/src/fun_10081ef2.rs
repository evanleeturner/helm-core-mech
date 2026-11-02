// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a00: *mut *mut u8;
    static mut DAT_100b9a1c: *mut u8;
    static mut DAT_100b9a20: *mut u8;
    static mut DAT_100b9a2c: i32;
    static mut DAT_100c967c: u32;
    static mut DAT_100c9680: *mut u8;
    static mut DAT_100b34d8: i32;
    static mut DAT_100b9dd8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *mut i8, param_2: i32, param_3: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10081ef2() {
    mech_trace::enter!("10081ef2");
    // SAFETY: all globals and callees are the game's own memory; the table,
    // its entries and the vtable-like function pointers are read exactly as
    // the original does.
    unsafe {
        let mut local_8: *mut u8 = core::ptr::null_mut();
        let mut local_c: i32 = 0;
        while local_c < 6 {
            let entry = mech_trace::rd!(
                "10081ef2",
                mech_trace::gr!("10081ef2", "100b9a00", DAT_100b9a00).offset(local_c as isize),
                read_unaligned
            );
            if mech_trace::rd!("10081ef2", entry.byte_add(8).cast::<i32>(), read_unaligned) != 0
                && mech_trace::rd!(
                    "10081ef2",
                    entry.byte_add(0xc).cast::<i32>(),
                    read_unaligned
                ) != 0
                && (local_8.is_null()
                    || mech_trace::rd!(
                        "10081ef2",
                        entry.byte_add(0xc).cast::<u32>(),
                        read_unaligned
                    ) < mech_trace::rd!(
                        "10081ef2",
                        local_8.byte_add(0xc).cast::<u32>(),
                        read_unaligned
                    ))
            {
                local_8 = mech_trace::rd!(
                    "10081ef2",
                    mech_trace::gr!("10081ef2", "100b9a00", DAT_100b9a00).offset(local_c as isize),
                    read_unaligned
                );
            }
            local_c = local_c.wrapping_add(1);
        }
        let f1 = mech_trace::rd!(
            "10081ef2",
            mech_trace::gr!("10081ef2", "100b9a1c", DAT_100b9a1c)
                .byte_add(0x14)
                .cast::<unsafe extern "C" fn()>(),
            read_unaligned
        );
        mech_trace::cr!("10081ef2", "ind", f1());
        mech_trace::gw!("10081ef2", "100b9a1c", DAT_100b9a1c, local_8);
        let f2 = mech_trace::rd!(
            "10081ef2",
            local_8
                .byte_add(0x10)
                .cast::<unsafe extern "C" fn(i32, u32, *mut u8)>(),
            read_unaligned
        );
        mech_trace::cr!(
            "10081ef2",
            "ind",
            f2(
                mech_trace::gr!("10081ef2", "100b9a2c", DAT_100b9a2c),
                mech_trace::gr!("10081ef2", "100c967c", DAT_100c967c),
                mech_trace::gr!("10081ef2", "100c9680", DAT_100c9680)
            )
        );
        mech_trace::gw!(
            "10081ef2",
            "100b9a20",
            DAT_100b9a20,
            mech_trace::gr!("10081ef2", "100b9a1c", DAT_100b9a1c)
        );
        mech_trace::gw!("10081ef2", "100b34d8", DAT_100b34d8, 1);
        FUN_10002168(
            (&raw mut DAT_100b9dd8).cast::<i8>(),
            mech_trace::gr!("10081ef2", "100b9a2c", DAT_100b9a2c),
            mech_trace::gr!("10081ef2", "100c967c", DAT_100c967c),
        );
    }
}
