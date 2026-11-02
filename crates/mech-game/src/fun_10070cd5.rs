// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b482c: [i8; 8];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10070ba0(param_1: i32, param_2: *mut i8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10070cd5(param_1: i32, param_2: *mut u32, param_3: *mut u32) {
    mech_trace::enter!("10070cd5", param_1, param_2, param_3);
    // SAFETY: the callee and the global are the game's own; param_1 is an
    // address of the caller's header block, and param_2/param_3 point at the
    // caller's output words, accessed where the original does.
    unsafe {
        let u_var1 = mech_trace::cr!(
            "10070cd5",
            "10070ba0",
            FUN_10070ba0(8, param_1.wrapping_add(100) as *mut i8)
        );
        mech_trace::wr!("10070cd5", param_2.add(2), write_unaligned, u_var1);
        let u_var1 = mech_trace::cr!(
            "10070cd5",
            "10070ba0",
            FUN_10070ba0(0xc, param_1.wrapping_add(0x88) as *mut i8)
        );
        mech_trace::wr!("10070cd5", param_2.add(9), write_unaligned, u_var1);
        let u_var1 = mech_trace::cr!(
            "10070cd5",
            "10070ba0",
            FUN_10070ba0(0xc, param_1.wrapping_add(0x7c) as *mut i8)
        );
        mech_trace::wr!("10070cd5", param_2.add(7), write_unaligned, u_var1);
        let mut i_var2: i32 = 8;
        let mut pc_var3 = param_1.wrapping_add(0x101) as *const i8;
        let mut pc_var4 = &raw mut DAT_100b482c as *const i8;
        let mut b_var5;
        loop {
            if i_var2 == 0 {
                // unreachable on first pass; flag kept from last iteration
                b_var5 = true;
                break;
            }
            i_var2 -= 1;
            b_var5 = mech_trace::rd!("10070cd5", pc_var3, read)
                == mech_trace::rd!("10070cd5", pc_var4, read);
            pc_var3 = pc_var3.add(1);
            pc_var4 = pc_var4.add(1);
            if !b_var5 {
                break;
            }
        }
        if b_var5 {
            mech_trace::wr!("10070cd5", param_3, write_unaligned, 1);
            mech_trace::wr!("10070cd5", param_2, write_unaligned, 0);
        } else {
            mech_trace::wr!("10070cd5", param_3, write_unaligned, 0);
            let u_var1 = mech_trace::cr!(
                "10070cd5",
                "10070ba0",
                FUN_10070ba0(8, param_1.wrapping_add(0x6c) as *mut i8)
            );
            mech_trace::wr!("10070cd5", param_2.add(4), write_unaligned, u_var1);
            let u_var1 = mech_trace::cr!(
                "10070cd5",
                "10070ba0",
                FUN_10070ba0(8, param_1.wrapping_add(0x74) as *mut i8)
            );
            mech_trace::wr!("10070cd5", param_2.add(5), write_unaligned, u_var1);
            mech_trace::wr!("10070cd5", param_2, write_unaligned, 0);
        }
    }
}
