// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a1cc0: u8;
    static mut DAT_100a1e20: u8;
    static mut DAT_100a1f60: u8;
    static mut DAT_100a2060: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056be0(lo: i32, hi: i32) -> i64;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10057200(param_1: i32, out_entry_index: *mut i32) -> *mut *mut u8 {
    mech_trace::enter!("10057200", param_1, out_entry_index);
    let fallback_table: *mut *mut u8 = 0xa000b_usize as *mut *mut u8;
    // The original's four frame words; its first two are fallback_table's bits.
    let table_sizes: [i16; 4] = [0xb, 0xa, 0x8, 0x12];
    let mut table_index: i16;
    // SAFETY: the callee and the table symbols are the game's own memory;
    // out_entry_index points at the caller's i32.
    unsafe {
        if param_1 < 0xbea {
            table_index = (param_1.wrapping_sub(0xbe4) / 2) as i16;
            let roll = mech_trace::cr!("10057200", "10056be0", FUN_10056be0(0, 1000));
            if 0x2ed < (roll as i32) {
                let roll2 = mech_trace::cr!(
                    "10057200",
                    "10056be0",
                    FUN_10056be0(0, i32::from(table_index))
                );
                table_index = roll2 as i16;
            }
        } else {
            table_index = 3;
        }
        let mut selected_table: *mut *mut u8 = (&raw mut DAT_100a1cc0).cast::<*mut u8>();
        if table_index != 0 {
            selected_table = fallback_table;
        }
        if table_index == 1 {
            selected_table = (&raw mut DAT_100a1e20).cast::<*mut u8>();
        }
        if table_index == 2 {
            selected_table = (&raw mut DAT_100a1f60).cast::<*mut u8>();
        }
        if table_index == 3 {
            selected_table = (&raw mut DAT_100a2060).cast::<*mut u8>();
        }
        // Outside 0..=3 (a param_1 below 0xbe3) the original reads past its
        // four words into its own saved registers; the port takes 0 there.
        let table_size = usize::try_from(table_index)
            .ok()
            .and_then(|i| table_sizes.get(i).copied())
            .unwrap_or(0);
        let entry_roll = mech_trace::cr!(
            "10057200",
            "10056be0",
            FUN_10056be0(0, i32::from(table_size).wrapping_mul(10).wrapping_sub(1))
        ) as u64;
        let hi = ((entry_roll as i32) >> 0x1f) as u32 as u64;
        let combined = ((hi << 32) | (entry_roll & 0xffff_ffff)) as i64;
        let entry_index = (combined / 10) as i32;
        mech_trace::wr!("10057200", out_entry_index, write_unaligned, entry_index);
        mech_trace::rt!(
            "10057200",
            selected_table.wrapping_offset(entry_index.wrapping_mul(8) as isize)
        )
    }
}
