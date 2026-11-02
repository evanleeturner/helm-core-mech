// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: i32;
    static mut DAT_10160c00: [i32; 1];
    static mut DAT_100aa540: i32;
    static mut DAT_100aa544: i32;
    static mut DAT_100aa548: i32;
    static mut DAT_100aa558: i32;
    static mut DAT_100aa55c: i32;
    static mut DAT_100aa560: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002634e(param_1: i32) -> i32;
    fn FUN_100145d4(param_1: i32, param_2: i32, param_3: i32, param_4: *mut i32) -> i32;
    fn FUN_1003d2d3(param_1: i32, param_2: i32);
    fn FUN_1001bc2d(param_1: i32, param_2: *mut i32) -> i32;
    fn FUN_100267fd(param_1: *mut i32, param_2: *mut i32);
    fn FUN_10014b90(param_1: i32, param_2: *mut i32, param_3: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10014991(param_1: *mut i32, param_2: *mut i32) -> i32 {
    mech_trace::enter!("10014991", param_1, param_2);
    let mut local_88: [i32; 14] = [0; 14];
    let mut local_48: [i32; 14] = [0; 14];
    let mut local_50: i32 = 0;
    let mut local_8c: i32 = 0;
    let mut local_c: i32 = 0x7fffffff;

    // SAFETY: the callees and globals are the game's own; param_1 and param_2
    // point at caller-provided buffers, and the list nodes are walked at the
    // offsets the original uses.
    unsafe {
        let iv = mech_trace::cr!("10014991", "1002634e", FUN_1002634e(param_1 as i32));
        if iv < 1 {
            return mech_trace::rt!(
                "10014991",
                mech_trace::cr!(
                    "10014991",
                    "100145d4",
                    FUN_100145d4(
                        mech_trace::rd!("10014991", param_1, read_unaligned),
                        mech_trace::rd!("10014991", param_1.add(1), read_unaligned),
                        mech_trace::rd!("10014991", param_1.add(2), read_unaligned),
                        param_2,
                    )
                )
            );
        }
        if mech_trace::gr!("10014991", "100af098", DAT_100af098) == 0 {
            return mech_trace::rt!("10014991", 0);
        }
        FUN_1003d2d3(
            mech_trace::rd!("10014991", param_1, read_unaligned),
            mech_trace::rd!("10014991", param_1.add(2), read_unaligned),
        );
        let tbl = (&raw mut DAT_10160c00).cast::<i32>();
        let mut local_10 = mech_trace::rd!("10014991", tbl, read_unaligned);
        while local_10 != 0 {
            local_8c = local_8c.wrapping_add(1);
            let mut local_90 = (local_10 as u32).wrapping_add(0x10) as *const i32;
            let mut node = mech_trace::rd!("10014991", local_90, read_unaligned);
            while node != 0 {
                local_90 = node as u32 as *const i32;
                let _ = local_90;
                let n = node as u32;
                let local_8 = mech_trace::rd!(
                    "10014991",
                    ((n.wrapping_add(2)) as *const u16),
                    read_unaligned
                );
                if (local_8 & 0x100) == 0
                    && mech_trace::rd!(
                        "10014991",
                        ((n.wrapping_add(0x24)) as *const i32),
                        read_unaligned
                    ) != 6
                {
                    let local_4c =
                        mech_trace::cr!("10014991", "1001bc2d", FUN_1001bc2d(node, param_1));
                    if local_4c < local_c {
                        FUN_100267fd(local_88.as_mut_ptr(), param_1);
                        let r = mech_trace::cr!(
                            "10014991",
                            "10014b90",
                            FUN_10014b90(node, local_88.as_mut_ptr(), local_4c)
                        );
                        if r != 0 {
                            local_c = mech_trace::cr!(
                                "10014991",
                                "1002634e",
                                FUN_1002634e(local_88.as_mut_ptr() as i32)
                            );
                            FUN_100267fd(local_48.as_mut_ptr(), local_88.as_mut_ptr());
                            local_50 = node;
                            mech_trace::gw!(
                                "10014991",
                                "100aa558",
                                DAT_100aa558,
                                mech_trace::gr!("10014991", "100aa540", DAT_100aa540)
                            );
                            mech_trace::gw!(
                                "10014991",
                                "100aa55c",
                                DAT_100aa55c,
                                mech_trace::gr!("10014991", "100aa544", DAT_100aa544)
                            );
                            mech_trace::gw!(
                                "10014991",
                                "100aa560",
                                DAT_100aa560,
                                mech_trace::gr!("10014991", "100aa548", DAT_100aa548)
                            );
                        }
                    }
                }
                node = mech_trace::rd!(
                    "10014991",
                    ((n.wrapping_add(0x10)) as *const i32),
                    read_unaligned
                );
            }
            local_10 = mech_trace::rd!("10014991", tbl.offset(local_8c as isize), read_unaligned);
        }
        mech_trace::wr!("10014991", param_2, write_unaligned, local_50);
        mech_trace::rt!(
            "10014991",
            if local_50 == 0 {
                0
            } else {
                FUN_100267fd(param_1, local_48.as_mut_ptr());
                1
            }
        )
    }
}
