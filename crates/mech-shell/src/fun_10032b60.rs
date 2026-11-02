// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a890(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10032b60(param_1: i32, param_2: i32) -> i32 {
    mech_trace::enter!("10032b60", param_1, param_2);
    // SAFETY: param_1 is the address of the game's own structure; every
    // offset read and written here is where the original accesses it.
    unsafe {
        let base = param_1 as usize as *mut u8;
        mech_trace::wr!(
            "10032b60",
            base.byte_add(0x7f0).cast::<i32>(),
            write_unaligned,
            1
        );
        if mech_trace::rd!(
            "10032b60",
            base.byte_add(0x84f).cast::<i32>(),
            read_unaligned
        ) == 1
        {
            let p = base.byte_add(0x278).cast::<i32>();
            mech_trace::wr!(
                "10032b60",
                p,
                write_unaligned,
                mech_trace::rd!("10032b60", p, read_unaligned).wrapping_add(0x32)
            );
        }
        mech_trace::cr!("10032b60", "1000a890", FUN_1000a890(param_1));
        let mut i_var1: i32 = 0;
        'restart: loop {
            i_var1 = i_var1.wrapping_add(1);
            let mut local_4: i32 = 0;
            let mut local_8 = base.byte_add(0x368).cast::<i32>();
            loop {
                let mut piVar3 = local_8;
                let mut i_var2: i32 = 0;
                loop {
                    let v = mech_trace::rd!("10032b60", piVar3, read_unaligned);
                    let t = (v / 10).wrapping_mul(5).wrapping_sub(0x12a7) as u32;
                    if (t & 0x7fffffff) == 0 && v % 10 == i_var1 {
                        continue 'restart;
                    }
                    piVar3 = piVar3.byte_add(4);
                    i_var2 += 1;
                    if i_var2 >= 0xc {
                        break;
                    }
                }
                local_8 = local_8.byte_add(0x30);
                local_4 += 1;
                if 7 < local_4 {
                    return mech_trace::rt!("10032b60", param_2.wrapping_add(i_var1));
                }
            }
        }
    }
}
