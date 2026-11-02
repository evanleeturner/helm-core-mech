// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a6f0(param_1: i32) -> u8;
    fn FUN_1000a890(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100329e0(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("100329e0", param_1, param_2);
    // SAFETY: param_1 points at the caller's game structure; the reads and
    // writes happen at the same offsets and in the same order as the original.
    unsafe {
        let p = param_1.wrapping_add(0x22c) as *mut i32;
        mech_trace::wr!(
            "100329e0",
            p,
            write_unaligned,
            mech_trace::rd!("100329e0", p, read_unaligned).wrapping_add(100)
        );
        mech_trace::cr!("100329e0", "1000a6f0", FUN_1000a6f0(param_1));
        mech_trace::cr!("100329e0", "1000a890", FUN_1000a890(param_1));
        let mut i_var1: i32 = 0;
        'outer: loop {
            i_var1 = i_var1.wrapping_add(1);
            let mut local_8 = param_1.wrapping_add(0x368) as *const i32;
            let mut local_4: i32 = 0;
            loop {
                let mut pi = local_8;
                let mut i_var3: i32 = 0;
                loop {
                    let v = mech_trace::rd!("100329e0", pi, read_unaligned);
                    if (v / 100)
                        .wrapping_mul(4)
                        .wrapping_sub(0x140)
                        .wrapping_mul(0x19)
                        == 0
                        && v % 100 == i_var1
                    {
                        continue 'outer;
                    }
                    pi = pi.add(1);
                    i_var3 += 1;
                    if i_var3 >= 0xc {
                        break;
                    }
                }
                local_8 = local_8.add(0xc);
                local_4 += 1;
                if 7 < local_4 {
                    return mech_trace::rt!("100329e0", param_2.wrapping_add(i_var1));
                }
            }
        }
    }
}
