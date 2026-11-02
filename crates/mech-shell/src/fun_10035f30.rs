// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093b2c: *mut i8;
    static mut DAT_10083d8c: [*mut i8; 1];
    static mut DAT_10093b30: u32;
    static mut DAT_10093b34: u16;
}

unsafe extern "C" {
    fn __strnicmp(a: *const i8, b: *const i8, n: u32) -> i32;
    fn _strncpy(a: *mut i8, b: *const i8, n: u32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10035f30(
    slot_index: i32,
    key_text: *mut i8,
    first_text: *mut i8,
    second_text: *mut i8,
) -> u32 {
    mech_trace::enter!("10035f30", slot_index, key_text, first_text, second_text);
    let mut slot_index = slot_index;
    // SAFETY: the globals and the buffers are the game's own memory; every
    // read and write mirrors the original's accesses in the same order.
    unsafe {
        if slot_index < 0 {
            slot_index = mech_trace::rd!(
                "10035f30",
                mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                    .byte_add(4)
                    .cast::<i32>(),
                read_unaligned
            );
        } else {
            mech_trace::wr!(
                "10035f30",
                mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                    .byte_add(4)
                    .cast::<i32>(),
                write_unaligned,
                slot_index
            );
        }
        let slot_off = (slot_index as u32).wrapping_mul(0x40) as i32 as isize;
        if !first_text.is_null() {
            let mut len: u32 = 0;
            while mech_trace::dr!("10035f30", first_text.add(len as usize)) != 0 {
                len = len.wrapping_add(1);
            }
            len = len.wrapping_add(1);
            let dest = mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                .byte_offset(slot_off)
                .byte_add(0x28);
            let mut i = 0usize;
            while i < len as usize {
                mech_trace::dw!(
                    "10035f30",
                    dest.add(i),
                    mech_trace::dr!("10035f30", first_text.add(i))
                );
                i += 1;
            }
        }
        if !second_text.is_null() {
            let mut len: u32 = 0;
            while mech_trace::dr!("10035f30", second_text.add(len as usize)) != 0 {
                len = len.wrapping_add(1);
            }
            len = len.wrapping_add(1);
            let dest = mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                .byte_offset(slot_off)
                .byte_add(0x3e);
            let mut i = 0usize;
            while i < len as usize {
                mech_trace::dw!(
                    "10035f30",
                    dest.add(i),
                    mech_trace::dr!("10035f30", second_text.add(i))
                );
                i += 1;
            }
        }
        if !key_text.is_null() {
            let tbl = (&raw mut DAT_10083d8c).cast::<*mut i8>();
            let mut idx: i32 = 0;
            if !(mech_trace::dr!("10035f30", tbl)).is_null() {
                loop {
                    let r = mech_trace::cr!(
                        "10035f30",
                        "__strnicmp",
                        __strnicmp(
                            key_text,
                            mech_trace::dr!("10035f30", tbl.offset(idx.wrapping_mul(10) as isize)),
                            3
                        )
                    );
                    if r == 0 {
                        break;
                    }
                    idx = idx.wrapping_add(1);
                    if (mech_trace::dr!("10035f30", tbl.offset(idx.wrapping_mul(10) as isize)))
                        .is_null()
                    {
                        break;
                    }
                }
            }
            if (mech_trace::dr!("10035f30", tbl.offset(idx.wrapping_mul(10) as isize))).is_null() {
                if slot_index != 0 {
                    let c = mech_trace::rd!(
                        "10035f30",
                        mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                            .byte_add(0xc)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    if c.wrapping_sub(slot_index) == 1 {
                        mech_trace::wr!(
                            "10035f30",
                            mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                                .byte_add(0xc)
                                .cast::<i32>(),
                            write_unaligned,
                            c.wrapping_sub(1)
                        );
                        return mech_trace::rt!("10035f30", 0xffffffff);
                    }
                }
                let c = mech_trace::rd!(
                    "10035f30",
                    mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                        .byte_add(0xc)
                        .cast::<i32>(),
                    read_unaligned
                );
                return mech_trace::rt!("10035f30", (c <= slot_index) as u32);
            }
            let lim = mech_trace::rd!(
                "10035f30",
                tbl.cast::<u8>()
                    .byte_offset((idx as u32).wrapping_mul(0x28) as i32 as isize)
                    .byte_add(0xc)
                    .cast::<i32>(),
                read_unaligned
            );
            if mech_trace::rd!(
                "10035f30",
                mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                    .byte_add(0x10)
                    .cast::<i32>(),
                read_unaligned
            ) < lim
            {
                return mech_trace::rt!("10035f30", 0);
            }
            let dst = mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                .byte_offset(slot_off)
                .byte_add(0x18);
            mech_trace::cr!("10035f30", "_strncpy", _strncpy(dst, key_text, 8));
            mech_trace::dw!(
                "10035f30",
                mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                    .byte_offset(slot_off)
                    .byte_add(0x20),
                0
            );
            let mut klen: usize = 0;
            while mech_trace::dr!("10035f30", key_text.add(klen)) != 0 {
                klen += 1;
            }
            if klen == 3 {
                let dst = mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                    .byte_offset(slot_off)
                    .byte_add(0x18);
                let mut n: usize = 0;
                while mech_trace::dr!("10035f30", dst.add(n)) != 0 {
                    n += 1;
                }
                let term = dst.add(n);
                mech_trace::wr!(
                    "10035f30",
                    term.cast::<u32>(),
                    write_unaligned,
                    mech_trace::gr!("10035f30", "10093b30", DAT_10093b30)
                );
                mech_trace::wr!(
                    "10035f30",
                    term.add(4).cast::<u16>(),
                    write_unaligned,
                    mech_trace::gr!("10035f30", "10093b34", DAT_10093b34)
                );
            }
            mech_trace::wr!(
                "10035f30",
                mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                    .byte_offset(slot_off)
                    .byte_add(0x14)
                    .cast::<i32>(),
                write_unaligned,
                idx
            );
        }
        mech_trace::wr!(
            "10035f30",
            mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                .byte_add(0xc)
                .cast::<i32>(),
            write_unaligned,
            0
        );
        let mut i: i32 = 0;
        if 0 < mech_trace::rd!(
            "10035f30",
            mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                .byte_add(8)
                .cast::<i32>(),
            read_unaligned
        ) {
            let mut off: i32 = 0;
            loop {
                let v = mech_trace::rd!(
                    "10035f30",
                    mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                        .byte_offset(off as isize)
                        .byte_add(0x14)
                        .cast::<i32>(),
                    read_unaligned
                );
                if -1 < v {
                    let c = mech_trace::rd!(
                        "10035f30",
                        mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                            .byte_add(0xc)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    mech_trace::wr!(
                        "10035f30",
                        mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                            .byte_add(0xc)
                            .cast::<i32>(),
                        write_unaligned,
                        c.wrapping_add(1)
                    );
                }
                off = off.wrapping_add(0x40);
                i = i.wrapping_add(1);
                if !(i < mech_trace::rd!(
                    "10035f30",
                    mech_trace::gr!("10035f30", "10093b2c", DAT_10093b2c)
                        .byte_add(8)
                        .cast::<i32>(),
                    read_unaligned
                )) {
                    break;
                }
            }
        }
        mech_trace::rt!("10035f30", 0xffffffff)
    }
}
