// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aae84: u32;
    static mut DAT_100ada6c: u32;
    static mut DAT_100af17c: u32;
    static mut DAT_100b351c: u32;
    static mut DAT_100b352c: *mut u16;
    static mut DAT_10115100: u8;
    static mut DAT_10115160: u32;
    static mut DAT_101f52e0: u8;
    static mut DAT_100aaf68: u32;
    static mut DAT_100af100: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017bc1(param_1: i32, param_2: u32, param_3: i32, param_4: i32) -> u32;
    fn FUN_1008668b(param_1: i32, param_2: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1007b068(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: i32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
        param_7: u32,
        param_8: u32,
    ) -> u8;
    fn FUN_1003eb9c(param_1: *mut i32, param_2: i32, param_3: *mut u32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100317f8(param_1: i32, param_2: i32) {
    mech_trace::enter!("100317f8", param_1, param_2);
    let mut local_74c: [u32; 120] = [0; 120];
    let mut local_564: [u8; 1360] = [0; 1360];

    // SAFETY: all pointers come from the game's own structures and globals;
    // reads and writes happen in the original's order, and the local buffers
    // are sized for the game's maximum of 20 entries.
    unsafe {
        if param_2 != 0
            && (-1
                < mech_trace::rd!(
                    "100317f8",
                    ((param_2.wrapping_add(0x18)) as *const i32),
                    read_unaligned
                ))
        {
            mech_trace::gw!("100317f8", "10115160", DAT_10115160, 0);
            let mut local_14: u32 = mech_trace::rd!(
                "100317f8",
                ((param_2.wrapping_add(2)) as *const u16),
                read_unaligned
            ) as u32;
            let p20 = mech_trace::rd!(
                "100317f8",
                ((param_2.wrapping_add(0x20)) as *const i32),
                read_unaligned
            );
            let ivar1 = mech_trace::rd!(
                "100317f8",
                ((p20.wrapping_add(0x20)) as *const i32),
                read_unaligned
            );
            let mut local_56c: *mut u8 = local_564.as_mut_ptr();
            let mut local_568: u32 = 0;
            while local_568 < local_14 {
                let p4 = mech_trace::rd!(
                    "100317f8",
                    ((param_2.wrapping_add(4)) as *const i32),
                    read_unaligned
                );
                let idx = mech_trace::rd!(
                    "100317f8",
                    ((p4.wrapping_add(local_568 as i32).wrapping_add(param_2)) as *const u8),
                    read
                ) as u32;
                let ivar2 = (idx.wrapping_mul(0x44) as i32)
                    .wrapping_add(ivar1)
                    .wrapping_add(0x18);
                let v = mech_trace::rd!(
                    "100317f8",
                    ((iVar2_add(ivar2, 0xc)) as *const u32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "100317f8",
                    local_56c.byte_add(0xc).cast::<u32>(),
                    write_unaligned,
                    v
                );
                let v = mech_trace::rd!(
                    "100317f8",
                    ((iVar2_add(ivar2, 0x14)) as *const u32),
                    read_unaligned
                );
                mech_trace::wr!(
                    "100317f8",
                    local_56c.byte_add(0x14).cast::<u32>(),
                    write_unaligned,
                    v
                );
                let g = mech_trace::rd!(
                    "100317f8",
                    ((param_1.wrapping_add(0x18)) as *const i32),
                    read_unaligned
                );
                if g == 0 {
                    mech_trace::wr!(
                        "100317f8",
                        local_56c.byte_add(0x10).cast::<u32>(),
                        write_unaligned,
                        0
                    );
                } else {
                    mech_trace::wr!(
                        "100317f8",
                        local_56c.byte_add(0x10).cast::<i32>(),
                        write_unaligned,
                        g.wrapping_add(5)
                    );
                }
                mech_trace::wr!(
                    "100317f8",
                    local_56c.byte_add(0x18).cast::<u32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "100317f8",
                    local_56c.byte_add(0x1c).cast::<u32>(),
                    write_unaligned,
                    0
                );
                mech_trace::wr!(
                    "100317f8",
                    local_56c.byte_add(0x24).cast::<u32>(),
                    write_unaligned,
                    0
                );
                local_56c = local_56c.byte_add(0x44);
                local_568 = local_568.wrapping_add(1);
            }
            let tbl = mech_trace::gr!("100317f8", "100aaf68", DAT_100aaf68) as i32;
            let a = mech_trace::rd!(
                "100317f8",
                ((tbl.wrapping_add(0x3c)) as *const i32),
                read_unaligned
            );
            let b = mech_trace::rd!(
                "100317f8",
                ((tbl.wrapping_add(0x40)) as *const i32),
                read_unaligned
            );
            local_14 = mech_trace::cr!(
                "100317f8",
                "10017bc1",
                FUN_10017bc1(local_564.as_mut_ptr() as i32, local_14, a, b)
            );
            if 2 < local_14 {
                let mut local_10: *const i32 = (&raw mut DAT_10115100) as *const i32;
                let mut local_c: *mut u32 = local_74c.as_mut_ptr();
                local_568 = 0;
                while local_568 < local_14 {
                    let iv = mech_trace::rd!("100317f8", local_10, read_unaligned);
                    local_10 = local_10.wrapping_add(1);
                    let x = mech_trace::rd!(
                        "100317f8",
                        ((iv.wrapping_add(0xc)) as *const u32),
                        read_unaligned
                    );
                    mech_trace::wr!("100317f8", local_c, write_unaligned, x);
                    let y = mech_trace::rd!(
                        "100317f8",
                        ((iv.wrapping_add(0x10)) as *const u32),
                        read_unaligned
                    );
                    mech_trace::wr!("100317f8", local_c.add(1), write_unaligned, y);
                    mech_trace::wr!("100317f8", local_c.add(5), write_unaligned, 1);
                    mech_trace::wr!("100317f8", local_c.add(3), write_unaligned, 1);
                    mech_trace::wr!("100317f8", local_c.add(4), write_unaligned, 1);
                    mech_trace::wr!("100317f8", local_c.add(2), write_unaligned, 0xe00000);
                    local_c = local_c.wrapping_add(6);
                    local_568 = local_568.wrapping_add(1);
                }
                if mech_trace::gr!("100317f8", "100b352c", DAT_100b352c).is_null() {
                    mech_trace::gw!(
                        "100317f8",
                        "100b352c",
                        DAT_100b352c,
                        mech_trace::cr!(
                            "100317f8",
                            "1008668b",
                            FUN_1008668b(
                                mech_trace::gr!("100317f8", "100af17c", DAT_100af17c) as i32,
                                mech_trace::gr!("100317f8", "100b351c", DAT_100b351c),
                                mech_trace::gr!("100317f8", "100af100", DAT_100af100) as usize
                                    as *mut i32,
                            )
                        )
                    );
                }
                let local_8 = mech_trace::gr!("100317f8", "100b352c", DAT_100b352c).wrapping_add(
                    mech_trace::gr!("100317f8", "100ada6c", DAT_100ada6c).wrapping_mul(0x80)
                        as usize,
                );
                if mech_trace::gr!("100317f8", "100aae84", DAT_100aae84) == 0 {
                    mech_trace::cr!(
                        "100317f8",
                        "1003eb9c",
                        FUN_1003eb9c(
                            (&raw mut DAT_101f52e0) as *mut i32,
                            local_14 as i32,
                            local_74c.as_mut_ptr(),
                        )
                    );
                } else {
                    mech_trace::cr!(
                        "100317f8",
                        "1007b068",
                        FUN_1007b068(
                            (&raw mut DAT_101f52e0) as *mut i32,
                            local_74c.as_mut_ptr() as *mut i32,
                            local_14 as i32,
                            0x80,
                            0,
                            0,
                            local_8 as usize as u32,
                            0,
                        )
                    );
                }
            }
        }
    }
}

#[inline(always)]
fn iVar2_add(base: i32, off: i32) -> i32 {
    base.wrapping_add(off)
}
