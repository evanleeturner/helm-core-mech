// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b09dc: u32;
    static mut DAT_101155bc: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064f77(param_1: i32, param_2: i32, param_3: i32, param_4: i32, param_5: u32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10051e44(param_1: u32, param_2: u32, param_3: i32, param_4: i32) -> u64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10051d99(param_1: i32) -> i32 {
    mech_trace::enter!("10051d99", param_1);
    let mut local_c: i32 = 2;
    // SAFETY: the globals and callee are the game's own; param_1 points at a
    // caller-provided structure of at least 0x14 bytes, read where the original does.
    unsafe {
        if param_1 != 0 && mech_trace::gr!("10051d99", "100b09dc", DAT_100b09dc) == 1 {
            mech_trace::gw!("10051d99", "101155bc", DAT_101155bc, param_1 as u32);
            let p = param_1 as *const u8;
            let a = mech_trace::rd!("10051d99", p.byte_add(4).cast::<i32>(), read_unaligned);
            let b = mech_trace::rd!("10051d99", p.byte_add(8).cast::<i32>(), read_unaligned);
            let c = mech_trace::rd!("10051d99", p.byte_add(0xc).cast::<i32>(), read_unaligned)
                .wrapping_sub(mech_trace::rd!(
                    "10051d99",
                    p.byte_add(4).cast::<i32>(),
                    read_unaligned
                ))
                .wrapping_add(1);
            let d = mech_trace::rd!("10051d99", p.byte_add(0x10).cast::<i32>(), read_unaligned)
                .wrapping_sub(mech_trace::rd!(
                    "10051d99",
                    p.byte_add(8).cast::<i32>(),
                    read_unaligned
                ))
                .wrapping_add(1);
            local_c = mech_trace::cr!(
                "10051d99",
                "10064f77",
                FUN_10064f77(
                    a,
                    b,
                    c,
                    d,
                    FUN_10051e44 as unsafe extern "fastcall" fn(u32, u32, i32, i32) -> u64 as usize
                        as u32,
                )
            );
            mech_trace::gw!("10051d99", "100b09dc", DAT_100b09dc, 2);
        }
    }
    mech_trace::rt!("10051d99", local_c)
}
