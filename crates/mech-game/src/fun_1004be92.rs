// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a9854: u32;
    static mut DAT_101f26f0: u8;
    static mut DAT_101f2704: u8;
    static mut DAT_101f2708: u8;
    static mut DAT_101f2714: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007be21(param_1: *mut i32, param_2: u32) -> i32;
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1007bf28(param_1: i32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004be92(param_1: u32, param_2: u32, param_3: u32, param_4: i32) {
    mech_trace::enter!("1004be92", param_1, param_2, param_3, param_4);
    let mut param_4 = param_4;
    let mut local_8: i32 = 0;
    if param_4 == 0 {
        param_4 = 0xff;
    }
    let mut local_14: i32 = 0;
    // SAFETY: the globals are the game's own tables; offsets index its
    // 0x2c-byte records, read and written where the original does.
    unsafe {
        while local_14 < 0xff && local_8 < param_4 {
            let i_var2 = local_14.wrapping_mul(0x2c) as usize;
            let a08 = (&raw mut DAT_101f2708).byte_add(i_var2).cast::<i32>();
            let a14 = (&raw mut DAT_101f2714).byte_add(i_var2).cast::<i32>();
            let a26 = (&raw mut DAT_101f26f0).byte_add(i_var2).cast::<*mut i32>();
            if mech_trace::rd!("1004be92", a08, read_unaligned) == 0
                && mech_trace::rd!("1004be92", a14, read_unaligned) == 0xb
                && !mech_trace::rd!("1004be92", a26, read_unaligned).is_null()
            {
                let pi_var1 = mech_trace::rd!("1004be92", a26, read_unaligned);
                let i_var3 = mech_trace::cr!("1004be92", "1007be21", FUN_1007be21(pi_var1, 1));
                if i_var3 != -1 {
                    mech_trace::wr!("1004be92", a08, write_unaligned, 1);
                    mech_trace::wr!(
                        "1004be92",
                        (&raw mut DAT_101f2704).byte_add(i_var2).cast::<u32>(),
                        write_unaligned,
                        mech_trace::gr!("1004be92", "100a9854", DAT_100a9854)
                    );
                    mech_trace::cr!(
                        "1004be92",
                        "1002f37a",
                        FUN_1002f37a(pi_var1 as i32, param_1, param_2, param_3)
                    );
                    mech_trace::cr!("1004be92", "1007bf28", FUN_1007bf28(i_var3));
                    mech_trace::cr!("1004be92", "1002fbc0", FUN_1002fbc0(pi_var1));
                    local_8 = local_8.wrapping_add(1);
                }
            }
            local_14 = local_14.wrapping_add(1);
        }
    }
}
