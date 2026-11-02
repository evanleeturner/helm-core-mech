// SPDX-License-Identifier: MIT

unsafe extern "C" {
    fn FUN_1000d230(param_1: i32) -> i32;
    fn FUN_1000d260(param_1: i32, param_2: i32);
    fn FUN_1006b3f0() -> i32;
    fn FUN_1006b420() -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006b450(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
) {
    mech_trace::enter!("1006b450", param_1, param_2, param_3, param_4, param_5);
    // SAFETY: the callees are the game's own; param_1 and param_2 are
    // addresses of the caller's structures, read and written where the
    // original does.
    unsafe {
        let mut i_var5 = mech_trace::cr!("1006b450", "1000d230", FUN_1000d230(param_5));
        FUN_1000d260(i_var5, param_5);
        if i_var5 == 0 {
            i_var5 = mech_trace::cr!("1006b450", "1006b3f0", FUN_1006b3f0());
            mech_trace::wr!(
                "1006b450",
                ((param_1
                    .wrapping_add(0x2f0)
                    .wrapping_add(i_var5.wrapping_mul(8))) as *mut i32),
                write_unaligned,
                param_5
            );
        } else if i_var5 == 1 {
            i_var5 = mech_trace::cr!("1006b450", "1006b420", FUN_1006b420());
            mech_trace::wr!(
                "1006b450",
                ((param_1
                    .wrapping_add(0x28c)
                    .wrapping_add(i_var5.wrapping_mul(4))) as *mut i32),
                write_unaligned,
                param_5
            );
        } else if i_var5 == 2 {
            if param_5 == 0x1e78 {
                return;
            }
            if param_5 == 0x1eaa {
                return;
            }
        }
        i_var5 = 0;
        let mut pi_var2 = param_2.wrapping_add(0x368) as *const i32;
        let mut pi_var3 = param_1.wrapping_add(0x368) as *mut i32;
        loop {
            let mut i_var4: i32 = 0;
            loop {
                if (param_3 != i_var5 || param_4 != i_var4)
                    && mech_trace::rd!("1006b450", pi_var2, read_unaligned) == param_5
                {
                    mech_trace::wr!("1006b450", pi_var3, write_unaligned, param_5);
                }
                pi_var3 = pi_var3.wrapping_add(1);
                pi_var2 = pi_var2.wrapping_add(1);
                i_var4 = i_var4.wrapping_add(1);
                if i_var4 >= 0xc {
                    break;
                }
            }
            i_var5 = i_var5.wrapping_add(1);
            if i_var5 >= 8 {
                break;
            }
        }
    }
}
