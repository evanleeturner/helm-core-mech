// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100bd7c8: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10050a30(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    if param_2 < 8 {
        let mut i_var1: i32 = param_3
            .wrapping_add(1)
            .wrapping_add(param_2.wrapping_mul(0xc));
        // SAFETY: the global is the game's own memory; only its address is taken.
        let mut pu_var2: *mut u32 = unsafe {
            (&raw mut DAT_100bd7c8)
                .cast::<u8>()
                .byte_add(i_var1 as isize as usize)
                .cast::<u32>()
        };
        let mut pi_var5: *mut i32 = i_var1
            .wrapping_mul(4)
            .wrapping_add(0x368)
            .wrapping_add(param_1) as usize as *mut i32;
        let mut local_8: i32 = 8i32.wrapping_sub(param_2);
        loop {
            if param_3.wrapping_add(1) < 0xc {
                i_var1 = 0xci32.wrapping_sub(param_3.wrapping_add(1));
                let mut pu_var3 = pu_var2;
                let mut pi_var4 = pi_var5;
                loop {
                    // SAFETY: the pointers walk the game's own tables exactly
                    // as the original does.
                    unsafe {
                        if pi_var4.read_unaligned() == param_4 {
                            pu_var3.write_unaligned(1);
                        }
                    }
                    pu_var3 = pu_var3.wrapping_add(1);
                    pi_var4 = pi_var4.wrapping_add(1);
                    i_var1 = i_var1.wrapping_sub(1);
                    if i_var1 == 0 {
                        break;
                    }
                }
            }
            pi_var5 = pi_var5.wrapping_add(0xc);
            pu_var2 = pu_var2.wrapping_add(0xc);
            local_8 = local_8.wrapping_sub(1);
            if local_8 == 0 {
                break;
            }
        }
    }
}
