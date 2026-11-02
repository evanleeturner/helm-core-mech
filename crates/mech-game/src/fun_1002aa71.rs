// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_1016a6a0: i32;
    static mut DAT_1016a6a4: *mut *mut u16;
    static mut DAT_100af17c: i32;
    static mut PTR_DAT_100af0f8: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, param_2: i32, param_3: *mut i32) -> *mut u16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1002aa71() -> i32 {
    mech_trace::enter!("1002aa71");
    // SAFETY: the globals and the callee are the game's own; the table at
    // DAT_1016a6a4 holds at least nine pointer slots, written in order.
    unsafe {
        mech_trace::gw!("1002aa71", "_DAT_1016a6a0", _DAT_1016a6a0, 0);
        let mut local_8: i32 = 0;
        while local_8 < 9 {
            let p_var1 = mech_trace::cr!(
                "1002aa71",
                "1008668b",
                FUN_1008668b(
                    mech_trace::gr!("1002aa71", "100af17c", DAT_100af17c),
                    local_8.wrapping_add(1),
                    mech_trace::gr!("1002aa71", "PTR_DAT_100af0f8", PTR_DAT_100af0f8)
                )
            );
            mech_trace::wr!(
                "1002aa71",
                mech_trace::gr!("1002aa71", "1016a6a4", DAT_1016a6a4).offset(local_8 as isize),
                write_unaligned,
                p_var1
            );
            local_8 = local_8.wrapping_add(1);
        }
    }
    mech_trace::rt!("1002aa71", 1)
}
