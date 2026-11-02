// SPDX-License-Identifier: MIT

#[inline(always)]
fn scarry4(a: u32, b: u32) -> bool {
    let s = a.wrapping_add(b);
    (((s ^ a) & (s ^ b)) >> 31) != 0
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100696b7(
    dest_surface: *mut i32,
    dest_x: i32,
    dest_y: i32,
    font_record: i32,
    char_code: i32,
    remap_table: i32,
) -> i32 {
    mech_trace::enter!(
        "100696b7",
        dest_surface,
        dest_x,
        dest_y,
        font_record,
        char_code,
        remap_table
    );
    // SAFETY: all pointers come from the caller's surface, font and remap data;
    // they are read and written exactly where the original does.
    unsafe {
        let mut dest_x = dest_x;
        let mut dest_y = dest_y;
        let bitmap_header =
            mech_trace::rd!("100696b7", dest_surface, read_unaligned) as usize as *mut i32;
        let mut i_var10 = mech_trace::rd!("100696b7", bitmap_header.add(1), read_unaligned);
        let row_pitch = i_var10.wrapping_add(1);
        if row_pitch == 0 || i_var10 < -1 {
            return mech_trace::rt!("100696b7", -1);
        }
        let i_var9 = mech_trace::rd!("100696b7", bitmap_header.add(2), read_unaligned);
        if i_var9 < 0 {
            return mech_trace::rt!("100696b7", -1);
        }
        let rect_left = mech_trace::rd!("100696b7", dest_surface.add(1), read_unaligned);
        let mut i_var6 = rect_left;
        if rect_left < 1 {
            i_var6 = 0;
        }
        let rect_top = mech_trace::rd!("100696b7", dest_surface.add(2), read_unaligned);
        let mut i_var7 = rect_top;
        if rect_top < 1 {
            i_var7 = 0;
        }
        let mut clamped_right = mech_trace::rd!("100696b7", dest_surface.add(3), read_unaligned);
        if i_var10 <= mech_trace::rd!("100696b7", dest_surface.add(3), read_unaligned) {
            clamped_right = i_var10;
        }
        i_var10 = mech_trace::rd!("100696b7", dest_surface.add(4), read_unaligned);
        if i_var9 <= mech_trace::rd!("100696b7", dest_surface.add(4), read_unaligned) {
            i_var10 = i_var9;
        }
        if clamped_right < i_var6 || i_var10 < i_var7 {
            return mech_trace::rt!("100696b7", -2);
        }
        dest_x = dest_x.wrapping_add(rect_left);
        dest_y = dest_y.wrapping_add(rect_top);
        let mut rows_left: u32 = mech_trace::rd!(
            "100696b7",
            ((font_record as u32).wrapping_add(8) as usize as *const u32),
            read_unaligned
        );
        let off_addr = (char_code as u32)
            .wrapping_mul(4)
            .wrapping_add(font_record as u32)
            .wrapping_add(0x10);
        let mut src: u32 = mech_trace::rd!(
            "100696b7",
            (off_addr as usize as *const i32),
            read_unaligned
        )
        .wrapping_add(font_record) as u32;
        let mut saved_visible_width: u32 = 0;
        let mut local_8: u32 =
            mech_trace::rd!("100696b7", (src as usize as *const u32), read_unaligned);
        'end: {
            if local_8 == 0 {
                break 'end;
            }
            src = src.wrapping_add(4);
            let i_var9 = clamped_right
                .wrapping_add(1)
                .wrapping_sub(local_8 as i32)
                .wrapping_sub(dest_x);
            let mut visible_width: u32 = local_8;
            if !(i_var9 > -1) {
                visible_width = local_8.wrapping_add(i_var9 as u32);
                let ok = visible_width != 0
                    && scarry4(local_8, i_var9 as u32) == ((visible_width as i32) < 0);
                if !ok {
                    break 'end;
                }
            }
            let i_var6 = dest_x.wrapping_sub(i_var6);
            if i_var6 < 0 {
                let add_overflow = scarry4(visible_width, i_var6 as u32);
                visible_width = visible_width.wrapping_add(i_var6 as u32);
                if visible_width == 0 || add_overflow != ((visible_width as i32) < 0) {
                    break 'end;
                }
                src = src.wrapping_sub(i_var6 as u32);
                dest_x = dest_x.wrapping_sub(i_var6);
            }
            let i_var10 = i_var10
                .wrapping_add(1)
                .wrapping_sub(rows_left as i32)
                .wrapping_sub(dest_y);
            let mut rows_ok = i_var10 > -1;
            if !rows_ok {
                let add_overflow = scarry4(rows_left, i_var10 as u32);
                rows_left = rows_left.wrapping_add(i_var10 as u32);
                rows_ok = rows_left != 0 && add_overflow == ((rows_left as i32) < 0);
            }
            if !rows_ok {
                break 'end;
            }
            let i_var7 = dest_y.wrapping_sub(i_var7);
            if i_var7 < 0 {
                let add_overflow = scarry4(rows_left, i_var7 as u32);
                rows_left = rows_left.wrapping_add(i_var7 as u32);
                if rows_left == 0 || add_overflow != ((rows_left as i32) < 0) {
                    break 'end;
                }
                dest_y = dest_y.wrapping_sub(i_var7);
                src = src.wrapping_sub((i_var7 as u32).wrapping_mul(local_8));
            }
            let mut dest: u32 = (dest_y as u32)
                .wrapping_mul(row_pitch as u32)
                .wrapping_add(mech_trace::rd!("100696b7", bitmap_header, read_unaligned) as u32)
                .wrapping_add(dest_x as u32);
            local_8 = local_8.wrapping_sub(visible_width);
            let mut u_var13: u32 = visible_width;
            if remap_table == 0 {
                loop {
                    u_var13 = visible_width >> 1;
                    while u_var13 != 0 {
                        let v = mech_trace::rd!(
                            "100696b7",
                            (src as usize as *const u16),
                            read_unaligned
                        );
                        mech_trace::wr!(
                            "100696b7",
                            (dest as usize as *mut u16),
                            write_unaligned,
                            v
                        );
                        src = src.wrapping_add(2);
                        dest = dest.wrapping_add(2);
                        u_var13 = u_var13.wrapping_sub(1);
                    }
                    u_var13 = ((visible_width & 1) != 0) as u32;
                    while u_var13 != 0 {
                        let v = mech_trace::rd!("100696b7", (src as usize as *const u8), read);
                        mech_trace::wr!("100696b7", (dest as usize as *mut u8), write, v);
                        src = src.wrapping_add(1);
                        dest = dest.wrapping_add(1);
                        u_var13 = u_var13.wrapping_sub(1);
                    }
                    src = src.wrapping_add(local_8);
                    dest = dest.wrapping_add((row_pitch as u32).wrapping_sub(visible_width));
                    rows_left = rows_left.wrapping_sub(1);
                    if rows_left == 0 {
                        break;
                    }
                }
                return mech_trace::rt!("100696b7", local_8.wrapping_add(visible_width) as i32);
            }
            loop {
                let mut pixels_left = visible_width;
                saved_visible_width = visible_width;
                if u_var13 == 0 {
                    break;
                }
                loop {
                    let s = mech_trace::rd!("100696b7", (src as usize as *const u8), read);
                    let mapped = mech_trace::rd!(
                        "100696b7",
                        ((remap_table as u32).wrapping_add(s as u32) as usize as *const u8),
                        read
                    );
                    if mapped != 0xff {
                        mech_trace::wr!("100696b7", (dest as usize as *mut u8), write, mapped);
                    }
                    src = src.wrapping_add(1);
                    dest = dest.wrapping_add(1);
                    pixels_left = pixels_left.wrapping_sub(1);
                    if pixels_left == 0 {
                        break;
                    }
                }
                src = src.wrapping_add(local_8);
                dest = dest.wrapping_add((row_pitch as u32).wrapping_sub(visible_width));
                rows_left = rows_left.wrapping_sub(1);
                u_var13 = rows_left;
            }
        }
        mech_trace::rt!("100696b7", local_8.wrapping_add(saved_visible_width) as i32)
    }
}
