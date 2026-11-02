// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab0a8: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006403e(param_1: i32, param_2: *mut i32, param_3: *mut i32, param_4: *mut i32) -> u8;
    fn FUN_1002f322(param_1: i32, param_2: *mut u32, param_3: *mut u32, param_4: *mut u32) -> u8;
    fn FUN_1002628c(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10063fbd(param_1: i32, param_2: *mut i32) {
    mech_trace::enter!("10063fbd", param_1, param_2);
    let mut local_1c: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: the callees and the global are the game's own; the locals
    // outlive the calls and param_1 points at the game's object.
    unsafe {
        mech_trace::cr!(
            "10063fbd",
            "1006403e",
            FUN_1006403e(
                param_1,
                &raw mut local_8,
                &raw mut local_c,
                &raw mut local_10,
            )
        );
        let p = ((param_1 as usize).wrapping_add(0x44)) as *const i32;
        mech_trace::cr!(
            "10063fbd",
            "1002f322",
            FUN_1002f322(
                mech_trace::rd!("10063fbd", p, read_unaligned),
                (&raw mut local_14).cast::<u32>(),
                (&raw mut local_18).cast::<u32>(),
                (&raw mut local_1c).cast::<u32>(),
            )
        );
        let g = mech_trace::gr!("10063fbd", "100ab0a8", DAT_100ab0a8);
        if !g.is_null() {
            local_18 = local_18.wrapping_add(mech_trace::rd!("10063fbd", g, read_unaligned));
        }
        mech_trace::cr!(
            "10063fbd",
            "1002628c",
            FUN_1002628c(
                param_2, local_14, local_18, local_1c, local_8, local_c, local_10, 150000,
            )
        );
    }
}
