// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10023525(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut u32) -> u8;
    fn FUN_1003ba89(param_1: *mut i32, param_2: *mut i32, param_3: *mut i32) -> u32;
    fn FUN_1003510c(param_1: i32, param_2: i32, param_3: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10033e5a(param_1: *mut i32) {
    mech_trace::enter!("10033e5a", param_1);
    let mut local_1c: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_14: u32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    // SAFETY: param_1 points at the game's object; the pointers derived from
    // its fields are the game's own memory, read where the original reads.
    unsafe {
        let base = mech_trace::rd!("10033e5a", param_1, read_unaligned);
        let v = mech_trace::rd!(
            "10033e5a",
            (base.wrapping_add(0xdc) as *const u32),
            read_unaligned
        );
        if (((v >> 8) as u8) & 0xf) == 2
            && (mech_trace::rd!("10033e5a", (base.wrapping_add(0xdd) as *const u8), read) & 0x10)
                == 0
        {
            let idx = mech_trace::rd!("10033e5a", param_1.add(0xf8), read_unaligned);
            let tbl = mech_trace::rd!("10033e5a", param_1.add(0x15), read_unaligned);
            let addr = idx.wrapping_mul(0x40).wrapping_add(tbl).wrapping_add(8);
            if mech_trace::rd!("10033e5a", (addr as *const i32), read_unaligned) == 1 {
                mech_trace::cr!(
                    "10033e5a",
                    "10023525",
                    FUN_10023525(
                        base,
                        &raw mut local_1c,
                        &raw mut local_18,
                        &raw mut local_14,
                    )
                );
                let r = mech_trace::cr!(
                    "10033e5a",
                    "1003ba89",
                    FUN_1003ba89(
                        (&raw mut local_14).cast::<i32>(),
                        &raw mut local_10,
                        &raw mut local_c,
                    )
                );
                if r != 0 {
                    mech_trace::cr!(
                        "10033e5a",
                        "1003510c",
                        FUN_1003510c(local_14 as i32, local_10, 0x5d)
                    );
                }
            }
        }
    }
}
