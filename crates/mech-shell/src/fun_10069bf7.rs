// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a8fb1: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069881(draw_target: *mut i32, image_data: i32, param_3: *mut i32, param_4: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10069bf7(draw_target: *mut i32, image_data: i32) -> i32 {
    // SAFETY: image_data points at the game's image block, and the row buffer
    // DAT_100a8fb1 is the game's own scratch memory, read and written where
    // the original does.
    unsafe {
        let y_max = ((image_data.wrapping_add(10)) as *const i16).read_unaligned();
        let y_min = ((image_data.wrapping_add(6)) as *const i16).read_unaligned();
        let mut row_index: i32 = 0;
        let row_bytes = ((image_data.wrapping_add(0x42)) as *const u16).read_unaligned();
        let mut coded_cursor = (image_data.wrapping_add(0x80)) as *const u8;
        let buf = &raw mut DAT_100a8fb1;
        loop {
            let mut row_cursor: *mut i8 = buf;
            loop {
                let b = coded_cursor.read();
                let src_next = coded_cursor.add(1);
                let flagged: u16 = ((((b as u16) << 8) | (b as u16)) & 0xc0ff) as u16;
                let run_token: u16 = flagged ^ 0xc000;
                let next_row_cursor: *mut i8;
                if (run_token >> 8) as u8 == 0 {
                    coded_cursor = coded_cursor.add(2);
                    let data = src_next.read() as i8;
                    next_row_cursor = row_cursor;
                    let mut p = next_row_cursor;
                    let mut repeats = (run_token & 0x3f) as u32;
                    while repeats != 0 {
                        p.write(data);
                        p = p.add(1);
                        repeats -= 1;
                    }
                    row_cursor = p;
                } else {
                    row_cursor.write(flagged as u8 as i8);
                    row_cursor = row_cursor.add(1);
                    coded_cursor = src_next;
                }
                let limit = (row_bytes as i32).wrapping_add(0x100a8fb1);
                if !((row_cursor as usize as i32) < limit) {
                    break;
                }
            }
            FUN_10069881(
                draw_target,
                row_index,
                buf as *mut i32,
                row_bytes as usize as *mut i8,
            );
            row_index = row_index.wrapping_add(1);
            let bound = y_max.wrapping_sub(y_min) as u16 as i32;
            if !(row_index <= bound) {
                break;
            }
        }
        0
    }
}
