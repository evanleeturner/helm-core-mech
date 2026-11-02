// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100baa08: u32;
    static mut DAT_100bf7d0: i32;
    static mut DAT_100c8908: u8;
    static mut DAT_100c8cf8: u8;
    static mut DAT_100c23bc: i8;
    static mut DAT_100c23b0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    #[link_name = "_strcmpi"]
    fn __strcmpi(_Str1: *mut i8, _Str2: *mut i8) -> i32;
    fn FUN_10086a65(param_1: *mut i8) -> i32;
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10086cc1(param_1: *mut i8) -> i32 {
    mech_trace::enter!("10086cc1", param_1);
    // SAFETY: the globals and callees are the game's own; param_1 is the
    // caller's C string, and the tables are indexed as the original does.
    unsafe {
        let mut local_c: i32 = 0;
        loop {
            if mech_trace::gr!("10086cc1", "100bf7d0", DAT_100bf7d0) <= local_c {
                let i_var2 = mech_trace::cr!("10086cc1", "10086a65", FUN_10086a65(param_1));
                let i_var3 = mech_trace::cr!(
                    "10086cc1",
                    "__strcmpi",
                    __strcmpi(param_1, &raw mut DAT_100c23b0)
                );
                let mut i_var1 = mech_trace::gr!("10086cc1", "100baa08", DAT_100baa08) as i32;
                if i_var3 == 0 {
                    i_var1 = i_var2;
                    if i_var2 == -1 {
                        mech_trace::cr!(
                            "10086cc1",
                            "100108a0",
                            FUN_100108a0(0x11, &raw mut DAT_100c23bc)
                        );
                        i_var1 = mech_trace::gr!("10086cc1", "100baa08", DAT_100baa08) as i32;
                    }
                }
                mech_trace::gw!("10086cc1", "100baa08", DAT_100baa08, i_var1 as u32);
                return mech_trace::rt!("10086cc1", i_var2);
            }
            let i_var1 = mech_trace::cr!(
                "10086cc1",
                "__strcmpi",
                __strcmpi(
                    (&raw mut DAT_100c8cf8).byte_add(local_c.wrapping_mul(0x28) as usize)
                        as *mut i8,
                    param_1,
                )
            );
            if i_var1 == 0 {
                break;
            }
            local_c = local_c.wrapping_add(1);
        }
        if mech_trace::rd!(
            "10086cc1",
            (&raw mut DAT_100c8908)
                .byte_add(local_c.wrapping_mul(4) as usize)
                .cast::<i32>(),
            read_unaligned
        ) != 0
        {
            return mech_trace::rt!("10086cc1", local_c);
        }
        mech_trace::rt!("10086cc1", -1)
    }
}
