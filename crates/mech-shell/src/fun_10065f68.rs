// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065f68(
    dest_surface: *mut i32,
    rle_image: i32,
    dest_x: i32,
    dest_y: i32,
) -> u32 {
    // SAFETY: all pointers are the game's own surface and RLE image data,
    // read and written exactly where the original does.
    unsafe {
        let surf = dest_surface.read_unaligned() as usize as *const i32;
        let i_var1 = surf.add(1).read_unaligned();
        let row_pitch = (i_var1 as u32).wrapping_add(1) as i32;
        if row_pitch != 0 && -2 < i_var1 {
            let img = rle_image as usize as *const u8;
            let top = img.byte_add(0xc).cast::<i32>().read_unaligned();
            let base = surf.read_unaligned() as u32;
            let off8 = img.byte_add(8).cast::<i32>().read_unaligned() as u32;
            let d1 = dest_surface.add(1).read_unaligned() as u32;
            let d2 = dest_surface.add(2).read_unaligned() as u32;
            let rows_part = (top as u32)
                .wrapping_add(dest_y as u32)
                .wrapping_add(d2)
                .wrapping_mul(row_pitch as u32);
            let mut dest_row = base
                .wrapping_add(off8)
                .wrapping_add(dest_x as u32)
                .wrapping_add(d1)
                .wrapping_add(rows_part) as usize as *mut u8;
            let end_row =
                (img.byte_add(0x14).cast::<i32>().read_unaligned() as u32).wrapping_add(1) as i32;
            let mut rows_left = (end_row as u32).wrapping_sub(top as u32) as i32;
            if rows_left != 0 && top <= end_row {
                let mut src = img.byte_add(0x18);
                loop {
                    let b = *src;
                    src = src.add(1);
                    let mut low = (b & 1) != 0;
                    let mut run = (b >> 1) as u32;
                    let mut dp = dest_row;
                    // 0 = fill, 1 = copy, 2 = skip/end check
                    let mut state: u8 = if run != 0 { if low { 1 } else { 0 } } else { 2 };
                    loop {
                        match state {
                            0 => {
                                let v = *src;
                                let next = src.add(1);
                                let mut n = run & 3;
                                while n != 0 {
                                    *dp = v;
                                    dp = dp.add(1);
                                    n -= 1;
                                }
                                let mut w = (run & 0xff) >> 2;
                                let word = (v as u32).wrapping_mul(0x0101_0101);
                                while w != 0 {
                                    dp.cast::<u32>().write_unaligned(word);
                                    dp = dp.add(4);
                                    w -= 1;
                                }
                                src = src.add(2);
                                let nb = *next;
                                low = (nb & 1) != 0;
                                run = (nb >> 1) as u32;
                            }
                            1 => {
                                let mut n = run & 3;
                                while n != 0 {
                                    *dp = *src;
                                    src = src.add(1);
                                    dp = dp.add(1);
                                    n -= 1;
                                }
                                let mut w = (run & 0xff) >> 2;
                                while w != 0 {
                                    let v = src.cast::<u32>().read_unaligned();
                                    dp.cast::<u32>().write_unaligned(v);
                                    src = src.add(4);
                                    dp = dp.add(4);
                                    w -= 1;
                                }
                                let nb = *src;
                                src = src.add(1);
                                low = (nb & 1) != 0;
                                run = (nb >> 1) as u32;
                            }
                            _ => {
                                if !low {
                                    break;
                                }
                                dp = dp.wrapping_add(*src as usize);
                                let nb = *src.add(1);
                                src = src.add(2);
                                low = (nb & 1) != 0;
                                run = (nb >> 1) as u32;
                            }
                        }
                        state = if run != 0 { if low { 1 } else { 0 } } else { 2 };
                    }
                    dest_row = dest_row.wrapping_offset(row_pitch as isize);
                    rows_left = rows_left.wrapping_sub(1);
                    if rows_left == 0 {
                        break;
                    }
                }
            }
        }
        0
    }
}
