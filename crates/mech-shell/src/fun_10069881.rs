// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069881(
    param_1: *mut i32,
    param_2: i32,
    param_3: *mut u32,
    param_4: *mut u8,
) -> *mut u32 {
    // SAFETY: the view, its bitmap and the source/destination buffers are the
    // game's own memory; reads and writes happen where the original does.
    unsafe {
        let dst_bitmap = param_1.read_unaligned() as usize as *mut i32;
        let max_x = dst_bitmap.add(1).read_unaligned();
        if max_x.wrapping_add(1) == 0 || max_x < -1 {
            return 0xffff_ffffusize as *mut u32;
        }
        let max_y = dst_bitmap.add(2).read_unaligned();
        if max_y < 0 {
            return 0xffff_ffffusize as *mut u32;
        }
        let mut start_column = param_1.add(1).read_unaligned();
        let mut clip_left = start_column;
        if start_column < 1 {
            clip_left = 0;
        }
        let bitmap_row = param_1.add(2).read_unaligned();
        let mut clip_top = bitmap_row;
        if bitmap_row < 1 {
            clip_top = 0;
        }
        let mut clip_right = param_1.add(3).read_unaligned();
        if max_x <= param_1.add(3).read_unaligned() {
            clip_right = max_x;
        }
        let mut clip_bottom = param_1.add(4).read_unaligned();
        if max_y <= param_1.add(4).read_unaligned() {
            clip_bottom = max_y;
        }
        if clip_left <= clip_right && clip_top <= clip_bottom {
            let row = param_2.wrapping_add(bitmap_row);
            let mut byte_count = param_4 as usize as i32;
            let mut src = param_3;
            let mut ret = clip_right
                .wrapping_sub(start_column)
                .wrapping_add(1)
                .wrapping_sub(byte_count);
            let mut proceed = true;
            if !(-1 < ret) {
                let (nb, ov) = byte_count.overflowing_add(ret);
                byte_count = nb;
                proceed = byte_count != 0 && ov == (byte_count < 0);
            }
            if proceed {
                ret = start_column.wrapping_sub(clip_left);
                if ret < 0 {
                    let (nb, ov) = byte_count.overflowing_add(ret);
                    byte_count = nb;
                    if byte_count == 0 || ov != (byte_count < 0) {
                        return ret as usize as *mut u32;
                    }
                    src = (src as usize as i32).wrapping_sub(ret) as usize as *mut u32;
                    start_column = start_column.wrapping_sub(ret);
                }
                ret = clip_bottom.wrapping_sub(row);
                if -1 < ret {
                    ret = row.wrapping_sub(clip_top);
                    if -1 < ret {
                        ret = row
                            .wrapping_mul(max_x.wrapping_add(1))
                            .wrapping_add(dst_bitmap.read_unaligned())
                            .wrapping_add(start_column);
                        let mut dst = ret as usize as *mut u32;
                        let mut left = (byte_count as u32) & 3;
                        while left != 0 {
                            (dst as *mut u8).write((src as *const u8).read());
                            src = (src as *mut u8).add(1) as *mut u32;
                            dst = (dst as *mut u8).add(1) as *mut u32;
                            left -= 1;
                        }
                        left = (byte_count as u32) >> 2;
                        while left != 0 {
                            dst.write_unaligned(src.read_unaligned());
                            src = src.add(1);
                            dst = dst.add(1);
                            left -= 1;
                        }
                    }
                }
            }
            return ret as usize as *mut u32;
        }
        0xffff_fffeusize as *mut u32
    }
}
