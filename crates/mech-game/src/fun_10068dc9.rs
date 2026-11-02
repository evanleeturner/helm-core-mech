// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_100b3980: i32;
    static mut DAT_100b39a4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10068ea5(
        param_1: i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: i32,
        param_5: *mut i32,
    ) -> i32;
    fn FUN_1001d615(param_1: *mut u16, param_2: u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10068dc9(
    param_1: i32,
    param_2: *mut i32,
    param_3: i32,
    param_4: i32,
) -> *mut u16 {
    mech_trace::enter!("10068dc9", param_1, param_2, param_3, param_4);
    let mut local_10: i32 = 0;
    let mut local_c: *mut u16 = core::ptr::null_mut();
    let mut local_8: *mut u16 = core::ptr::null_mut();
    // SAFETY: the globals and callees are the game's own; param_2 points at
    // the caller's int, and the locals' addresses are passed as the original does.
    unsafe {
        mech_trace::gw!("10068dc9", "_DAT_100b3980", _DAT_100b3980, 0);
        if mech_trace::rd!("10068dc9", param_2, read_unaligned) < param_3 {
            let r = mech_trace::cr!(
                "10068dc9",
                "10068ea5",
                FUN_10068ea5(
                    param_1,
                    param_2,
                    (&raw mut local_c).cast::<i32>(),
                    param_4,
                    &raw mut local_10,
                )
            );
            if r == 0 {
                local_8 = local_c;
                while mech_trace::rd!("10068dc9", param_2, read_unaligned) < param_3 {
                    let r2 = mech_trace::cr!(
                        "10068dc9",
                        "10068ea5",
                        FUN_10068ea5(
                            param_1,
                            param_2,
                            (&raw mut local_8).cast::<i32>(),
                            param_4,
                            &raw mut local_10,
                        )
                    );
                    if r2 != 0 {
                        break;
                    }
                    if !local_8.is_null() {
                        FUN_1001d615(
                            local_8,
                            mech_trace::gr!("10068dc9", "100b39a4", DAT_100b39a4) as u16,
                        );
                    }
                }
            }
        }
        if !local_c.is_null() {
            FUN_1001d615(
                local_c,
                mech_trace::gr!("10068dc9", "100b39a4", DAT_100b39a4) as u16,
            );
        }
        mech_trace::rt!("10068dc9", local_c)
    }
}
