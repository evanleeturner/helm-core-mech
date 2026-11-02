// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008d02c: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(record: i32, collection: i32) -> u32;
    fn strncpy(dest: *mut i8, source: *const i8, count: usize) -> *mut i8;
    fn atoi(s: *const i8) -> i32;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000be60(record: i32, collection: i32) {
    let mut prefix: [i8; 4] = [0; 4];
    let mut tail: [i8; 16] = [0; 16];
    let mut number_text: [i8; 4] = [0; 4];
    let mut candidate_number: i32 = 0;
    let unused_number: i32;

    // SAFETY: record and collection point at the game's own structures; the
    // callees and globals are the game's; buffers are local and sized as in the original.
    unsafe {
        loop {
            let mut number_taken = false;
            let mut item_index: i32 = 0;
            if 0 < ((collection.wrapping_add(4)) as *const i32).read_unaligned() {
                loop {
                    let item = FUN_10018060(collection, item_index) as i32;
                    strncpy(
                        number_text.as_mut_ptr(),
                        item.wrapping_add(0x826) as *const i8,
                        2,
                    );
                    number_text[2] = 0;
                    let parsed = atoi(number_text.as_ptr());
                    if parsed == candidate_number {
                        candidate_number = candidate_number.wrapping_add(1);
                        number_taken = true;
                        break;
                    }
                    item_index = item_index.wrapping_add(1);
                    if !(item_index < ((collection.wrapping_add(4)) as *const i32).read_unaligned())
                    {
                        break;
                    }
                }
            }
            if !number_taken {
                unused_number = candidate_number;
                break;
            }
        }
        strncpy(
            prefix.as_mut_ptr(),
            record.wrapping_add(0x823) as *const i8,
            3,
        );
        prefix[3] = 0;
        let mut src = record.wrapping_add(0x828) as *const i8;
        let mut dst = tail.as_mut_ptr();
        loop {
            let c = *src;
            *dst = c;
            src = src.add(1);
            dst = dst.add(1);
            if c == 0 {
                break;
            }
        }
        sprintf(
            record.wrapping_add(0x823) as *mut i8,
            &raw mut DAT_1008d02c as *const i8,
            prefix.as_ptr(),
            unused_number,
            tail.as_ptr(),
        );
    }
}
