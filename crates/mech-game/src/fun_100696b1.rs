// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eeed0: i32;
    static mut DAT_100eeed4: i32;
    static mut DAT_100eeed8: i32;
    static mut DAT_100eef40: i32;
    static mut DAT_101600ec: i32;
    static mut DAT_100b39b0: i32;
    static mut DAT_100b39c0: i32;
    static mut DAT_100ef03c: i32;
    static mut DAT_100c7fb0: i32;
    static mut DAT_100b39a8: i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100696b1(param_1: u32) -> u32 {
    mech_trace::enter!("100696b1", param_1);
    let mut param_1 = param_1;
    let mut local_c: i32 = 0;
    // SAFETY: all globals are the game's own memory; the computed addresses
    // point into the game's tables exactly as the original does.
    unsafe {
        if mech_trace::gr!("100696b1", "100eeed0", DAT_100eeed0) != 0 {
            let c = param_1 as u8;
            if c == 0 || c == 2 || c == 4 || c == 6 || c == 8 || c == 10 {
                if mech_trace::gr!("100696b1", "100eeed4", DAT_100eeed4) != 0x100 {
                    return mech_trace::rt!("100696b1", param_1);
                }
                let ent = mech_trace::rd!(
                    "100696b1",
                    (&raw mut DAT_100eef40).add(mech_trace::gr!(
                        "100696b1",
                        "100eeed8",
                        DAT_100eeed8
                    ) as usize),
                    read_unaligned
                );
                let idx = mech_trace::rd!(
                    "100696b1",
                    ((ent as usize).wrapping_add(8) as *const i32),
                    read_unaligned
                );
                local_c = mech_trace::rd!(
                    "100696b1",
                    (&raw mut DAT_101600ec)
                        .cast::<u8>()
                        .wrapping_offset(idx.wrapping_mul(0x3c) as isize)
                        .cast::<i32>(),
                    read_unaligned
                );
                if local_c < 1 {
                    local_c = 0;
                } else if 7 < local_c {
                    local_c = 7;
                }
                if local_c != 1 && local_c != 0 {
                    let i = mech_trace::rd!(
                        "100696b1",
                        (&raw mut DAT_100b39c0)
                            .cast::<u8>()
                            .wrapping_offset(local_c.wrapping_mul(4) as isize)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    let b = mech_trace::rd!(
                        "100696b1",
                        (&raw mut DAT_100b39b0)
                            .cast::<u8>()
                            .wrapping_offset(i as isize),
                        read
                    );
                    return mech_trace::rt!(
                        "100696b1",
                        (u32::from(b) | (param_1 & 0xff00)).wrapping_add((param_1 & 0xfe) >> 1)
                    );
                }
                return mech_trace::rt!("100696b1", param_1.wrapping_add(local_c as u32));
            }
            if c == b'?' {
                if mech_trace::gr!("100696b1", "100eeed4", DAT_100eeed4) == 0x100 {
                    let ent = mech_trace::rd!(
                        "100696b1",
                        (&raw mut DAT_100eef40).add(mech_trace::gr!(
                            "100696b1",
                            "100eeed8",
                            DAT_100eeed8
                        ) as usize),
                        read_unaligned
                    );
                    let idx = mech_trace::rd!(
                        "100696b1",
                        ((ent as usize).wrapping_add(8) as *const i32),
                        read_unaligned
                    );
                    local_c = mech_trace::rd!(
                        "100696b1",
                        (&raw mut DAT_101600ec)
                            .cast::<u8>()
                            .wrapping_offset(idx.wrapping_mul(0x3c) as isize)
                            .cast::<i32>(),
                        read_unaligned
                    );
                } else {
                    let v = mech_trace::rd!(
                        "100696b1",
                        (&raw mut DAT_100ef03c)
                            .cast::<u8>()
                            .wrapping_offset(
                                mech_trace::gr!("100696b1", "100eeed8", DAT_100eeed8)
                                    .wrapping_mul(0x40) as isize
                            )
                            .cast::<i32>(),
                        read_unaligned
                    );
                    if v != -1 {
                        local_c = v;
                    }
                }
                return mech_trace::rt!("100696b1", param_1.wrapping_add(local_c as u32));
            }
        }
        if (param_1 & 0x8000) != 0 {
            param_1 &= 0x7fff;
            if mech_trace::gr!("100696b1", "100c7fb0", DAT_100c7fb0) != 0
                && param_1 <= mech_trace::gr!("100696b1", "100b39a8", DAT_100b39a8) as u32
            {
                param_1 = mech_trace::rd!(
                    "100696b1",
                    ((mech_trace::gr!("100696b1", "100c7fb0", DAT_100c7fb0) as u32)
                        .wrapping_add(param_1.wrapping_mul(4)) as usize
                        as *const u32),
                    read_unaligned
                );
            }
        }
        mech_trace::rt!("100696b1", param_1)
    }
}
