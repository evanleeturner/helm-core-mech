// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: u32;
    static mut DAT_100b681c: *mut u32;
    static mut DAT_100c96c0: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007d08d(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007cf3a() {
    mech_trace::enter!("1007cf3a");
    // SAFETY: the globals and the callee are the game's own; the list nodes
    // and the output table are read and written where the original does.
    unsafe {
        let mut local_10: i32 = 0;
        if !mech_trace::gr!("1007cf3a", "100b681c", DAT_100b681c).is_null() {
            let mut local_c: *mut u32 = mech_trace::gr!("1007cf3a", "100b681c", DAT_100b681c);
            while !local_c.is_null() {
                let e = mech_trace::rd!("1007cf3a", local_c.add(0xe), read_unaligned);
                if e != 0xFFFF_FFFF {
                    let nv = mech_trace::rd!("1007cf3a", local_c.add(0xf), read_unaligned)
                        .wrapping_add(mech_trace::gr!("1007cf3a", "100b3904", DAT_100b3904));
                    mech_trace::wr!("1007cf3a", local_c.add(0xf), write_unaligned, nv);
                    if (e as i32) < (nv as i32) {
                        let v = mech_trace::rd!("1007cf3a", local_c.add(3), read_unaligned);
                        mech_trace::wr!(
                            "1007cf3a",
                            (&raw mut DAT_100c96c0)
                                .cast::<u8>()
                                .byte_add((local_10 as usize).wrapping_mul(4))
                                .cast::<u32>(),
                            write_unaligned,
                            v
                        );
                        local_10 = local_10.wrapping_add(1);
                    }
                }
                local_c = mech_trace::rd!("1007cf3a", local_c, read_unaligned) as *mut u32;
            }
            if local_10 != 0 {
                let mut local_8: i32 = 0;
                while local_8 < local_10 {
                    let v = mech_trace::rd!(
                        "1007cf3a",
                        (&raw mut DAT_100c96c0)
                            .cast::<u8>()
                            .byte_add((local_8 as usize).wrapping_mul(4))
                            .cast::<i32>(),
                        read_unaligned
                    );
                    mech_trace::cr!("1007cf3a", "1007d08d", FUN_1007d08d(v));
                    local_8 = local_8.wrapping_add(1);
                }
            }
        }
    }
}
