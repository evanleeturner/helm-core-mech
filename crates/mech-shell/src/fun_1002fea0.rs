// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000cfe0(first_record_likely: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fea0(first_record_likely: i32, second_record_likely: i32) -> i32 {
    mech_trace::enter!("1002fea0", first_record_likely, second_record_likely);
    let mut mismatch_count: i32 = 0;
    let mut first_row_base: i32 = first_record_likely.wrapping_add(0x760);
    let mut second_row_base: i32 = second_record_likely.wrapping_add(0x760);
    let mut abs_difference_total: i32 = 0;
    let mut rows_left: i32 = 8;
    let mut first_cell_cursor: *const i32 = first_record_likely.wrapping_add(0x368) as *const i32;
    let mut second_cell_cursor: *const i32 = second_record_likely.wrapping_add(0x368) as *const i32;
    // SAFETY: the records are the game's own memory; reads happen at the same
    // offsets as the original, and the callee is the game's function.
    unsafe {
        loop {
            let s8 = mech_trace::rd!(
                "1002fea0",
                ((second_row_base.wrapping_add(8)) as *const i32),
                read_unaligned
            );
            let f8 = mech_trace::rd!(
                "1002fea0",
                ((first_row_base.wrapping_add(8)) as *const i32),
                read_unaligned
            );
            let fc = mech_trace::rd!(
                "1002fea0",
                ((first_row_base.wrapping_add(0xc)) as *const i32),
                read_unaligned
            );
            let sc = mech_trace::rd!(
                "1002fea0",
                ((second_row_base.wrapping_add(0xc)) as *const i32),
                read_unaligned
            );
            let sum_difference: u32 =
                (s8.wrapping_sub(f8).wrapping_sub(fc).wrapping_add(sc)) as u32;
            let sign_mask: u32 = ((sum_difference as i32) >> 0x1f) as u32;
            abs_difference_total = (abs_difference_total as u32)
                .wrapping_sub(sign_mask)
                .wrapping_add(sum_difference ^ sign_mask) as i32;
            let mut columns_left: i32 = 0xc;
            loop {
                let a = mech_trace::rd!("1002fea0", first_cell_cursor, read_unaligned);
                let b = mech_trace::rd!("1002fea0", second_cell_cursor, read_unaligned);
                if a != b && (a != 0 || b != -1) {
                    mismatch_count = mismatch_count.wrapping_add(1);
                }
                second_cell_cursor = second_cell_cursor.wrapping_add(1);
                first_cell_cursor = first_cell_cursor.wrapping_add(1);
                columns_left = columns_left.wrapping_sub(1);
                if columns_left == 0 {
                    break;
                }
            }
            first_row_base = first_row_base.wrapping_add(0x12);
            second_row_base = second_row_base.wrapping_add(0x12);
            rows_left = rows_left.wrapping_sub(1);
            if rows_left == 0 {
                break;
            }
        }
        let r = mech_trace::cr!("1002fea0", "1000cfe0", FUN_1000cfe0(abs_difference_total));
        mech_trace::rt!(
            "1002fea0",
            r.wrapping_add(mismatch_count.wrapping_mul(50000))
        )
    }
}
