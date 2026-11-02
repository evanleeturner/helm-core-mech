// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_10034390: u8;
    static mut LAB_10034440: u8;
    static mut LAB_10034540: u8;
    static mut LAB_10034640: u8;
    static mut LAB_10034690: u8;
    static mut LAB_10034710: u8;
    static mut LAB_10034780: u8;
    static mut LAB_100347e0: u8;
    static mut DAT_10083508: u32;
    static mut DAT_10098224: *mut core::ffi::c_void;
    static mut DAT_100a6744: u32;
    static mut DAT_100a67b8: i32;
    static mut DAT_100a67bc: i32;
    static mut DAT_100a67c0: i32;
    static mut DAT_100a67c4: i32;
    static mut DAT_100a67c8: i32;
    static mut DAT_100c49d0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017ff0(collection: i32) -> u8;
    fn FUN_1000cd20(collection: i32) -> i32;
    fn FUN_10018060(collection: i32, param_2: i32) -> u32;
    fn FUN_10050490(collection: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut u32;
}

#[inline(always)]
unsafe fn st(p: *mut u32, i: usize, v: u32) {
    // SAFETY: p points into the heap block allocated by the caller.
    unsafe { mech_trace::wr!("10033f40", p.add(i), write, v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10033f40(collection: i32) -> *mut u32 {
    mech_trace::enter!("10033f40", collection);
    if collection == 0 {
        return mech_trace::rt!("10033f40", core::ptr::null_mut());
    }
    // SAFETY: the callees and globals are the game's own; collection and its
    // items are the game's structures, read and written where the original does.
    unsafe {
        let cnt = (collection as usize + 4) as *const i32;
        mech_trace::cr!("10033f40", "10017ff0", FUN_10017ff0(collection));
        let mut same = mech_trace::cr!("10033f40", "1000cd20", FUN_1000cd20(collection));
        let arr = mech_trace::cr!(
            "10033f40",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("10033f40", "10098224", DAT_10098224),
                1,
                same.wrapping_mul(0xa8).wrapping_add(0x118) as u32,
            )
        );
        let c49 = (&raw mut DAT_100c49d0) as usize as u32;
        st(arr, 0, 0x1cc);
        let mut header: i32 = 1;
        st(arr, 1, 0x5c);
        st(arr, 2, 0xffffffff);
        st(arr, 3, 0xffffffff);
        st(arr, 4, 0);
        st(arr, 5, c49);
        st(arr, 6, 0);
        st(arr, 8, 0);
        st(arr, 9, 0xffffffff);
        st(arr, 10, 0xffffffff);
        st(arr, 0xb, 0xffffffff);
        if mech_trace::rd!("10033f40", cnt, read_unaligned) == 0 {
            st(arr, 7, (&raw mut LAB_10034780) as usize as u32);
        } else {
            st(arr, 7, (&raw mut LAB_10034640) as usize as u32);
            same = mech_trace::rd!(
                "10033f40",
                ((mech_trace::gr!("10033f40", "10083508", DAT_10083508) as usize + 0x23bb)
                    as *const i32),
                read_unaligned
            );
            st(arr, 0xe, 0x231);
            if same != 1 {
                st(arr, 0xe, 0x263);
            }
            st(arr, 0xf, 0x5c);
            st(arr, 0x10, 0xffffffff);
            st(arr, 0x11, 0xffffffff);
            st(arr, 0x12, 0);
            st(arr, 0x13, c49);
            st(arr, 0x14, 0);
            st(arr, 0x16, 0);
            header = 3;
            st(arr, 0x17, 0xffffffff);
            st(arr, 0x18, 0xffffffff);
            st(arr, 0x19, 0xffffffff);
            st(arr, 0x15, (&raw mut LAB_10034710) as usize as u32);
            st(arr, 0x1c, 0x263);
            st(arr, 0x1d, 0x5c);
            st(arr, 0x1e, 0xffffffff);
            st(arr, 0x1f, 0xffffffff);
            st(arr, 0x20, 0);
            st(arr, 0x21, c49);
            st(arr, 0x22, 0);
            st(arr, 0x24, 0);
            st(arr, 0x25, 0xffffffff);
            st(arr, 0x26, 0xffffffff);
            st(arr, 0x27, 0xffffffff);
            st(arr, 0x23, (&raw mut LAB_10034690) as usize as u32);
        }
        same = 0;
        if 0 < mech_trace::rd!("10033f40", cnt, read_unaligned) {
            loop {
                let next = same.wrapping_add(1);
                let it = mech_trace::cr!("10033f40", "10018060", FUN_10018060(collection, same));
                mech_trace::wr!("10033f40", ((it as usize + 9) as *mut u8), write, 0);
                same = next;
                if next >= mech_trace::rd!("10033f40", cnt, read_unaligned) {
                    break;
                }
            }
        }
        let mut added: i32 = 0;
        let mut item_index: i32 = 0;
        let mut row: i32 = header - 1;
        if 0 < mech_trace::rd!("10033f40", cnt, read_unaligned) {
            loop {
                let cur =
                    mech_trace::cr!("10033f40", "10018060", FUN_10018060(collection, item_index))
                        as usize;
                if mech_trace::rd!("10033f40", ((cur + 9) as *const u8), read) != 1 {
                    let mut same_key: i32 = 1;
                    mech_trace::wr!("10033f40", ((cur + 9) as *mut u8), write, 1);
                    let mut scan = item_index.wrapping_add(1);
                    let cur_key = mech_trace::rd!("10033f40", (cur as *const i32), read_unaligned);
                    if scan < mech_trace::rd!("10033f40", cnt, read_unaligned) {
                        loop {
                            let later = mech_trace::cr!(
                                "10033f40",
                                "10018060",
                                FUN_10018060(collection, scan)
                            ) as usize;
                            if mech_trace::rd!("10033f40", (later as *const i32), read_unaligned)
                                == cur_key
                            {
                                mech_trace::wr!("10033f40", ((later + 9) as *mut u8), write, 1);
                                same_key = same_key.wrapping_add(1);
                            }
                            scan = scan.wrapping_add(1);
                            if scan >= mech_trace::rd!("10033f40", cnt, read_unaligned) {
                                break;
                            }
                        }
                    }
                    if mech_trace::rd!(
                        "10033f40",
                        ((mech_trace::gr!("10033f40", "10083508", DAT_10083508) as usize + 0x23bb)
                            as *const i32),
                        read_unaligned
                    ) != 1
                    {
                        same_key = 99999999;
                    }
                    let y = |r: i32| -> u32 {
                        mech_trace::rd!(
                            "10033f40",
                            ((mech_trace::gr!("10033f40", "100a6744", DAT_100a6744) as usize
                                + 0x40c) as *const i32),
                            read_unaligned
                        )
                        .wrapping_mul(r)
                        .wrapping_add(0x5c) as u32
                    };
                    // entry 0
                    let o = arr.add(header.wrapping_add(added).wrapping_mul(0xe) as usize);
                    st(o, 0, 0x1cc);
                    st(o, 1, y(row));
                    st(o, 2, 0x9c);
                    st(o, 3, 0xffffffff);
                    st(o, 4, 0);
                    st(o, 5, c49);
                    st(o, 6, 0);
                    st(o, 7, (&raw mut LAB_10034390) as usize as u32);
                    st(o, 9, cur_key as u32);
                    st(o, 10, same_key as u32);
                    st(o, 0xb, 0xffffffff);
                    let f = mech_trace::cr!("10033f40", "10050490", FUN_10050490(cur as i32));
                    st(o, 0xc, f);
                    st(o, 0xd, 0xffffffff);
                    st(o, 8, (&raw mut LAB_100347e0) as usize as u32);
                    if mech_trace::gr!("10033f40", "100a67c4", DAT_100a67c4) == collection
                        || collection == mech_trace::gr!("10033f40", "100a67bc", DAT_100a67bc)
                        || mech_trace::gr!("10033f40", "100a67c0", DAT_100a67c0) == collection
                    {
                        st(o, 0xd, 0);
                    } else if collection == mech_trace::gr!("10033f40", "100a67b8", DAT_100a67b8) {
                        st(o, 0xd, 1);
                    } else if mech_trace::gr!("10033f40", "100a67c8", DAT_100a67c8) == collection {
                        st(o, 0xd, 2);
                    }
                    // entry 1
                    let o = arr
                        .add(header.wrapping_add(added).wrapping_add(1).wrapping_mul(0xe) as usize);
                    let m = mech_trace::rd!(
                        "10033f40",
                        ((mech_trace::gr!("10033f40", "10083508", DAT_10083508) as usize + 0x23bb)
                            as *const i32),
                        read_unaligned
                    );
                    st(o, 0, 0x231);
                    if m != 1 {
                        st(o, 0, 0x263);
                    }
                    st(o, 1, y(row));
                    st(o, 2, 0xffffffff);
                    st(o, 3, 0xffffffff);
                    st(o, 4, 0);
                    st(o, 5, c49);
                    st(o, 6, 0);
                    st(o, 7, (&raw mut LAB_10034440) as usize as u32);
                    st(o, 9, cur_key as u32);
                    st(o, 10, same_key as u32);
                    st(o, 0xb, 0xffffffff);
                    let f = mech_trace::cr!("10033f40", "10050490", FUN_10050490(cur as i32));
                    st(o, 0xc, f);
                    st(o, 0xd, 0xffffffff);
                    st(o, 8, 0);
                    // entry 2
                    let o = arr
                        .add(header.wrapping_add(added).wrapping_add(2).wrapping_mul(0xe) as usize);
                    st(o, 0, 0x263);
                    st(o, 1, y(row));
                    st(o, 2, 0xffffffff);
                    st(o, 3, 0xffffffff);
                    st(o, 4, 0);
                    st(o, 5, c49);
                    st(o, 6, 0);
                    st(o, 7, (&raw mut LAB_10034540) as usize as u32);
                    st(o, 9, cur_key as u32);
                    st(o, 10, same_key as u32);
                    st(o, 0xb, 0xffffffff);
                    let f = mech_trace::cr!("10033f40", "10050490", FUN_10050490(cur as i32));
                    added = added.wrapping_add(3);
                    st(o, 0xc, f);
                    st(o, 0xd, 0xffffffff);
                    st(o, 8, 0);
                    row = row.wrapping_add(1);
                }
                item_index = item_index.wrapping_add(1);
                if item_index >= mech_trace::rd!("10033f40", cnt, read_unaligned) {
                    break;
                }
            }
        }
        st(
            arr,
            header.wrapping_add(added).wrapping_mul(0xe) as usize,
            0xffffffff,
        );
        mech_trace::rt!("10033f40", arr)
    }
}
