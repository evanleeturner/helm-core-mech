// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: u32;
    static mut DAT_100a6720: *mut u32;
    static mut DAT_100a89e0: [u8; 0];
    static mut DAT_100a8a30: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005af20() -> i32 {
    // SAFETY: the globals and the table they describe are the game's own
    // memory; every access is at the offsets the original uses.
    unsafe {
        let mut result_id: i32 = 0xc004;
        let base = DAT_10083508;
        let state = ((base.wrapping_add(0x23c7)) as *const i32).read_unaligned();
        if state == 0xc02d || state == 0xc00a || state == 0xc02b {
            return 0xc004;
        }
        let mut entry_index: i32 = 0;
        let mut table_entry = (&raw mut DAT_100a89e0) as *const u8 as *const i32;
        let end = (&raw mut DAT_100a8a30) as *const u8 as *const i32;
        loop {
            if (table_entry.read_unaligned() as u32) == DAT_100a6720.read_unaligned() {
                result_id = ((entry_index.wrapping_mul(8).wrapping_add(0x100a89e4)) as *const i32)
                    .read_unaligned();
                if result_id == state {
                    return 0xc004;
                }
                DAT_100a6720.write_unaligned(0);
                break;
            }
            table_entry = table_entry.wrapping_add(2);
            entry_index = entry_index.wrapping_add(1);
            if !(table_entry < end) {
                break;
            }
        }
        if result_id == 0xc004 {
            return result_id;
        }
        let mode = ((base.wrapping_add(0x23bb)) as *const i32).read_unaligned();
        let b9 = ((base.wrapping_add(0x23b9)) as *const u8).read() as i8;
        let b1 = ((base.wrapping_add(1)) as *const u8).read() as i8;
        let flag = base.wrapping_add(0x23ba) as *mut u8;
        if mode == 0 {
            if b9 == 1 {
                return 0xc004;
            } else if result_id == 0xc026 {
                if b1 == 1 {
                    flag.write(1);
                    return result_id;
                }
                return 0xc004;
            }
        } else if mode == 1 {
            if b9 == 1 {
                match result_id {
                    0xc01f | 0xc020 | 0xc028 => {
                        flag.write(1);
                        return result_id;
                    }
                    _ => {}
                }
            } else {
                match result_id {
                    0xc01d | 0xc01e | 0xc01f | 0xc020 | 0xc022 | 0xc024 | 0xc026 | 0xc028
                    | 0xc02f | 0xc030 => {
                        if b1 == 1 {
                            flag.write(1);
                            return result_id;
                        }
                        return 0xc004;
                    }
                    _ => {}
                }
            }
        }
        0xc004
    }
}
