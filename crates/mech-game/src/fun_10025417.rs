// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100258ea(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: *mut i32,
        param_8: *mut i32,
    ) -> u32;
    fn FUN_10025799(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
        param_7: u32,
    ) -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10025417(param_1: *mut i32, param_2: u8, param_3: i32) -> *mut u32 {
    mech_trace::enter!("10025417", param_1, param_2, param_3);
    let mut local_70: [i32; 25] = [0; 25];
    let mut local_78: i32 = 0;
    let mut local_94: i32 = 0;
    let local_7c: i32;
    let local_98: i32;
    let local_84: i32;
    let local_a0: i32;

    // SAFETY: param_1 points at the caller's 6+ int rectangle/node record.
    unsafe {
        let p0 = mech_trace::rd!("10025417", param_1, read_unaligned);
        let p1 = mech_trace::rd!("10025417", param_1.add(1), read_unaligned);
        let p4 = mech_trace::rd!("10025417", param_1.add(4), read_unaligned);
        let p5 = mech_trace::rd!("10025417", param_1.add(5), read_unaligned);
        if (param_2 & 1) == 0 {
            local_7c = p0;
            local_98 = (p1.wrapping_add(p0)) >> 1;
        } else {
            local_7c = ((p1.wrapping_add(p0)) >> 1).wrapping_add(1);
            local_98 = p1;
        }
        if (param_2 & 2) == 0 {
            local_84 = p4;
            local_a0 = (p5.wrapping_add(p4)) >> 1;
        } else {
            local_84 = ((p5.wrapping_add(p4)) >> 1).wrapping_add(1);
            local_a0 = p5;
        }
    }

    mech_trace::rt!(
        "10025417",
        if local_7c < local_98 && local_84 < local_a0 {
            let mut local_88: u32 = 0;
            let mut local_80: i32 = 0x7fffffff;
            let mut local_9c: i32 = -0x7fffffff;
            let mut local_c: i32 = 0;
            // SAFETY: param_3 points at the game's data block; the callees and
            // returned records are the game's own.
            unsafe {
                while local_c
                    < mech_trace::rd!(
                        "10025417",
                        ((param_3.wrapping_add(6)) as *const i16),
                        read_unaligned
                    ) as i32
                {
                    let local_90: i32 = (mech_trace::rd!(
                        "10025417",
                        ((param_3.wrapping_add(8)) as *const i32),
                        read_unaligned
                    ))
                    .wrapping_add(param_3)
                    .wrapping_add(local_c.wrapping_mul(0x24));
                    let iVar3 = mech_trace::cr!(
                        "10025417",
                        "100258ea",
                        FUN_100258ea(
                            local_90,
                            param_3,
                            local_7c,
                            local_98,
                            local_84,
                            local_a0,
                            &raw mut local_78,
                            &raw mut local_94,
                        )
                    );
                    if iVar3 != 0 {
                        let uVar1 = local_88.wrapping_add(1);
                        if 0x19 < uVar1 as i32 {
                            local_80 = 0x7fffffff;
                            local_9c = -0x7fffffff;
                            local_88 = uVar1;
                            let local_74 = mech_trace::cr!(
                                "10025417",
                                "10025799",
                                FUN_10025799(
                                    local_7c as u32,
                                    local_98 as u32,
                                    0x7fffffff,
                                    0x80000001,
                                    local_84 as u32,
                                    local_a0 as u32,
                                    0,
                                )
                            ) as *mut i32;
                            if local_74.is_null() {
                                return mech_trace::rt!("10025417", core::ptr::null_mut());
                            }
                            let mut local_8: i32 = 0;
                            while local_8 < 4 {
                                let puVar2 = FUN_10025417(local_74, local_8 as u8, param_3);
                                mech_trace::wr!(
                                    "10025417",
                                    local_74.add((local_8 + 7) as usize),
                                    write_unaligned,
                                    puVar2 as usize as i32
                                );
                                let local_8c = mech_trace::rd!(
                                    "10025417",
                                    local_74.add((local_8 + 7) as usize),
                                    read_unaligned
                                );
                                if local_8c != 0 {
                                    let a = mech_trace::rd!(
                                        "10025417",
                                        (local_8c.wrapping_add(0xc) as *const i32),
                                        read_unaligned
                                    );
                                    if local_9c < a {
                                        local_9c = a;
                                    }
                                    let b = mech_trace::rd!(
                                        "10025417",
                                        (local_8c.wrapping_add(8) as *const i32),
                                        read_unaligned
                                    );
                                    if b < local_80 {
                                        local_80 = b;
                                    }
                                }
                                local_8 += 1;
                            }
                            mech_trace::wr!("10025417", local_74.add(3), write_unaligned, local_9c);
                            mech_trace::wr!("10025417", local_74.add(2), write_unaligned, local_80);
                            if mech_trace::rd!("10025417", local_74.add(2), read_unaligned)
                                != 0x7fffffff
                            {
                                return mech_trace::rt!("10025417", local_74 as *mut u32);
                            }
                            mech_trace::wr!("10025417", local_74.add(3), write_unaligned, 0);
                            let v = mech_trace::rd!("10025417", local_74.add(3), read_unaligned);
                            mech_trace::wr!("10025417", local_74.add(2), write_unaligned, v);
                            return mech_trace::rt!("10025417", local_74 as *mut u32);
                        }
                        if local_9c < local_94 {
                            local_9c = local_94;
                        }
                        if local_78 < local_80 {
                            local_80 = local_78;
                        }
                        local_70[local_88 as usize] = local_90;
                        local_88 = uVar1;
                    }
                    local_c += 1;
                }
                if local_88 == 0 {
                    local_9c = 0;
                    local_80 = 0;
                }
                let puVar2 = mech_trace::cr!(
                    "10025417",
                    "10025799",
                    FUN_10025799(
                        local_7c as u32,
                        local_98 as u32,
                        local_80 as u32,
                        local_9c as u32,
                        local_84 as u32,
                        local_a0 as u32,
                        local_88,
                    )
                );
                if puVar2.is_null() {
                    return mech_trace::rt!("10025417", core::ptr::null_mut());
                } else if 0 < local_88 as i32 {
                    let dst = puVar2.add(0xb) as *mut i32;
                    let mut n = local_88 & 0x3fffffff;
                    let mut i = 0usize;
                    while n != 0 {
                        mech_trace::wr!("10025417", dst.add(i), write_unaligned, local_70[i]);
                        i += 1;
                        n -= 1;
                    }
                }
                puVar2
            }
        } else {
            core::ptr::null_mut()
        }
    )
}
