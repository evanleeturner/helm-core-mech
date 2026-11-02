// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083a59(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    ) -> u8;
    fn FUN_10006b9a(param_1: i32) -> u8;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_10006b20(param_1: i32) -> i32;
    fn FUN_100142b4(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c4b2(
    param_1: i32,
    mut param_2: i32,
    mut param_3: i32,
    mut param_4: i32,
) -> u32 {
    mech_trace::enter!("1004c4b2", param_1, param_2, param_3, param_4);
    let mut local_24: i32 = 0;
    let mut local_18: u32 = 0;
    let mut local_14: u32 = 0;
    let mut local_10: i32 = 0;
    let local_c: i32 = 0;

    // SAFETY: the callees are the game's own; param_1 points at a game object
    // whose fields at 0x50..0x58 are read; out-pointers refer to live locals.
    unsafe {
        let a = mech_trace::rd!(
            "1004c4b2",
            (param_1 as *const u8).byte_add(0x50).cast::<i32>(),
            read_unaligned
        );
        let b = mech_trace::rd!(
            "1004c4b2",
            (param_1 as *const u8).byte_add(0x54).cast::<i32>(),
            read_unaligned
        );
        let c = mech_trace::rd!(
            "1004c4b2",
            (param_1 as *const u8).byte_add(0x58).cast::<i32>(),
            read_unaligned
        );
        mech_trace::cr!(
            "1004c4b2",
            "10083a59",
            FUN_10083a59(
                a.wrapping_sub(param_2) as u32,
                b.wrapping_sub(param_3) as u32,
                c.wrapping_sub(param_4) as u32,
                &raw mut local_24,
                &raw mut local_18,
                &raw mut local_14,
                &raw mut local_10,
            )
        );
        let local_8: i32 = (local_14 as i32) / 1000;
        let r1 = mech_trace::cr!("1004c4b2", "10006b9a", FUN_10006b9a(local_10)) as i32;
        let r2 = mech_trace::cr!("1004c4b2", "10006b9a", FUN_10006b9a(local_24)) as i32;
        let i_var1 = mech_trace::cr!(
            "1004c4b2",
            "10003d60",
            FUN_10003d60(r2.wrapping_mul(1000), r1)
        );
        let r3 = mech_trace::cr!("1004c4b2", "10006b9a", FUN_10006b9a(local_10)) as i32;
        let r4 = mech_trace::cr!("1004c4b2", "10006b20", FUN_10006b20(local_24));
        let i_var2 = mech_trace::cr!(
            "1004c4b2",
            "10003d60",
            FUN_10003d60(r4.wrapping_mul(1000), r3)
        );
        let i_var3 = mech_trace::cr!("1004c4b2", "10006b20", FUN_10006b20(local_10));
        let mut local_20: i32 = 0;
        loop {
            if local_8 <= local_20 {
                return mech_trace::rt!("1004c4b2", 1);
            }
            param_2 = param_2.wrapping_sub(i_var1);
            param_3 = param_3.wrapping_add(i_var3.wrapping_mul(-1000));
            param_4 = param_4.wrapping_sub(i_var2);
            let local_1c = mech_trace::cr!(
                "1004c4b2",
                "100142b4",
                FUN_100142b4(param_2, param_3, param_4)
            );
            if local_c < local_1c.wrapping_sub(1000) || local_1c.wrapping_add(1000) < local_c {
                break;
            }
            local_20 = local_20.wrapping_add(1);
        }
        mech_trace::rt!("1004c4b2", 0)
    }
}
