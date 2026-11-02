// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7ba8: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10062fc4(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10062d5f(param_1: i32, param_2: i32) {
    mech_trace::enter!("10062d5f", param_1, param_2);
    let mut b_var1 = false;
    // SAFETY: param_1 points at the game's structure; the table and the
    // callee are the game's own memory, accessed where the original does.
    unsafe {
        let base = param_1 as usize as *const u8;
        if 0 < mech_trace::rd!(
            "10062d5f",
            base.byte_add(0x3dc).cast::<i32>(),
            read_unaligned
        ) {
            let mut local_8 = mech_trace::rd!(
                "10062d5f",
                base.byte_add(0x3e0).cast::<i32>(),
                read_unaligned
            );
            let mut local_1c: i32 = 1;
            while local_1c < 0xe {
                local_8 = local_8.wrapping_add(1) % 0xe;
                let arr = mech_trace::rd!(
                    "10062d5f",
                    base.byte_add(0x54).cast::<i32>(),
                    read_unaligned
                );
                let p = (arr.wrapping_add(local_8.wrapping_mul(0x40))) as usize as *const i32;
                let p1 = mech_trace::rd!("10062d5f", p.byte_add(4), read_unaligned);
                if p1 > -1
                    && mech_trace::rd!("10062d5f", p, read_unaligned) != -1
                    && mech_trace::rd!("10062d5f", p.byte_add(12), read_unaligned) == {
                        let cur = mech_trace::rd!(
                            "10062d5f",
                            base.byte_add(0x3e0).cast::<i32>(),
                            read_unaligned
                        );
                        let arr2 = mech_trace::rd!(
                            "10062d5f",
                            base.byte_add(0x54).cast::<i32>(),
                            read_unaligned
                        );
                        let addr = cur.wrapping_mul(0x40).wrapping_add(0xc).wrapping_add(arr2);
                        mech_trace::rd!("10062d5f", (addr as usize as *const i32), read_unaligned)
                    }
                    && mech_trace::rd!("10062d5f", p.byte_add(8), read_unaligned) != -1
                    && local_8
                        != mech_trace::rd!(
                            "10062d5f",
                            base.byte_add(0x3e0).cast::<i32>(),
                            read_unaligned
                        )
                {
                    let tbl = (&raw mut DAT_100a7ba8).cast::<u8>();
                    let flag = mech_trace::rd!(
                        "10062d5f",
                        tbl.byte_offset(p1.wrapping_mul(0x68) as isize),
                        read
                    );
                    if (flag & 4) != 0 {
                        b_var1 = true;
                        mech_trace::wr!(
                            "10062d5f",
                            (param_1 as usize as *mut u8).byte_add(0x3e0).cast::<i32>(),
                            write_unaligned,
                            local_8
                        );
                        break;
                    }
                }
                local_1c = local_1c.wrapping_add(1);
            }
            if !b_var1 && param_2 != 0 {
                mech_trace::cr!("10062d5f", "10062fc4", FUN_10062fc4(param_1));
            }
        }
    }
}
