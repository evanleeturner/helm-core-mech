// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b98a0: i32;
    static mut DAT_100b98a4: u32;
    static mut DAT_100c8240: [i32; 0];
    static mut DAT_100c8280: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100816df();
    fn FUN_100816ea(param_1: *mut i8) -> i32;
    fn FUN_100108a0(param_1: i32, param_2: *mut i8);
    fn _qsort(
        base: *mut c_void,
        num: u32,
        width: u32,
        cmp: unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
    );
    fn FUN_10086951(param_1: i32) -> i32;
    fn FUN_10081687(a: *const c_void, b: *const c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10081270(param_1: *mut i8) -> i32 {
    mech_trace::enter!("10081270", param_1);
    // SAFETY: the globals, arrays and callees are the game's own memory;
    // indices follow the original's layout (12-byte and 20-byte records).
    unsafe {
        let a = (&raw mut DAT_100c8240) as *mut i32;
        let b = (&raw mut DAT_100c8280) as *mut i32;

        mech_trace::gw!("10081270", "100b98a0", DAT_100b98a0, 0);
        mech_trace::gw!("10081270", "100b98a4", DAT_100b98a4, 0);
        FUN_100816df();
        let r = mech_trace::cr!("10081270", "100816ea", FUN_100816ea(param_1));
        if r == 0 {
            FUN_100108a0(0x44, core::ptr::null_mut());
            return mech_trace::rt!("10081270", 0);
        }
        _qsort(
            b as *mut c_void,
            mech_trace::gr!("10081270", "100b98a4", DAT_100b98a4),
            0x14,
            FUN_10081687,
        );

        let mut local_8: i32 = 0;
        while local_8 < mech_trace::gr!("10081270", "100b98a4", DAT_100b98a4) as i32 {
            let i2 = local_8.wrapping_mul(5);
            let mut local_18: i32 = -1;
            let mut local_10: i32 = 0;
            while local_10 < mech_trace::gr!("10081270", "100b98a0", DAT_100b98a0) {
                let v = mech_trace::dr!(
                    "10081270",
                    a.offset(local_10.wrapping_mul(3).wrapping_add(2) as isize)
                );
                if mech_trace::dr!("10081270", b.offset(i2.wrapping_add(1) as isize)) <= v {
                    if local_18 < 0 {
                        local_18 = local_10;
                    } else if v < mech_trace::dr!(
                        "10081270",
                        a.offset(local_18.wrapping_mul(3).wrapping_add(2) as isize)
                    ) {
                        local_18 = local_10;
                    }
                }
                local_10 = local_10.wrapping_add(1);
            }
            let local_14: *mut i32;
            if local_18 == -1 {
                if 4 < mech_trace::gr!("10081270", "100b98a0", DAT_100b98a0) {
                    FUN_100108a0(0x44, core::ptr::null_mut());
                }
                let i1 = mech_trace::gr!("10081270", "100b98a0", DAT_100b98a0);
                local_14 = a.offset(
                    mech_trace::gr!("10081270", "100b98a0", DAT_100b98a0).wrapping_mul(3) as isize,
                );
                local_18 = mech_trace::gr!("10081270", "100b98a0", DAT_100b98a0);
                mech_trace::gw!(
                    "10081270",
                    "100b98a0",
                    DAT_100b98a0,
                    mech_trace::gr!("10081270", "100b98a0", DAT_100b98a0).wrapping_add(1)
                );
                mech_trace::dw!("10081270", local_14, 0);
                mech_trace::dw!(
                    "10081270",
                    a.offset(i1.wrapping_mul(3).wrapping_add(2) as isize),
                    0xffdc
                );
                mech_trace::dw!(
                    "10081270",
                    a.offset(i1.wrapping_mul(3).wrapping_add(1) as isize),
                    mech_trace::dr!(
                        "10081270",
                        a.offset(i1.wrapping_mul(3).wrapping_add(2) as isize)
                    )
                );
            } else {
                local_14 = a.offset(local_18.wrapping_mul(3) as isize);
            }
            mech_trace::dw!(
                "10081270",
                b.offset(i2.wrapping_add(4) as isize),
                (mech_trace::dr!("10081270", local_14.offset(1)))
                    .wrapping_sub(mech_trace::dr!("10081270", local_14.offset(2)))
            );
            mech_trace::dw!("10081270", b.offset(i2.wrapping_add(3) as isize), local_18);
            mech_trace::dw!("10081270", b.offset(i2.wrapping_add(2) as isize), 0);
            mech_trace::dw!(
                "10081270",
                local_14.offset(2),
                (mech_trace::dr!("10081270", local_14.offset(2))).wrapping_sub(mech_trace::dr!(
                    "10081270",
                    b.offset(i2.wrapping_add(1) as isize)
                ))
            );
            local_8 = local_8.wrapping_add(1);
        }

        local_8 = 0;
        while local_8 < mech_trace::gr!("10081270", "100b98a0", DAT_100b98a0) {
            let i3 = local_8.wrapping_mul(3);
            mech_trace::dw!(
                "10081270",
                a.offset(i3.wrapping_add(1) as isize),
                (mech_trace::dr!("10081270", a.offset(i3.wrapping_add(1) as isize))).wrapping_sub(
                    mech_trace::dr!("10081270", a.offset(i3.wrapping_add(2) as isize))
                )
            );
            mech_trace::dw!(
                "10081270",
                a.offset(i3.wrapping_add(2) as isize),
                mech_trace::dr!("10081270", a.offset(i3.wrapping_add(1) as isize))
            );
            let r2 = mech_trace::cr!(
                "10081270",
                "10086951",
                FUN_10086951(mech_trace::dr!(
                    "10081270",
                    a.offset(i3.wrapping_add(1) as isize)
                ))
            );
            mech_trace::dw!("10081270", a.offset(i3 as isize), r2);
            if mech_trace::dr!("10081270", a.offset(i3 as isize)) == 0 {
                FUN_100108a0(0x45, core::ptr::null_mut());
            }
            local_8 = local_8.wrapping_add(1);
        }
        mech_trace::rt!("10081270", 1)
    }
}
