// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
    static mut DAT_100c6d08: *mut u16;
    static mut DAT_100c6d38: *mut u16;
    static mut DAT_100c6d50: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e051(param_1: *mut u16) -> u8;
    fn FUN_1003ddbc(param_1: *mut u16) -> u8;
    fn FUN_1003dfde(param_1: *mut u16) -> u8;
    fn FUN_1003df3a(param_1: *mut u16) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003e425(param_1: i32) {
    mech_trace::enter!("1003e425", param_1);
    // SAFETY: the globals, the callees and the list nodes are the game's own;
    // nodes are read where the original reads them, and the next pointer is
    // fetched before the node is released.
    unsafe {
        let mut local_10: *mut u16;
        if param_1 == 0 {
            if mech_trace::gr!("1003e425", "100aefc0", DAT_100aefc0) == 0 {
                local_10 = mech_trace::gr!("1003e425", "100c6d08", DAT_100c6d08);
                while !local_10.is_null() {
                    let next = mech_trace::rd!(
                        "1003e425",
                        local_10.add(4).cast::<*mut u16>(),
                        read_unaligned
                    );
                    if (mech_trace::rd!("1003e425", local_10.add(1), read_unaligned) as u8) & 0xf0
                        == 0xc0
                    {
                        mech_trace::cr!("1003e425", "1003dfde", FUN_1003dfde(local_10));
                        mech_trace::cr!("1003e425", "1003df3a", FUN_1003df3a(local_10));
                    }
                    local_10 = next;
                }
            } else {
                let mut local_8: i32 = 0;
                while local_8 < 0xc {
                    let mut local_c: i32 = 0;
                    while local_c < 0xc {
                        let base = (&raw mut DAT_100c6d50).cast::<u8>();
                        local_10 = mech_trace::rd!(
                            "1003e425",
                            base.byte_add(
                                local_c
                                    .wrapping_mul(0x18)
                                    .wrapping_add(local_8.wrapping_mul(0x120))
                                    as usize,
                            )
                            .cast::<*mut u16>(),
                            read_unaligned
                        );
                        while !local_10.is_null() {
                            let next = mech_trace::rd!(
                                "1003e425",
                                local_10.add(4).cast::<*mut u16>(),
                                read_unaligned
                            );
                            if (mech_trace::rd!("1003e425", local_10.add(1), read_unaligned) as u8)
                                & 0xf0
                                == 0xc0
                            {
                                mech_trace::cr!("1003e425", "1003dfde", FUN_1003dfde(local_10));
                                mech_trace::cr!("1003e425", "1003df3a", FUN_1003df3a(local_10));
                            }
                            local_10 = next;
                        }
                        local_c = local_c.wrapping_add(1);
                    }
                    local_8 = local_8.wrapping_add(1);
                }
            }
        } else {
            local_10 = mech_trace::gr!("1003e425", "100c6d38", DAT_100c6d38);
            while !local_10.is_null() {
                let next = mech_trace::rd!(
                    "1003e425",
                    local_10.add(4).cast::<*mut u16>(),
                    read_unaligned
                );
                if (mech_trace::rd!("1003e425", local_10.add(1), read_unaligned) as u8) & 0xf0
                    == 0xc0
                {
                    mech_trace::cr!("1003e425", "1003e051", FUN_1003e051(local_10));
                    mech_trace::cr!("1003e425", "1003ddbc", FUN_1003ddbc(local_10));
                }
                local_10 = next;
            }
        }
    }
}
