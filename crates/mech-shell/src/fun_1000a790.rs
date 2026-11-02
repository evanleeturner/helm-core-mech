// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10084c58: u8;
    static mut DAT_10084c5c: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000a790(record: i32) {
    mech_trace::enter!("1000a790", record);
    let rec = record as usize as *mut u8;
    // SAFETY: the record and the code table are the game's own memory;
    // reads and writes happen at the same offsets and order as the original.
    unsafe {
        let selector_code = mech_trace::rd!(
            "1000a790",
            rec.byte_add(0x20c).cast::<i32>(),
            read_unaligned
        );
        let off = (selector_code % 10000).wrapping_mul(0x10);
        let a = (&raw mut DAT_10084c58)
            .cast::<u8>()
            .wrapping_byte_offset(off as isize);
        mech_trace::wr!(
            "1000a790",
            rec.byte_add(0x210).cast::<i32>(),
            write_unaligned,
            mech_trace::rd!("1000a790", a.cast::<i32>(), read_unaligned)
        );
        let b = (&raw mut DAT_10084c5c)
            .cast::<u8>()
            .wrapping_byte_offset(off as isize);
        let mut i_var2 = mech_trace::rd!("1000a790", b.cast::<i32>(), read_unaligned);
        mech_trace::wr!(
            "1000a790",
            rec.byte_add(0x214).cast::<i32>(),
            write_unaligned,
            i_var2
        );
        if 9999 < selector_code {
            i_var2 /= 2;
            mech_trace::wr!(
                "1000a790",
                rec.byte_add(0x214).cast::<i32>(),
                write_unaligned,
                i_var2
            );
        }
        let flag = ((selector_code < 10000) as u32).wrapping_sub(1) & 7;
        mech_trace::wr!(
            "1000a790",
            rec.byte_add(0x218).cast::<u32>(),
            write_unaligned,
            flag
        );
        let r200 = mech_trace::rd!(
            "1000a790",
            rec.byte_add(0x200).cast::<i32>(),
            read_unaligned
        );
        let r214 = mech_trace::rd!(
            "1000a790",
            rec.byte_add(0x214).cast::<i32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1000a790",
            rec.byte_add(0x21c).cast::<i32>(),
            write_unaligned,
            r200.wrapping_sub(r214)
        );
    }
}
