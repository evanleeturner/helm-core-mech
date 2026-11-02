// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_10215654: u32;
    static mut DAT_10215658: u32;
    static mut DAT_1021565c: u32;
    static mut DAT_10215660: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_100086f7() -> u32;
    fn FUN_1000895c();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d80(param_1: *mut i32);
    fn FUN_1000a3fd(param_1: i32, param_2: *mut u32, param_3: *mut i32, param_4: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000a4d6() {
    mech_trace::enter!("1000a4d6");
    // SAFETY: the globals and callees are the game's own; the pointer table
    // and the entries it holds are read where the original reads them.
    unsafe {
        if mech_trace::gr!("1000a4d6", "10215658", DAT_10215658) < 3 {
            mech_trace::cr!("1000a4d6", "100086f7", FUN_100086f7());
            if mech_trace::gr!("1000a4d6", "10215658", DAT_10215658) < 3 {
                if mech_trace::gr!("1000a4d6", "10215658", DAT_10215658) != 0 {
                    mech_trace::gw!(
                        "1000a4d6",
                        "10215654",
                        DAT_10215654,
                        mech_trace::gr!("1000a4d6", "1021565c", DAT_1021565c)
                    );
                    mech_trace::gw!("1000a4d6", "1021565c", DAT_1021565c, 1);
                }
                let off =
                    mech_trace::gr!("1000a4d6", "10215658", DAT_10215658).wrapping_mul(4) as usize;
                let puvar1 = mech_trace::rd!(
                    "1000a4d6",
                    (&raw mut DAT_100a7b30)
                        .cast::<u8>()
                        .byte_add(off)
                        .cast::<*mut u32>(),
                    read_unaligned
                );
                if !puvar1.is_null() {
                    let pi_var2 = mech_trace::rd!(
                        "1000a4d6",
                        puvar1.byte_add(20).cast::<*mut i32>(),
                        read_unaligned
                    );
                    if !pi_var2.is_null() {
                        if mech_trace::gr!("1000a4d6", "10215654", DAT_10215654)
                            != mech_trace::gr!("1000a4d6", "1021565c", DAT_1021565c)
                        {
                            FUN_10003d80(pi_var2);
                        }
                        let i_var3 = mech_trace::cr!(
                            "1000a4d6",
                            "1000a3fd",
                            FUN_1000a3fd(0, puvar1, pi_var2, 0)
                        );
                        if i_var3 == 0 {
                            FUN_1000895c();
                        }
                    }
                }
            }
        } else if mech_trace::gr!("1000a4d6", "10215658", DAT_10215658) == 4
            && mech_trace::gr!("1000a4d6", "10215660", DAT_10215660) < 3
        {
            mech_trace::cr!("1000a4d6", "100086f7", FUN_100086f7());
        }
    }
}
