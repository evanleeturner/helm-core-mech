// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4114: u32;
    static mut DAT_100aa1ac: u32;
    static mut DAT_100ab318: u32;
    static mut DAT_100af17c: u32;
    static mut DAT_100b0a20: u32;
    static mut DAT_101686f0: u32;
    static mut PTR_DAT_100af0b8: u32;
    static mut DAT_100ad9a8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn _sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1005acaa(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002e635(param_1: i32) {
    mech_trace::enter!("1002e635", param_1);
    let mut local_2c = [0u8; 40];
    // SAFETY: globals and callees are the game's own; param_1 points at the
    // caller's object, read at the offsets the original uses.
    unsafe {
        if (mech_trace::rd!(
            "1002e635",
            (param_1.wrapping_add(4) as *const i16),
            read_unaligned
        )) != 0
        {
            let pu_var1 = mech_trace::cr!(
                "1002e635",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("1002e635", "100af17c", DAT_100af17c) as i32,
                    mech_trace::gr!("1002e635", "101686f0", DAT_101686f0).wrapping_add(1),
                    mech_trace::gr!("1002e635", "PTR_DAT_100af0b8", PTR_DAT_100af0b8) as *mut i32,
                )
            );
            if !pu_var1.is_null() {
                if mech_trace::gr!("1002e635", "100aa1ac", DAT_100aa1ac) != 0
                    && mech_trace::gr!("1002e635", "100b0a20", DAT_100b0a20) == 0
                    && mech_trace::gr!("1002e635", "100ab318", DAT_100ab318) == 0
                {
                    mech_trace::cr!(
                        "1002e635",
                        "_sprintf",
                        _sprintf(
                            local_2c.as_mut_ptr().cast::<i8>(),
                            (&raw mut DAT_100ad9a8).cast::<i8>(),
                            mech_trace::gr!("1002e635", "100a4114", DAT_100a4114),
                        )
                    );
                    let a = mech_trace::rd!(
                        "1002e635",
                        (param_1.wrapping_add(0xa4) as *const *mut i32),
                        read_unaligned
                    );
                    let b = mech_trace::rd!(
                        "1002e635",
                        (param_1.wrapping_add(0xa8) as *const *mut i32),
                        read_unaligned
                    );
                    let b0 = mech_trace::rd!("1002e635", b, read_unaligned);
                    let b1 = mech_trace::rd!("1002e635", b.byte_add(4), read_unaligned);
                    mech_trace::cr!(
                        "1002e635",
                        "1005acaa",
                        FUN_1005acaa(a, b0, b1, pu_var1 as i32, local_2c.as_mut_ptr(), 0x100eebb0)
                    );
                }
                mech_trace::cr!(
                    "1002e635",
                    "1008664f",
                    FUN_1008664f(
                        mech_trace::gr!("1002e635", "101686f0", DAT_101686f0).wrapping_add(1),
                        mech_trace::gr!("1002e635", "PTR_DAT_100af0b8", PTR_DAT_100af0b8)
                            as *mut i32
                    )
                );
            }
        }
    }
}
