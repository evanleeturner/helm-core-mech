// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069a27() {
    mech_trace::enter!("10069a27");
    let mut local_8: i32 = 10;
    let mut local_c: i32 = 10;
    let mut local_10: i32 = 0;
    while local_10 < 0x100 {
        let mut local_14: i32 = 0;
        while local_14 < 2 {
            // SAFETY: the callee and the global are the game's own; the
            // array's address is passed as the original does.
            unsafe {
                mech_trace::cr!(
                    "10069a27",
                    "10056489",
                    FUN_10056489(
                        (&raw mut DAT_101f52e0).cast::<i32>(),
                        local_8,
                        local_c.wrapping_add(local_14),
                        local_8.wrapping_add(3),
                        local_c.wrapping_add(local_14),
                        0,
                        local_10,
                    )
                );
            }
            local_14 = local_14.wrapping_add(1);
        }
        let n = (local_10 as u32).wrapping_add(1);
        let u_var1 = ((n as i32) >> 0x1f) as u32;
        if ((n ^ u_var1).wrapping_sub(u_var1) & 0xf) ^ u_var1 == u_var1 {
            local_8 = 10;
            local_c = local_c.wrapping_add(2);
        } else {
            local_8 = local_8.wrapping_add(4);
        }
        local_10 = local_10.wrapping_add(1);
    }
}
