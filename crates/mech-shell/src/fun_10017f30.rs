// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018020(param_1: i32) -> u8;
    fn FUN_100707a0(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: i32,
        param_5: *mut u8,
    ) -> u32;
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(export_name = "FUN_10017f30")]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FindCollectionItemIndex(collection: i32, mut item: i32) -> u32 {
    mech_trace::enter!("10017f30", collection, item);
    if collection == 0 {
        return mech_trace::rt!("10017f30", 0xffffffff);
    }
    // SAFETY: collection points at the game's own container structure; fields are
    // read at the original's byte offsets, and callees are the game's own.
    unsafe {
        let base = collection as usize as *const u8;
        if (mech_trace::rd!("10017f30", base.byte_add(0xc), read) & 2) == 0 {
            let mut u_var1: u32 = 0;
            if 0 < mech_trace::rd!("10017f30", base.byte_add(4).cast::<i32>(), read_unaligned) {
                loop {
                    let mut item_at_index: i32 = mech_trace::cr!(
                        "10017f30",
                        "10018060",
                        FUN_10018060(collection, u_var1 as i32)
                    ) as i32;
                    let mut local_4: i32 = item;
                    let cb = mech_trace::rd!(
                        "10017f30",
                        base.byte_add(0x19)
                            .cast::<Option<unsafe extern "C" fn(*mut i32, *mut i32) -> i32>>(),
                        read_unaligned
                    );
                    match cb {
                        None => {
                            if item_at_index == item {
                                return mech_trace::rt!("10017f30", u_var1);
                            }
                        }
                        Some(f) => {
                            let i_var2 = mech_trace::cr!(
                                "10017f30",
                                "ind",
                                f(&raw mut item_at_index, &raw mut local_4)
                            );
                            if i_var2 == 0 {
                                return mech_trace::rt!("10017f30", u_var1);
                            }
                        }
                    }
                    u_var1 = u_var1.wrapping_add(1);
                    if !((u_var1 as i32)
                        < mech_trace::rd!(
                            "10017f30",
                            base.byte_add(4).cast::<i32>(),
                            read_unaligned
                        ))
                    {
                        break;
                    }
                }
            }
        } else {
            mech_trace::cr!("10017f30", "10018020", FUN_10018020(collection));
            let u_var1 = mech_trace::cr!(
                "10017f30",
                "100707a0",
                FUN_100707a0(
                    (&raw mut item) as usize as u32,
                    mech_trace::rd!(
                        "10017f30",
                        base.byte_add(0x11).cast::<u32>(),
                        read_unaligned
                    ),
                    mech_trace::rd!("10017f30", base.byte_add(4).cast::<u32>(), read_unaligned),
                    4,
                    mech_trace::rd!(
                        "10017f30",
                        base.byte_add(0x19).cast::<*mut u8>(),
                        read_unaligned
                    ),
                )
            );
            if u_var1 != 0 {
                return mech_trace::rt!(
                    "10017f30",
                    u_var1.wrapping_sub(mech_trace::rd!(
                        "10017f30",
                        base.byte_add(0x11).cast::<i32>(),
                        read_unaligned
                    ) as u32)
                        >> 2
                );
            }
        }
    }
    mech_trace::rt!("10017f30", 0xffffffff)
}
