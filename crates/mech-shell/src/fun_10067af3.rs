// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067af3(
    src_view: *mut i32,
    src_x: i32,
    src_y: i32,
    dst_view: *mut i32,
    dst_x: i32,
    dst_y: i32,
    fill_value: u32,
) -> u32 {
    mech_trace::enter!(
        "10067af3", src_view, src_x, src_y, dst_view, dst_x, dst_y, fill_value
    );
    // SAFETY: all pointers are the caller's view/bitmap structures and pixel
    // buffers, read and written exactly where the original does.
    unsafe {
        let rd = |p: *const i32, i: usize| -> i32 {
            mech_trace::rd!("10067af3", p.add(i), read_unaligned)
        };

        let src_bitmap = rd(src_view, 0) as u32 as usize as *const i32;
        let mut src_clip_bottom = rd(src_bitmap, 1);
        let src_pitch = src_clip_bottom.wrapping_add(1);
        let mut t = 0;
        if src_pitch == 0 || src_clip_bottom < -1 || {
            t = rd(src_bitmap, 2);
            t < 0
        } {
            return mech_trace::rt!("10067af3", 0xffffffff);
        }
        let dst_clip_bottom_tmp = t;
        let src_view_left = rd(src_view, 1);
        let mut src_clip_left = src_view_left;
        if src_view_left < 1 {
            src_clip_left = 0;
        }
        let src_view_top = rd(src_view, 2);
        let mut src_clip_top = src_view_top;
        if src_view_top < 1 {
            src_clip_top = 0;
        }
        let mut src_clip_right = rd(src_view, 3);
        if src_clip_bottom <= rd(src_view, 3) {
            src_clip_right = src_clip_bottom;
        }
        src_clip_bottom = rd(src_view, 4);
        if dst_clip_bottom_tmp <= rd(src_view, 4) {
            src_clip_bottom = dst_clip_bottom_tmp;
        }
        if src_clip_right < src_clip_left || src_clip_bottom < src_clip_top {
            return mech_trace::rt!("10067af3", 0xfffffffe);
        }
        src_clip_left = src_clip_left.wrapping_sub(src_view_left);
        src_clip_right = src_clip_right.wrapping_sub(src_view_left);
        src_clip_top = src_clip_top.wrapping_sub(src_view_top);
        src_clip_bottom = src_clip_bottom.wrapping_sub(src_view_top);

        let dst_bitmap = rd(dst_view, 0) as u32 as usize as *const i32;
        let mut dst_clip_bottom = rd(dst_bitmap, 1);
        let dst_pitch = dst_clip_bottom.wrapping_add(1);
        let mut src_overlap_left;
        if dst_pitch == 0 || dst_clip_bottom < -1 || {
            src_overlap_left = rd(dst_bitmap, 2);
            src_overlap_left < 0
        } {
            return mech_trace::rt!("10067af3", 0xffffffff);
        }
        let dst_view_left = rd(dst_view, 1);
        let mut dst_overlap_left = dst_view_left;
        if dst_view_left < 1 {
            dst_overlap_left = 0;
        }
        let dst_view_top = rd(dst_view, 2);
        let mut dst_start_row = dst_view_top;
        if dst_view_top < 1 {
            dst_start_row = 0;
        }
        let mut dst_start_col = rd(dst_view, 3);
        if dst_clip_bottom <= rd(dst_view, 3) {
            dst_start_col = dst_clip_bottom;
        }
        dst_clip_bottom = rd(dst_view, 4);
        if src_overlap_left <= rd(dst_view, 4) {
            dst_clip_bottom = src_overlap_left;
        }
        if !(dst_overlap_left <= dst_start_col && dst_start_row <= dst_clip_bottom) {
            return mech_trace::rt!("10067af3", 0xfffffffe);
        }
        dst_overlap_left = dst_overlap_left.wrapping_sub(dst_view_left);
        dst_start_col = dst_start_col.wrapping_sub(dst_view_left);
        dst_start_row = dst_start_row.wrapping_sub(dst_view_top);
        dst_clip_bottom = dst_clip_bottom.wrapping_sub(dst_view_top);
        let smd_x = src_x.wrapping_sub(dst_x);
        let smd_y = src_y.wrapping_sub(dst_y);
        src_overlap_left = src_clip_left;
        if src_clip_left <= dst_overlap_left.wrapping_add(smd_x) {
            src_overlap_left = dst_overlap_left.wrapping_add(smd_x);
        }
        let mut src_start_row = src_clip_top;
        if src_clip_top <= dst_start_row.wrapping_add(smd_y) {
            src_start_row = dst_start_row.wrapping_add(smd_y);
        }
        let mut src_start_col = src_clip_right;
        if dst_start_col.wrapping_add(smd_x) <= src_clip_right {
            src_start_col = dst_start_col.wrapping_add(smd_x);
        }
        let mut src_overlap_bottom = src_clip_bottom;
        if dst_clip_bottom.wrapping_add(smd_y) <= src_clip_bottom {
            src_overlap_bottom = dst_clip_bottom.wrapping_add(smd_y);
        }
        if !(src_overlap_left <= src_start_col && src_start_row <= src_overlap_bottom) {
            return mech_trace::rt!("10067af3", 0xfffffffd);
        }
        if dst_overlap_left <= src_clip_left.wrapping_sub(smd_x) {
            dst_overlap_left = src_clip_left.wrapping_sub(smd_x);
        }
        if dst_start_row <= src_clip_top.wrapping_sub(smd_y) {
            dst_start_row = src_clip_top.wrapping_sub(smd_y);
        }
        if src_clip_right.wrapping_sub(smd_x) <= dst_start_col {
            dst_start_col = src_clip_right.wrapping_sub(smd_x);
        }
        if src_clip_bottom.wrapping_sub(smd_y) <= dst_clip_bottom {
            dst_clip_bottom = src_clip_bottom.wrapping_sub(smd_y);
        }
        let row_bytes: u32 = (src_start_col as u32)
            .wrapping_add(1)
            .wrapping_sub(src_overlap_left as u32);
        let mut rows: i32 = (src_overlap_bottom as u32)
            .wrapping_add(1)
            .wrapping_sub(src_start_row as u32) as i32;
        let mut dst_row_advance = dst_pitch;
        let mut src_row_advance = src_pitch;
        if src_start_row <= dst_start_row {
            dst_row_advance = 0i32.wrapping_sub(dst_pitch);
            src_row_advance = 0i32.wrapping_sub(src_pitch);
            src_start_row = src_overlap_bottom;
            dst_start_row = dst_clip_bottom;
        }
        let right_to_left = src_overlap_left <= dst_overlap_left;
        let back: u32;
        let bytes_left: u32;
        if right_to_left {
            back = 3;
            bytes_left = row_bytes;
        } else {
            back = 0;
            src_start_col = src_overlap_left;
            dst_start_col = dst_overlap_left;
            bytes_left = 0u32.wrapping_sub(row_bytes);
        }
        src_row_advance = src_row_advance.wrapping_add(bytes_left as i32);
        dst_row_advance = dst_row_advance.wrapping_add(bytes_left as i32);
        let step: u32 = if right_to_left { 0xffff_ffff } else { 1 };
        let step4: u32 = step.wrapping_mul(4);

        let mut dst_cursor: u32 = (dst_view_top as u32)
            .wrapping_mul(dst_pitch as u32)
            .wrapping_add(mech_trace::rd!("10067af3", dst_bitmap, read_unaligned) as u32)
            .wrapping_add(dst_view_left as u32)
            .wrapping_add((dst_start_row as u32).wrapping_mul(dst_pitch as u32))
            .wrapping_add(dst_start_col as u32);
        let mut src_cursor: u32 = (src_view_top as u32)
            .wrapping_mul(src_pitch as u32)
            .wrapping_add(mech_trace::rd!("10067af3", src_bitmap, read_unaligned) as u32)
            .wrapping_add(src_view_left as u32)
            .wrapping_add((src_start_row as u32).wrapping_mul(src_pitch as u32))
            .wrapping_add(src_start_col as u32);

        if fill_value & 0xffff_ff00 == 0 {
            let fill_byte = fill_value as u8;
            let pattern = (fill_byte as u32).wrapping_mul(0x01010101);
            loop {
                let mut n = row_bytes & 3;
                while n != 0 {
                    mech_trace::wr!(
                        "10067af3",
                        (dst_cursor as usize as *mut u8),
                        write,
                        fill_byte
                    );
                    dst_cursor = dst_cursor.wrapping_add(step);
                    n -= 1;
                }
                let mut dw = dst_cursor.wrapping_sub(back);
                let mut n = row_bytes >> 2;
                while n != 0 {
                    mech_trace::wr!(
                        "10067af3",
                        (dw as usize as *mut u32),
                        write_unaligned,
                        pattern
                    );
                    dw = dw.wrapping_add(step4);
                    n -= 1;
                }
                dst_cursor = dw.wrapping_add(dst_row_advance as u32).wrapping_add(back);
                rows = rows.wrapping_sub(1);
                if rows == 0 {
                    break;
                }
            }
        } else {
            loop {
                let mut n = row_bytes & 3;
                while n != 0 {
                    let b = mech_trace::rd!("10067af3", (src_cursor as usize as *const u8), read);
                    mech_trace::wr!("10067af3", (dst_cursor as usize as *mut u8), write, b);
                    src_cursor = src_cursor.wrapping_add(step);
                    dst_cursor = dst_cursor.wrapping_add(step);
                    n -= 1;
                }
                let mut sw = src_cursor.wrapping_sub(back);
                let mut dw = dst_cursor.wrapping_sub(back);
                let mut n = row_bytes >> 2;
                while n != 0 {
                    let v =
                        mech_trace::rd!("10067af3", (sw as usize as *const u32), read_unaligned);
                    mech_trace::wr!("10067af3", (dw as usize as *mut u32), write_unaligned, v);
                    sw = sw.wrapping_add(step4);
                    dw = dw.wrapping_add(step4);
                    n -= 1;
                }
                src_cursor = sw.wrapping_add(src_row_advance as u32).wrapping_add(back);
                dst_cursor = dw.wrapping_add(dst_row_advance as u32).wrapping_add(back);
                rows = rows.wrapping_sub(1);
                if rows == 0 {
                    break;
                }
            }
        }
        mech_trace::rt!("10067af3", 0)
    }
}
