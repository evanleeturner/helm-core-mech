// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b3904: i32;
    static mut DAT_100c8dc0: [i32; 0];
    static mut DAT_100c8990: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100885c8(param_1: *mut i32) -> i32 {
    mech_trace::enter!("100885c8", param_1);
    // SAFETY: param_1 and the structures it points to belong to the game;
    // every read and write happens where the original does it.
    unsafe {
        let i_var1: *mut u8 =
            mech_trace::rd!("100885c8", param_1.add(5), read_unaligned) as usize as *mut u8;
        let mut local_c: i32 =
            mech_trace::rd!("100885c8", i_var1.byte_add(0x15).cast::<i8>(), read) as i32;

        let p_a: *const u8 = mech_trace::rd!(
            "100885c8",
            i_var1.byte_add(0x2a).cast::<*const u8>(),
            read_unaligned
        );
        if p_a.is_null() || mech_trace::rd!("100885c8", p_a, read) == 0 {
            let p_b: *const u8 = mech_trace::rd!(
                "100885c8",
                i_var1.byte_add(0x2e).cast::<*const u8>(),
                read_unaligned
            );
            if !p_b.is_null() && mech_trace::rd!("100885c8", p_b, read) != 0 {
                let p_c: *mut i32 = mech_trace::rd!(
                    "100885c8",
                    i_var1.byte_add(4).cast::<*mut i32>(),
                    read_unaligned
                );
                mech_trace::wr!("100885c8", p_c, write_unaligned, 0);
                let l_var2 = mech_trace::cr!("100885c8", "__ftol", __ftol());
                let tmp = l_var2 as i32;
                mech_trace::wr!(
                    "100885c8",
                    i_var1.byte_add(0x1a).cast::<i32>(),
                    write_unaligned,
                    tmp
                );
                local_c = 1;
            }
        } else {
            if mech_trace::rd!("100885c8", param_1.add(2), read_unaligned) != 0 {
                let idx = mech_trace::rd!("100885c8", param_1, read_unaligned);
                let tbl = (&raw const DAT_100c8dc0).cast::<i32>();
                let base = mech_trace::rd!("100885c8", tbl.offset(idx as isize), read_unaligned);
                let fp_addr =
                    base.wrapping_add(0x10) as usize as *const unsafe extern "C" fn(i32, i32);
                let f = mech_trace::rd!("100885c8", fp_addr, read_unaligned);
                let a0_addr = (&raw const DAT_100c8990)
                    .cast::<u8>()
                    .byte_offset(idx.wrapping_mul(0x74) as isize)
                    .cast::<i32>();
                let a0 = mech_trace::rd!("100885c8", a0_addr, read_unaligned);
                mech_trace::cr!(
                    "100885c8",
                    "ind",
                    f(
                        a0,
                        mech_trace::rd!("100885c8", param_1.add(1), read_unaligned)
                    )
                );
            }
            let p_c: *mut i32 = mech_trace::rd!(
                "100885c8",
                i_var1.byte_add(4).cast::<*mut i32>(),
                read_unaligned
            );
            mech_trace::wr!("100885c8", p_c, write_unaligned, 0);
            let tmp = mech_trace::rd!(
                "100885c8",
                i_var1.byte_add(0x10).cast::<i32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "100885c8",
                i_var1.byte_add(0x1a).cast::<i32>(),
                write_unaligned,
                tmp
            );
            local_c = 1;
        }

        if mech_trace::rd!("100885c8", i_var1.byte_add(0x15).cast::<i8>(), read) == 0 {
            let p_d: *const u8 = mech_trace::rd!(
                "100885c8",
                i_var1.byte_add(0x22).cast::<*const u8>(),
                read_unaligned
            );
            if !p_d.is_null() && mech_trace::rd!("100885c8", p_d, read) != 0 {
                let p_c: *mut i32 = mech_trace::rd!(
                    "100885c8",
                    i_var1.byte_add(4).cast::<*mut i32>(),
                    read_unaligned
                );
                if mech_trace::rd!("100885c8", p_c, read_unaligned) < 0 {
                    mech_trace::wr!("100885c8", p_c, write_unaligned, 0);
                }
                let sh = (mech_trace::rd!("100885c8", i_var1.byte_add(0x1e), read) as u32) & 0x1f;
                let add = (mech_trace::gr!("100885c8", "100b3904", DAT_100b3904) as u32)
                    .wrapping_mul(3)
                    .wrapping_shl(sh);
                mech_trace::wr!(
                    "100885c8",
                    p_c,
                    write_unaligned,
                    (mech_trace::rd!("100885c8", p_c, read_unaligned) as u32).wrapping_add(add)
                        as i32
                );
                if 0x8000 < mech_trace::rd!("100885c8", p_c, read_unaligned) {
                    mech_trace::wr!("100885c8", p_c, write_unaligned, 0x8000);
                }
                let mut tmp = mech_trace::rd!(
                    "100885c8",
                    i_var1.byte_add(0x1a).cast::<i32>(),
                    read_unaligned
                );
                tmp = (tmp as u32)
                    .wrapping_add(mech_trace::rd!("100885c8", p_c, read_unaligned) as u32)
                    as i32;
                mech_trace::wr!(
                    "100885c8",
                    i_var1.byte_add(0x1a).cast::<i32>(),
                    write_unaligned,
                    tmp
                );
                if 0x10000 < tmp {
                    mech_trace::wr!(
                        "100885c8",
                        i_var1.byte_add(0x1a).cast::<i32>(),
                        write_unaligned,
                        0x10000
                    );
                }
                local_c = 1;
            }
            let p_d: *const u8 = mech_trace::rd!(
                "100885c8",
                i_var1.byte_add(0x26).cast::<*const u8>(),
                read_unaligned
            );
            if !p_d.is_null() && mech_trace::rd!("100885c8", p_d, read) != 0 {
                let p_c: *mut i32 = mech_trace::rd!(
                    "100885c8",
                    i_var1.byte_add(4).cast::<*mut i32>(),
                    read_unaligned
                );
                if 0 < mech_trace::rd!("100885c8", p_c, read_unaligned) {
                    mech_trace::wr!("100885c8", p_c, write_unaligned, 0);
                }
                let sh = (mech_trace::rd!("100885c8", i_var1.byte_add(0x1e), read) as u32) & 0x1f;
                let sub = (mech_trace::gr!("100885c8", "100b3904", DAT_100b3904) as u32)
                    .wrapping_mul(3)
                    .wrapping_shl(sh);
                mech_trace::wr!(
                    "100885c8",
                    p_c,
                    write_unaligned,
                    (mech_trace::rd!("100885c8", p_c, read_unaligned) as u32).wrapping_sub(sub)
                        as i32
                );
                if mech_trace::rd!("100885c8", p_c, read_unaligned) < -0x8000 {
                    mech_trace::wr!("100885c8", p_c, write_unaligned, -0x8000);
                }
                let mut tmp = mech_trace::rd!(
                    "100885c8",
                    i_var1.byte_add(0x1a).cast::<i32>(),
                    read_unaligned
                );
                tmp = (tmp as u32)
                    .wrapping_add(mech_trace::rd!("100885c8", p_c, read_unaligned) as u32)
                    as i32;
                mech_trace::wr!(
                    "100885c8",
                    i_var1.byte_add(0x1a).cast::<i32>(),
                    write_unaligned,
                    tmp
                );
                if tmp < -0x10000 {
                    mech_trace::wr!(
                        "100885c8",
                        i_var1.byte_add(0x1a).cast::<i32>(),
                        write_unaligned,
                        -0x10000
                    );
                }
                local_c = 1;
            }
            if local_c == 0 {
                let p_c: *mut i32 = mech_trace::rd!(
                    "100885c8",
                    i_var1.byte_add(4).cast::<*mut i32>(),
                    read_unaligned
                );
                mech_trace::wr!("100885c8", p_c, write_unaligned, 0);
            }
        }

        mech_trace::wr!(
            "100885c8",
            i_var1.byte_add(0x15).cast::<i8>(),
            write,
            local_c as i8
        );
        mech_trace::rt!("100885c8", local_c)
    }
}
