// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ad6c0: u32;
    static mut DAT_100b0864: i32;
    static mut DAT_101155c0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100814bb(param_1: i32, param_2: i32) -> i32;
    fn FUN_100504ef(param_1: i32, param_2: i32, param_3: *mut i32) -> u32;
    fn FUN_100509fd(param_1: i32, param_2: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10030398() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004ff90(param_1: i32) -> u32 {
    mech_trace::enter!("1004ff90", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 points at
    // the caller's record, and the table at DAT_101155c0 has 0x48-byte entries.
    unsafe {
        let base = (&raw mut DAT_101155c0).cast::<u8>();
        let key = mech_trace::rd!(
            "1004ff90",
            ((param_1 as u32).wrapping_add(4) as *const i32),
            read_unaligned
        );
        let mut local_c: i32 = 0;
        let mut local_10: i32 = 0;
        while local_10 < mech_trace::gr!("1004ff90", "100b0864", DAT_100b0864) {
            let v = mech_trace::rd!(
                "1004ff90",
                base.byte_add(local_10.wrapping_mul(0x48) as usize)
                    .cast::<i32>(),
                read_unaligned
            );
            if v == key {
                local_c = local_c.wrapping_add(1);
            }
            local_10 = local_10.wrapping_add(1);
        }
        let i_var3 = mech_trace::gr!("1004ff90", "100ad6c0", DAT_100ad6c0) as i32;
        let i_var1 = mech_trace::cr!("1004ff90", "10030398", FUN_10030398()) as i32;
        let mut local_8 = mech_trace::cr!(
            "1004ff90",
            "100814bb",
            FUN_100814bb(i_var1.wrapping_mul(local_c) & 0xffff, i_var3)
        ) as *mut i32;
        mech_trace::rt!(
            "1004ff90",
            if local_8.is_null() {
                0
            } else {
                local_10 = 0;
                while local_10 < mech_trace::gr!("1004ff90", "100b0864", DAT_100b0864) {
                    let key = mech_trace::rd!(
                        "1004ff90",
                        ((param_1 as u32).wrapping_add(4) as *const i32),
                        read_unaligned
                    );
                    let v = mech_trace::rd!(
                        "1004ff90",
                        base.byte_add(local_10.wrapping_mul(0x48) as usize)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    if v == key {
                        let p18 = mech_trace::rd!(
                            "1004ff90",
                            ((param_1 as u32).wrapping_add(0x18) as *const i32),
                            read_unaligned
                        );
                        mech_trace::cr!(
                            "1004ff90",
                            "100504ef",
                            FUN_100504ef(local_10, p18, local_8)
                        );
                        let i_var3 = mech_trace::cr!("1004ff90", "10030398", FUN_10030398());
                        local_8 = (local_8 as usize).wrapping_add(i_var3 as usize) as *mut i32;
                        let p18 = mech_trace::rd!(
                            "1004ff90",
                            ((param_1 as u32).wrapping_add(0x18) as *const i32),
                            read_unaligned
                        );
                        mech_trace::cr!("1004ff90", "100509fd", FUN_100509fd(local_10, p18));
                    }
                    local_10 = local_10.wrapping_add(1);
                }
                1
            }
        )
    }
}
