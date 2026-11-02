// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
    static mut DAT_100af098: u32;
    static mut DAT_100c6d5c: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003ddbc(param_1: *mut u16) {
    mech_trace::enter!("1003ddbc", param_1);
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the caller's record; the globals and the
    // linked records they point to are the game's own memory, accessed at
    // the same offsets and in the same order as the original.
    unsafe {
        let p = param_1.cast::<u8>();
        if (mech_trace::rd!("1003ddbc", param_1, read_unaligned) & 0x800) == 0 {
            return;
        }
        if mech_trace::rd!("1003ddbc", p.byte_add(0x24).cast::<i32>(), read_unaligned) == 4 {
            return;
        }
        mech_trace::wr!(
            "1003ddbc",
            param_1,
            write_unaligned,
            mech_trace::rd!("1003ddbc", param_1, read_unaligned) & 0xf7ff
        );
        if mech_trace::rd!("1003ddbc", p.byte_add(4).cast::<i32>(), read_unaligned) == 0 {
            return;
        }
        if mech_trace::gr!("1003ddbc", "100af098", DAT_100af098) == 0 {
            return;
        }
        let me = p as usize as u32;
        if mech_trace::gr!("1003ddbc", "100aefc0", DAT_100aefc0) == 0 {
            let g = mech_trace::gr!("1003ddbc", "100af098", DAT_100af098) as usize as *mut u8;
            let head = mech_trace::rd!("1003ddbc", g.byte_add(0x10).cast::<u32>(), read_unaligned);
            if head != 0 {
                mech_trace::wr!(
                    "1003ddbc",
                    (head as usize as *mut u8).byte_add(0xc).cast::<u32>(),
                    write_unaligned,
                    me
                );
            }
            mech_trace::wr!(
                "1003ddbc",
                p.byte_add(0x10).cast::<u32>(),
                write_unaligned,
                mech_trace::rd!("1003ddbc", g.byte_add(0x10).cast::<u32>(), read_unaligned)
            );
            mech_trace::wr!(
                "1003ddbc",
                g.byte_add(0x10).cast::<u32>(),
                write_unaligned,
                me
            );
            mech_trace::wr!(
                "1003ddbc",
                p.byte_add(0xc).cast::<u32>(),
                write_unaligned,
                mech_trace::gr!("1003ddbc", "100af098", DAT_100af098)
            );
        } else {
            let base = (&raw mut DAT_100c6d5c) as *mut u8;
            let idx = |q: *mut u8| -> usize {
                let a = mech_trace::rd!("1003ddbc", q.byte_add(0x48).cast::<i32>(), read_unaligned);
                let b = mech_trace::rd!("1003ddbc", q.byte_add(0x44).cast::<i32>(), read_unaligned);
                a.wrapping_mul(0x18).wrapping_add(b.wrapping_mul(0x120)) as usize
            };
            let entry = |q: *mut u8| -> *mut u8 {
                mech_trace::rd!(
                    "1003ddbc",
                    base.wrapping_add(idx(q)).cast::<u32>(),
                    read_unaligned
                ) as usize as *mut u8
            };
            let head = mech_trace::rd!(
                "1003ddbc",
                entry(p).byte_add(0x10).cast::<u32>(),
                read_unaligned
            );
            if head != 0 {
                mech_trace::wr!(
                    "1003ddbc",
                    (head as usize as *mut u8).byte_add(0xc).cast::<u32>(),
                    write_unaligned,
                    me
                );
            }
            let v = mech_trace::rd!(
                "1003ddbc",
                entry(p).byte_add(0x10).cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "1003ddbc",
                p.byte_add(0x10).cast::<u32>(),
                write_unaligned,
                v
            );
            mech_trace::wr!(
                "1003ddbc",
                entry(p).byte_add(0x10).cast::<u32>(),
                write_unaligned,
                me
            );
            let e = mech_trace::rd!(
                "1003ddbc",
                base.wrapping_add(idx(p)).cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "1003ddbc",
                p.byte_add(0xc).cast::<u32>(),
                write_unaligned,
                e
            );
        }
    }
}
