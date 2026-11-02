// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1001bd50() -> *mut u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001bf60(
        param_1: i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
        param_7: *mut i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001bdc6(param_1: *mut u16) {
    mech_trace::enter!("1001bdc6", param_1);
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the game's own structure; the callees are the
    // game's functions, and reads and writes follow the original's order.
    unsafe {
        if (mech_trace::rd!("1001bdc6", param_1, read_unaligned) & 0x200) != 0 {
            return;
        }
        let slot = param_1.add(0x26).cast::<i32>();
        if mech_trace::rd!("1001bdc6", slot, read_unaligned) == 0 {
            let p = mech_trace::cr!("1001bdc6", "1001bd50", FUN_1001bd50());
            mech_trace::wr!("1001bdc6", slot, write_unaligned, p as usize as i32);
        }
        let pi_var1 = mech_trace::rd!("1001bdc6", slot, read_unaligned) as usize as *mut i32;
        if !pi_var1.is_null() {
            mech_trace::cr!(
                "1001bdc6",
                "1001bf60",
                FUN_1001bf60(
                    param_1 as usize as i32,
                    pi_var1,
                    pi_var1.add(1),
                    pi_var1.add(2),
                    pi_var1.add(3),
                    pi_var1.add(4),
                    pi_var1.add(5),
                )
            );
            mech_trace::wr!(
                "1001bdc6",
                param_1,
                write_unaligned,
                mech_trace::rd!("1001bdc6", param_1, read_unaligned) | 0x200
            );
        }
    }
}
