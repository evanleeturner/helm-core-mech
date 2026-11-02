// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: *mut u8;
    static mut DAT_100b36d8: u32;
    static mut DAT_100aae84: u32;
    static mut DAT_100aaf8c: i32;
    static mut DAT_100aaf90: i32;
    static mut DAT_100aafa4: i32;
    static mut DAT_100a7b60: i32;
    static mut DAT_10160c00: [i32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a6ca(param_1: *mut u32, param_2: *mut u32);
    fn FUN_100603ff(param_1: i32);
    fn FUN_1001a74a(param_1: *mut u32, param_2: *mut u32);
    fn FUN_1003b59a(param_1: i32);
    fn FUN_1003b954(param_1: *mut i32);
    fn FUN_1003b2f0(param_1: *mut i8);
    fn FUN_100186c5(param_1: i32);
    fn FUN_10054c14(param_1: i32);
    fn FUN_1003cd85(param_1: *mut i32);
    fn FUN_10054925(param_1: i32);
    fn FUN_1005f8cf();
    fn FUN_10033f29(param_1: *mut i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1000ec6b();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b002(
    param_1: i32,
    param_2: u32,
    param_3: *mut u32,
    param_4: i32,
    param_5: *mut i32,
) {
    mech_trace::enter!("1006b002", param_1, param_2, param_3, param_4, param_5);
    let mut local_20: [u32; 7] = [0; 7];
    // SAFETY: globals and callees are the game's own; the pointers come from
    // the caller and the game's state object, accessed where the original does.
    unsafe {
        let u_var3 = mech_trace::gr!("1006b002", "100b36d8", DAT_100b36d8);
        let u_var2 = mech_trace::gr!("1006b002", "100aae84", DAT_100aae84);
        mech_trace::gw!("1006b002", "100aae84", DAT_100aae84, 0);
        let u_var1 = mech_trace::rd!(
            "1006b002",
            mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68)
                .byte_add(0x18)
                .cast::<u32>(),
            read_unaligned
        );
        FUN_1001a6ca(
            mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68).cast::<u32>(),
            local_20.as_mut_ptr(),
        );
        FUN_100603ff(param_1);
        mech_trace::wr!(
            "1006b002",
            mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68)
                .byte_add(0x18)
                .cast::<u32>(),
            write_unaligned,
            param_2
        );
        mech_trace::wr!("1006b002", param_3.add(6), write_unaligned, 1);
        FUN_1001a74a(
            mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68).cast::<u32>(),
            param_3,
        );
        FUN_1003b59a(mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68) as i32);
        FUN_1003b954(mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68).cast::<i32>());
        FUN_1003b2f0(mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68).cast::<i8>());
        if (mech_trace::gr!("1006b002", "100aaf8c", DAT_100aaf8c) != 0
            || mech_trace::gr!("1006b002", "100aaf90", DAT_100aaf90) != 0)
            && mech_trace::gr!("1006b002", "100aafa4", DAT_100aafa4) == 0
        {
            FUN_100186c5(mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68) as i32);
        }
        if param_4 == 0 {
            FUN_1003cd85(mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68).cast::<i32>());
            let mut local_34: usize = 0;
            let base = (&raw mut DAT_10160c00).cast::<i32>();
            let mut local_28 = mech_trace::rd!("1006b002", base, read_unaligned);
            while local_28 != 0 {
                FUN_10054925(local_28);
                local_34 = local_34.wrapping_add(1);
                local_28 = mech_trace::rd!("1006b002", base.add(local_34), read_unaligned);
            }
        } else {
            FUN_10054c14(param_4);
        }
        FUN_1005f8cf();
        if !param_5.is_null() {
            FUN_10033f29(param_5, 0, 0, 0);
        }
        mech_trace::gw!("1006b002", "100b36d8", DAT_100b36d8, u_var3);
        FUN_1000ec6b();
        mech_trace::wr!(
            "1006b002",
            mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68)
                .byte_add(0x18)
                .cast::<u32>(),
            write_unaligned,
            u_var1
        );
        mech_trace::gw!("1006b002", "100a7b60", DAT_100a7b60, 1);
        FUN_1001a74a(
            mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68).cast::<u32>(),
            local_20.as_mut_ptr(),
        );
        FUN_1003b59a(mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68) as i32);
        FUN_1003b954(mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68).cast::<i32>());
        FUN_1003b2f0(mech_trace::gr!("1006b002", "100aaf68", DAT_100aaf68).cast::<i8>());
        mech_trace::gw!("1006b002", "100a7b60", DAT_100a7b60, 0);
        mech_trace::gw!("1006b002", "100aae84", DAT_100aae84, u_var2);
    }
}
