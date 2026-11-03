// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6780: u8;
    static mut DAT_100a71e8: u8;
    static mut DAT_100a71f8: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1006a267(use_path_prefix: i32) -> u32;
    fn FUN_1006a0e7(use_path_prefix: i32, param_2: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FILE_read_8(param_1: u32, param_2: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1005f410(this: *mut u8, use_path_prefix: i32) -> u32 {
    mech_trace::enter!("1005f410", this, use_path_prefix);
    let mut shp_path = [0i8; 32];
    // SAFETY: the globals and callees are the game's own; `this` points at the
    // caller's shp object, whose fields are read and written where the original does.
    unsafe {
        if use_path_prefix == 1 && mech_trace::gr!("1005f410", "100a6780", DAT_100a6780) != 0 {
            mech_trace::cr!(
                "1005f410",
                "sprintf",
                sprintf(
                    shp_path.as_mut_ptr(),
                    (&raw const DAT_100a71e8).cast::<i8>(),
                    &raw mut DAT_100a6780,
                    this,
                )
            );
        } else {
            mech_trace::cr!(
                "1005f410",
                "sprintf",
                sprintf(
                    shp_path.as_mut_ptr(),
                    (&raw const DAT_100a71f8).cast::<i8>(),
                    this,
                )
            );
        }
        let i_var1 = mech_trace::cr!(
            "1005f410",
            "FILE_read_8",
            FILE_read_8(shp_path.as_ptr() as u32, 0)
        ) as i32;
        mech_trace::wr!(
            "1005f410",
            this.byte_add(0x104).cast::<i32>(),
            write_unaligned,
            i_var1
        );
        if i_var1 == 0 {
            return mech_trace::rt!("1005f410", 0);
        }
        let mut max_high: i32 = -1;
        let mut max_low: u32 = 0xffff_ffff;
        let count = mech_trace::cr!("1005f410", "1006a267", FUN_1006a267(i_var1)) as i32;
        let mut entry_index: i32 = 0;
        mech_trace::wr!(
            "1005f410",
            this.byte_add(0x130).cast::<i32>(),
            write_unaligned,
            count
        );
        if 0 < count {
            loop {
                let u_var2 = mech_trace::cr!(
                    "1005f410",
                    "1006a0e7",
                    FUN_1006a0e7(
                        mech_trace::rd!(
                            "1005f410",
                            this.byte_add(0x104).cast::<i32>(),
                            read_unaligned
                        ),
                        entry_index,
                    )
                );
                if max_high < (u_var2 as i32) >> 0x10 {
                    max_high = (u_var2 as i32) >> 0x10;
                }
                if (max_low as i32) < (u_var2 & 0xffff) as i32 {
                    max_low = u_var2 & 0xffff;
                }
                entry_index = entry_index.wrapping_add(1);
                if !(entry_index
                    < mech_trace::rd!(
                        "1005f410",
                        this.byte_add(0x130).cast::<i32>(),
                        read_unaligned
                    ))
                {
                    break;
                }
            }
        }
        mech_trace::wr!(
            "1005f410",
            this.byte_add(0x118).cast::<i32>(),
            write_unaligned,
            max_high.wrapping_add(1)
        );
        mech_trace::wr!(
            "1005f410",
            this.byte_add(0x11c).cast::<u32>(),
            write_unaligned,
            max_low.wrapping_add(1)
        );
        if mech_trace::rd!(
            "1005f410",
            this.byte_add(0x10c).cast::<u32>(),
            read_unaligned
        ) & 0x80
            != 0
        {
            let v = mech_trace::rd!(
                "1005f410",
                this.byte_add(0x110).cast::<i32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "1005f410",
                this.byte_add(0x110).cast::<i32>(),
                write_unaligned,
                v.wrapping_sub(max_high.wrapping_add(1) / 2)
            );
            let w = mech_trace::rd!(
                "1005f410",
                this.byte_add(0x114).cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "1005f410",
                this.byte_add(0x114).cast::<u32>(),
                write_unaligned,
                w.wrapping_sub(max_low.wrapping_add(1))
            );
        }
        mech_trace::wr!(
            "1005f410",
            this.byte_add(0x12c).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1005f410",
            this.byte_add(0x128).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1005f410",
            this.byte_add(0x108).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1005f410",
            this.byte_add(0x138).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::rt!("1005f410", 1)
    }
}
