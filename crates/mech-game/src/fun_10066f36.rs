// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10066ec0(param_1: i32, param_2: u32, param_3: *mut u32) -> i32;
    fn FUN_10067126(param_1: *mut i32, param_2: u32, param_3: i32, param_4: *mut i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066f36(
    param_1: i32,
    param_2: *mut i32,
    param_3: *mut i32,
) -> *mut i32 {
    mech_trace::enter!("10066f36", param_1, param_2, param_3);
    let mut local: [i32; 2] = [0; 2];
    let mut local_c: i32 = 0;
    // SAFETY: param_1 points at the game's rect record, param_2 and param_3
    // at the caller's integer pairs; the callees are the game's own.
    unsafe {
        let r = param_1 as *const i32;
        let l4 = mech_trace::rd!("10066f36", r.byte_add(4), read_unaligned);
        let lc = mech_trace::rd!("10066f36", r.byte_add(0xc), read_unaligned);
        let l8 = mech_trace::rd!("10066f36", r.byte_add(8), read_unaligned);
        let l10 = mech_trace::rd!("10066f36", r.byte_add(0x10), read_unaligned);
        local[0] = lc.wrapping_sub(l4).wrapping_add(1) >> 1;
        local[1] = l10.wrapping_sub(l8).wrapping_add(1) >> 1;
        let a = local[0].wrapping_sub(mech_trace::rd!("10066f36", param_2, read_unaligned));
        let mut local_8: u32 = (-1 < a) as u32;
        let b = local[1].wrapping_sub(mech_trace::rd!("10066f36", param_2.add(1), read_unaligned));
        if -1 < b {
            local_8 |= 2;
        }
        let a = local[0].wrapping_sub(mech_trace::rd!("10066f36", param_2, read_unaligned));
        let b = local[1].wrapping_sub(mech_trace::rd!("10066f36", param_2.add(1), read_unaligned));
        let u_var1 = mech_trace::cr!(
            "10066f36",
            "10066ec0",
            FUN_10066ec0(a, b as u32, (&raw mut local_c).cast::<u32>())
        ) as u32;
        local_8 = ((u_var1 & 1) << 2) | (local_8 & 0xfffffffb);
        mech_trace::cr!(
            "10066f36",
            "10067126",
            FUN_10067126(local.as_mut_ptr(), local_8, local_c, param_3)
        );
        mech_trace::rt!("10066f36", param_3)
    }
}
