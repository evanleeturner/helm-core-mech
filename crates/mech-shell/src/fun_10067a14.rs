// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067a14(dest_surface_likely: *mut i32, fill_byte: u8) -> i32 {
    mech_trace::enter!("10067a14", dest_surface_likely, fill_byte);
    // SAFETY: the caller supplies a surface descriptor whose first word points
    // at a bitmap header; the pixel buffer is read and written where the
    // original does.
    unsafe {
        let bitmap_header =
            mech_trace::rd!("10067a14", dest_surface_likely, read_unaligned) as u32 as *const i32;
        let mut i_var7 = mech_trace::rd!("10067a14", bitmap_header.add(1), read_unaligned);
        let row_pitch = i_var7.wrapping_add(1);
        if row_pitch == 0 || i_var7 < -1 {
            return mech_trace::rt!("10067a14", -1);
        }
        let last_row = mech_trace::rd!("10067a14", bitmap_header.add(2), read_unaligned);
        if last_row < 0 {
            return mech_trace::rt!("10067a14", -1);
        }
        let mut clamped_left =
            mech_trace::rd!("10067a14", dest_surface_likely.add(1), read_unaligned);
        if clamped_left < 1 {
            clamped_left = 0;
        }
        let mut row = mech_trace::rd!("10067a14", dest_surface_likely.add(2), read_unaligned);
        if row < 1 {
            row = 0;
        }
        let mut clamped_right =
            mech_trace::rd!("10067a14", dest_surface_likely.add(3), read_unaligned);
        if i_var7 <= mech_trace::rd!("10067a14", dest_surface_likely.add(3), read_unaligned) {
            clamped_right = i_var7;
        }
        i_var7 = mech_trace::rd!("10067a14", dest_surface_likely.add(4), read_unaligned);
        if last_row <= mech_trace::rd!("10067a14", dest_surface_likely.add(4), read_unaligned) {
            i_var7 = last_row;
        }
        if clamped_left <= clamped_right && row <= i_var7 {
            let base = mech_trace::rd!("10067a14", bitmap_header, read_unaligned);
            let mut dest: u32 = (row.wrapping_mul(row_pitch))
                .wrapping_add(base)
                .wrapping_add(clamped_left) as u32;
            let row_width: u32 = (clamped_right.wrapping_add(1)).wrapping_sub(clamped_left) as u32;
            let fill_dword: u32 = u32::from(fill_byte).wrapping_mul(0x0101_0101);
            while row <= i_var7 {
                let mut steps_left = row_width & 3;
                while steps_left != 0 {
                    mech_trace::wr!("10067a14", (dest as usize as *mut u8), write, fill_byte);
                    dest = dest.wrapping_add(1);
                    steps_left = steps_left.wrapping_sub(1);
                }
                steps_left = row_width >> 2;
                while steps_left != 0 {
                    mech_trace::wr!(
                        "10067a14",
                        (dest as usize as *mut u32),
                        write_unaligned,
                        fill_dword
                    );
                    dest = dest.wrapping_add(4);
                    steps_left = steps_left.wrapping_sub(1);
                }
                dest = dest.wrapping_add((row_pitch as u32).wrapping_sub(row_width));
                row = row.wrapping_add(1);
            }
            return mech_trace::rt!("10067a14", 0);
        }
        mech_trace::rt!("10067a14", -2)
    }
}
