// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f410(this: *mut u8, source_string: i32) -> u32;
    fn FUN_1005f530(this: *mut u8, source_string: i32) -> u32;
    fn FUN_1005ed50(this: *mut u8, source_string: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1005ee60(
    this: *mut u8,
    source_string: *const i8,
    param_2: u32,
    param_3: u32,
    flags: u32,
    param_5: u32,
    param_6: i32,
) -> *mut i8 {
    mech_trace::enter!(
        "1005ee60",
        this,
        source_string,
        param_2,
        param_3,
        flags,
        param_5,
        param_6
    );
    // SAFETY: the callees are the game's own; this and source_string point at
    // the caller's buffers, accessed where the original does, and param_6
    // points at the owner object.
    unsafe {
        let mut count: u32 = 0xffff_ffff;
        let mut src = source_string;
        loop {
            if count == 0 {
                break;
            }
            count = count.wrapping_sub(1);
            let c = mech_trace::dr!("1005ee60", src);
            src = src.add(1);
            if c == 0 {
                break;
            }
        }
        count = !count;
        let mut sc = src.byte_sub(count as usize) as *const u8;
        let mut dc = this;
        let mut dwords = count >> 2;
        while dwords != 0 {
            let v = mech_trace::rd!("1005ee60", sc.cast::<u32>(), read_unaligned);
            mech_trace::wr!("1005ee60", dc.cast::<u32>(), write_unaligned, v);
            sc = sc.add(4);
            dc = dc.add(4);
            dwords = dwords.wrapping_sub(1);
        }
        let mut rem = count & 3;
        while rem != 0 {
            mech_trace::dw!("1005ee60", dc, mech_trace::dr!("1005ee60", sc));
            sc = sc.add(1);
            dc = dc.add(1);
            rem = rem.wrapping_sub(1);
        }

        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x104).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x100).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x108).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x13c).cast::<i32>(),
            write_unaligned,
            param_6
        );
        let owner_field = mech_trace::rd!(
            "1005ee60",
            (param_6 as *const u8).byte_add(4).cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x140).cast::<u32>(),
            write_unaligned,
            owner_field
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x110).cast::<u32>(),
            write_unaligned,
            param_2
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x114).cast::<u32>(),
            write_unaligned,
            param_3
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x10c).cast::<u32>(),
            write_unaligned,
            (flags & 0xefff_ffff) | 0x100
        );
        let mut p5 = param_5;
        if p5 == 0 {
            p5 = 10;
        }
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x134).cast::<u32>(),
            write_unaligned,
            1000u32 / p5
        );

        let mut r = mech_trace::cr!("1005ee60", "1005f410", FUN_1005f410(this, 0));
        if (r as u8) == 0 {
            r = mech_trace::cr!("1005ee60", "1005f530", FUN_1005f530(this, 0));
            if (r as u8) == 0 {
                r = mech_trace::cr!("1005ee60", "1005f410", FUN_1005f410(this, 1));
                if (r as u8) == 0 {
                    r = mech_trace::cr!("1005ee60", "1005f530", FUN_1005f530(this, 1));
                    if (r as u8) == 0 {
                        mech_trace::wr!(
                            "1005ee60",
                            this.byte_add(0x10c).cast::<u32>(),
                            write_unaligned,
                            0x4000_0000
                        );
                    }
                }
            }
        }

        let v110 = mech_trace::rd!(
            "1005ee60",
            this.byte_add(0x110).cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x120).cast::<u32>(),
            write_unaligned,
            v110
        );
        let v114 = mech_trace::rd!(
            "1005ee60",
            this.byte_add(0x114).cast::<u32>(),
            read_unaligned
        );
        mech_trace::wr!(
            "1005ee60",
            this.byte_add(0x124).cast::<u32>(),
            write_unaligned,
            v114
        );
        if mech_trace::rd!(
            "1005ee60",
            this.byte_add(0x10c).cast::<i32>(),
            read_unaligned
        ) != 0x4000_0000
        {
            let owner = mech_trace::rd!(
                "1005ee60",
                this.byte_add(0x13c).cast::<u32>(),
                read_unaligned
            ) as *mut u8;
            FUN_1005ed50(owner, this as u32);
        }
        mech_trace::rt!("1005ee60", this as *mut i8)
    }
}
