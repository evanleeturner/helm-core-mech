// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa540: u32;
    static mut DAT_100aa544: u32;
    static mut DAT_100aa548: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005fbab(
        param_1: *mut u16,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005fb3a(
    param_1: *mut u16,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: *mut i32,
) -> i32 {
    mech_trace::enter!("1005fb3a", param_1, param_2, param_3, param_4, param_5);
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_8: u32 = 0;
    // SAFETY: the callee and globals are the game's own; locals are valid
    // for the callee's writes and param_5 is the caller's output pointer.
    unsafe {
        mech_trace::cr!(
            "1005fb3a",
            "1005fbab",
            FUN_1005fbab(
                param_1,
                param_2,
                param_3,
                param_4,
                &raw mut local_8,
                (&raw mut local_c).cast::<u32>(),
                &raw mut local_10,
            )
        );
        if local_c != 0 {
            mech_trace::wr!("1005fb3a", param_5, write_unaligned, local_10);
            mech_trace::gw!("1005fb3a", "100aa548", DAT_100aa548, 0);
            mech_trace::gw!("1005fb3a", "100aa540", DAT_100aa540, 0);
            mech_trace::gw!("1005fb3a", "100aa544", DAT_100aa544, 0x10000);
        }
    }
    mech_trace::rt!("1005fb3a", local_c)
}
