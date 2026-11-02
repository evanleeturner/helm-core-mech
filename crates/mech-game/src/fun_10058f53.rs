// SPDX-License-Identifier: MIT

#[inline(always)]
unsafe fn rd(p: u32, i: usize) -> i32 {
    // SAFETY: caller supplies a valid pointer to at least i+1 ints.
    unsafe { mech_trace::rd!("10058f53", (p as *const i32).add(i), read_unaligned) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10058f53(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: *mut i32,
    param_5: i32,
    param_6: i32,
    param_7: u32,
) -> u32 {
    mech_trace::enter!(
        "10058f53", param_1, param_2, param_3, param_4, param_5, param_6, param_7
    );
    // SAFETY: all pointers are the caller's descriptors and surface memory,
    // accessed exactly where the original does.
    unsafe {
        let p1 = param_1 as u32;
        let p4 = param_4 as u32;
        let pivar1 = rd(p1, 0) as u32;
        let mut ivar25 = rd(pivar1, 1);
        let ivar9 = ivar25.wrapping_add(1);
        if ivar9 == 0 || ivar25 < -1 {
            return mech_trace::rt!("10058f53", 0xffff_ffff);
        }
        let mut ivar23 = rd(pivar1, 2);
        if ivar23 < 0 {
            return mech_trace::rt!("10058f53", 0xffff_ffff);
        }
        let ivar2 = rd(p1, 1);
        let mut ivar10 = ivar2;
        if ivar2 < 1 {
            ivar10 = 0;
        }
        let ivar3 = rd(p1, 2);
        let mut ivar11 = ivar3;
        if ivar3 < 1 {
            ivar11 = 0;
        }
        let mut ivar12 = rd(p1, 3);
        if ivar25 <= rd(p1, 3) {
            ivar12 = ivar25;
        }
        ivar25 = rd(p1, 4);
        if ivar23 <= rd(p1, 4) {
            ivar25 = ivar23;
        }
        if ivar12 < ivar10 || ivar25 < ivar11 {
            return mech_trace::rt!("10058f53", 0xffff_fffe);
        }
        ivar10 = ivar10.wrapping_sub(ivar2);
        ivar12 = ivar12.wrapping_sub(ivar2);
        ivar11 = ivar11.wrapping_sub(ivar3);
        ivar25 = ivar25.wrapping_sub(ivar3);
        let pivar4 = rd(p4, 0) as u32;
        ivar23 = rd(pivar4, 1);
        let ivar13 = ivar23.wrapping_add(1);
        if ivar13 == 0 || ivar23 < -1 {
            return mech_trace::rt!("10058f53", 0xffff_ffff);
        }
        let mut ivar18 = rd(pivar4, 2);
        if ivar18 < 0 {
            return mech_trace::rt!("10058f53", 0xffff_ffff);
        }
        let ivar5 = rd(p4, 1);
        let mut ivar14 = ivar5;
        if ivar5 < 1 {
            ivar14 = 0;
        }
        let ivar6 = rd(p4, 2);
        let mut ivar15 = ivar6;
        if ivar6 < 1 {
            ivar15 = 0;
        }
        let mut ivar22 = rd(p4, 3);
        if ivar23 <= rd(p4, 3) {
            ivar22 = ivar23;
        }
        ivar23 = rd(p4, 4);
        if ivar18 <= rd(p4, 4) {
            ivar23 = ivar18;
        }
        if ivar14 <= ivar22 && ivar15 <= ivar23 {
            ivar14 = ivar14.wrapping_sub(ivar5);
            ivar22 = ivar22.wrapping_sub(ivar5);
            ivar15 = ivar15.wrapping_sub(ivar6);
            ivar23 = ivar23.wrapping_sub(ivar6);
            let ivar16 = param_2.wrapping_sub(param_5);
            let ivar17 = param_3.wrapping_sub(param_6);
            ivar18 = ivar10;
            if ivar10 <= ivar14.wrapping_add(ivar16) {
                ivar18 = ivar14.wrapping_add(ivar16);
            }
            let mut ivar19 = ivar11;
            if ivar11 <= ivar15.wrapping_add(ivar17) {
                ivar19 = ivar15.wrapping_add(ivar17);
            }
            let mut ivar20 = ivar12;
            if ivar22.wrapping_add(ivar16) <= ivar12 {
                ivar20 = ivar22.wrapping_add(ivar16);
            }
            let mut ivar21 = ivar25;
            if ivar23.wrapping_add(ivar17) <= ivar25 {
                ivar21 = ivar23.wrapping_add(ivar17);
            }
            if ivar18 <= ivar20 && ivar19 <= ivar21 {
                if ivar14 <= ivar10.wrapping_sub(ivar16) {
                    ivar14 = ivar10.wrapping_sub(ivar16);
                }
                if ivar15 <= ivar11.wrapping_sub(ivar17) {
                    ivar15 = ivar11.wrapping_sub(ivar17);
                }
                if ivar12.wrapping_sub(ivar16) <= ivar22 {
                    ivar22 = ivar12.wrapping_sub(ivar16);
                }
                if ivar25.wrapping_sub(ivar17) <= ivar23 {
                    ivar23 = ivar25.wrapping_sub(ivar17);
                }
                let uvar24 = (ivar20 as u32).wrapping_add(1).wrapping_sub(ivar18 as u32);
                ivar25 = (ivar21 as u32).wrapping_add(1).wrapping_sub(ivar19 as u32) as i32;
                let mut local_54 = ivar13;
                let mut local_40 = ivar9;
                if ivar19 <= ivar15 {
                    local_54 = 0i32.wrapping_sub(ivar13);
                    local_40 = 0i32.wrapping_sub(ivar9);
                    ivar19 = ivar21;
                    ivar15 = ivar23;
                }
                let bvar7: i32 = (ivar18 <= ivar14) as i32;
                let local_60: i32;
                let mut uvar26: u32;
                if bvar7 != 0 {
                    local_60 = 3;
                    uvar26 = uvar24;
                } else {
                    local_60 = 0;
                    ivar20 = ivar18;
                    ivar22 = ivar14;
                    uvar26 = 0u32.wrapping_sub(uvar24);
                }
                local_40 = (local_40 as u32).wrapping_add(uvar26) as i32;
                local_54 = (local_54 as u32).wrapping_add(uvar26) as i32;
                let mut puvar29: u32 = (ivar6 as u32)
                    .wrapping_mul(ivar13 as u32)
                    .wrapping_add(pivar4 as u32 * 0 + rd(p4, 0) as u32)
                    .wrapping_add(ivar5 as u32)
                    .wrapping_add((ivar15 as u32).wrapping_mul(ivar13 as u32))
                    .wrapping_add(ivar22 as u32);
                let mut puvar27: u32 = (ivar3 as u32)
                    .wrapping_mul(ivar9 as u32)
                    .wrapping_add(rd(p1, 0) as u32)
                    .wrapping_add(ivar2 as u32)
                    .wrapping_add((ivar19 as u32).wrapping_mul(ivar9 as u32))
                    .wrapping_add(ivar20 as u32);
                let step: u32 = (bvar7 as u32).wrapping_mul(0xffff_fffe).wrapping_add(1);
                let step4: u32 = step.wrapping_mul(4);
                let l60 = local_60 as u32;
                if param_7 & 0xffff_ff00 == 0 {
                    let uvar8 = param_7 as u8;
                    loop {
                        uvar26 = uvar24 & 3;
                        while uvar26 != 0 {
                            mech_trace::wr!("10058f53", (puvar29 as *mut u8), write, uvar8);
                            puvar29 = puvar29.wrapping_add(step);
                            uvar26 = uvar26.wrapping_sub(1);
                        }
                        let mut puvar28 = puvar29.wrapping_sub(l60);
                        uvar26 = uvar24 >> 2;
                        while uvar26 != 0 {
                            mech_trace::wr!(
                                "10058f53",
                                (puvar28 as *mut u32),
                                write_unaligned,
                                (uvar8 as u32).wrapping_mul(0x0101_0101)
                            );
                            puvar28 = puvar28.wrapping_add(step4);
                            uvar26 = uvar26.wrapping_sub(1);
                        }
                        puvar29 = puvar28.wrapping_add(local_54 as u32).wrapping_add(l60);
                        ivar25 = ivar25.wrapping_sub(1);
                        if ivar25 == 0 {
                            break;
                        }
                    }
                } else {
                    loop {
                        uvar26 = uvar24 & 3;
                        while uvar26 != 0 {
                            mech_trace::wr!(
                                "10058f53",
                                (puvar29 as *mut u8),
                                write,
                                mech_trace::rd!("10058f53", (puvar27 as *const u8), read)
                            );
                            puvar27 = puvar27.wrapping_add(step);
                            puvar29 = puvar29.wrapping_add(step);
                            uvar26 = uvar26.wrapping_sub(1);
                        }
                        let mut puvar28 = puvar27.wrapping_sub(l60);
                        let mut puvar30 = puvar29.wrapping_sub(l60);
                        uvar26 = uvar24 >> 2;
                        while uvar26 != 0 {
                            mech_trace::wr!(
                                "10058f53",
                                (puvar30 as *mut u32),
                                write_unaligned,
                                mech_trace::rd!(
                                    "10058f53",
                                    (puvar28 as *const u32),
                                    read_unaligned
                                )
                            );
                            puvar28 = puvar28.wrapping_add(step4);
                            puvar30 = puvar30.wrapping_add(step4);
                            uvar26 = uvar26.wrapping_sub(1);
                        }
                        puvar27 = puvar28.wrapping_add(local_40 as u32).wrapping_add(l60);
                        puvar29 = puvar30.wrapping_add(local_54 as u32).wrapping_add(l60);
                        ivar25 = ivar25.wrapping_sub(1);
                        if ivar25 == 0 {
                            break;
                        }
                    }
                }
                return mech_trace::rt!("10058f53", 0);
            }
            return mech_trace::rt!("10058f53", 0xffff_fffd);
        }
        mech_trace::rt!("10058f53", 0xffff_fffe)
    }
}
