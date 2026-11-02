// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa208: *mut i32;
    static mut DAT_101eb4c0: *mut u8;
    static mut DAT_100a4148: i32;
    static mut DAT_100a41ac: u8;
    static mut DAT_100a41c8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002168(param_1: *const i8, ...);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn Sleep(dw_milliseconds: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10006fc9() -> u32 {
    mech_trace::enter!("10006fc9");
    let mut local_14: [u8; 4] = [0; 4];
    let mut local_10: u32 = 0x100;
    let mut local_8: u32 = 0;
    // SAFETY: the globals, the DirectPlay vtable and the callees are the
    // game's own memory; the locals outlive the calls that receive pointers
    // to them.
    unsafe {
        let this = mech_trace::gr!("10006fc9", "100aa208", DAT_100aa208);
        let vtbl = mech_trace::rd!("10006fc9", this, read_unaligned) as usize;
        let f: unsafe extern "stdcall" fn(
            *mut i32,
            *mut u32,
            *mut u8,
            u32,
            *mut u8,
            *mut u32,
        ) -> i32 = core::mem::transmute(mech_trace::rd!(
            "10006fc9",
            (vtbl.wrapping_add(0x54) as *const usize),
            read_unaligned
        ));
        let local_c = mech_trace::cr!(
            "10006fc9",
            "ind",
            f(
                this,
                &raw mut local_8,
                local_14.as_mut_ptr(),
                1,
                mech_trace::gr!("10006fc9", "101eb4c0", DAT_101eb4c0),
                &raw mut local_10,
            )
        );
        if local_c == 0 {
            mech_trace::gw!("10006fc9", "100a4148", DAT_100a4148, 0);
            let buf = mech_trace::gr!("10006fc9", "101eb4c0", DAT_101eb4c0);
            FUN_10002168(
                (&raw const DAT_100a41ac).cast::<i8>(),
                mech_trace::rd!("10006fc9", buf.cast::<i8>(), read) as i32,
                mech_trace::rd!("10006fc9", buf.cast::<i8>().add(1), read) as i32,
                local_8,
                local_c,
            );
        } else {
            if local_c != -0x7788ff42 {
                FUN_10002168((&raw const DAT_100a41c8).cast::<i8>());
                let i_var1 = mech_trace::gr!("10006fc9", "100a4148", DAT_100a4148);
                mech_trace::gw!("10006fc9", "100a4148", DAT_100a4148, i_var1.wrapping_add(1));
                if i_var1 != 0 {
                    if i_var1 == 1 {
                        Sleep(100);
                    } else {
                        Sleep(100);
                    }
                }
            }
            local_8 = 0xffffffff;
        }
        mech_trace::rt!("10006fc9", local_8)
    }
}
