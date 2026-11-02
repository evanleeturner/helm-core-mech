// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001b971(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
        param_8: i32,
        param_9: i32,
        param_10: *mut i32,
        param_11: *mut i32,
        param_12: *mut i32,
    ) -> i32;
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001cfba(param_1: i32, param_2: i32) {
    mech_trace::enter!("1001cfba", param_1, param_2);
    let p = param_1 as usize as *mut u8;
    let mut local_c: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_24: i32 = 0;
    let mut local_20: *mut i32 = core::ptr::null_mut();
    let mut local_30: *mut i32 = core::ptr::null_mut();

    // SAFETY: param_1 points at the game's polygon record and param_2 at its
    // vertex array; all reads and writes mirror the original's accesses.
    unsafe {
        let set = |a: i32, b: i32, c: i32| {
            mech_trace::wr!(
                "1001cfba",
                p.byte_add(0x14).cast::<i32>(),
                write_unaligned,
                a
            );
            mech_trace::wr!("1001cfba", p.byte_add(8).cast::<i32>(), write_unaligned, a);
            mech_trace::wr!(
                "1001cfba",
                p.byte_add(0x18).cast::<i32>(),
                write_unaligned,
                b
            );
            mech_trace::wr!(
                "1001cfba",
                p.byte_add(0xc).cast::<i32>(),
                write_unaligned,
                b
            );
            mech_trace::wr!(
                "1001cfba",
                p.byte_add(0x1c).cast::<i32>(),
                write_unaligned,
                c
            );
            mech_trace::wr!(
                "1001cfba",
                p.byte_add(0x10).cast::<i32>(),
                write_unaligned,
                c
            );
        };
        let vert = |k: u32| -> *mut i32 {
            let base = mech_trace::rd!("1001cfba", p.byte_add(4).cast::<i32>(), read_unaligned);
            let addr = (base as u32).wrapping_add(param_1 as u32).wrapping_add(k);
            let b = mech_trace::rd!("1001cfba", (addr as usize as *const u8), read) as u32;
            b.wrapping_mul(0x44).wrapping_add(param_2 as u32) as usize as *mut i32
        };

        set(0x20000000, 0, 0);
        let count = mech_trace::rd!("1001cfba", p.byte_add(2).cast::<u16>(), read_unaligned);
        if 2 < count {
            if count == 3 {
                let p2 = vert(0);
                let p3 = vert(1);
                let p4 = vert(2);
                mech_trace::cr!(
                    "1001cfba",
                    "1001b971",
                    FUN_1001b971(
                        mech_trace::dr!("1001cfba", p2),
                        mech_trace::dr!("1001cfba", p2.add(1)),
                        mech_trace::dr!("1001cfba", p2.add(2)),
                        mech_trace::dr!("1001cfba", p3),
                        mech_trace::dr!("1001cfba", p3.add(1)),
                        mech_trace::dr!("1001cfba", p3.add(2)),
                        mech_trace::dr!("1001cfba", p4),
                        mech_trace::dr!("1001cfba", p4.add(1)),
                        mech_trace::dr!("1001cfba", p4.add(2)),
                        &raw mut local_c,
                        &raw mut local_14,
                        &raw mut local_18,
                    )
                );
                set(local_c, local_14, local_18);
            } else {
                let mut local_1c = vert(0);
                let mut local_28 = count as u32;
                while local_28 != 0 {
                    let local_10 = vert(local_28.wrapping_sub(1));
                    let local_8 = mech_trace::cr!(
                        "1001cfba",
                        "1000fc30",
                        FUN_1000fc30(
                            (mech_trace::dr!("1001cfba", local_1c))
                                .wrapping_sub(mech_trace::dr!("1001cfba", local_10)),
                            (mech_trace::dr!("1001cfba", local_1c.add(1)))
                                .wrapping_sub(mech_trace::dr!("1001cfba", local_10.add(1))),
                            (mech_trace::dr!("1001cfba", local_1c.add(2)))
                                .wrapping_sub(mech_trace::dr!("1001cfba", local_10.add(2))),
                        )
                    );
                    if local_24 < local_8 {
                        local_20 = local_10;
                        local_30 = local_1c;
                        local_24 = local_8;
                    }
                    local_1c = local_10;
                    local_28 = local_28.wrapping_sub(1);
                }
                if local_24 != 0 {
                    let mut local_34: i32 = -1;
                    local_28 = count as u32;
                    loop {
                        let mut local_10: *mut i32;
                        loop {
                            let u_var1 = local_28.wrapping_sub(1);
                            if local_28 == 0 {
                                return;
                            }
                            local_10 = vert(u_var1);
                            local_28 = u_var1;
                            if !(local_20 == local_10 || local_30 == local_10) {
                                break;
                            }
                        }
                        let i_var5 = mech_trace::cr!(
                            "1001cfba",
                            "1001b971",
                            FUN_1001b971(
                                mech_trace::dr!("1001cfba", local_20),
                                mech_trace::dr!("1001cfba", local_20.add(1)),
                                mech_trace::dr!("1001cfba", local_20.add(2)),
                                mech_trace::dr!("1001cfba", local_30),
                                mech_trace::dr!("1001cfba", local_30.add(1)),
                                mech_trace::dr!("1001cfba", local_30.add(2)),
                                mech_trace::dr!("1001cfba", local_10),
                                mech_trace::dr!("1001cfba", local_10.add(1)),
                                mech_trace::dr!("1001cfba", local_10.add(2)),
                                &raw mut local_c,
                                &raw mut local_14,
                                &raw mut local_18,
                            )
                        );
                        if local_34 < i_var5 {
                            set(local_c, local_14, local_18);
                            local_34 = i_var5;
                        }
                        if !(i_var5 < 0x11) {
                            break;
                        }
                    }
                }
            }
        }
    }
}
