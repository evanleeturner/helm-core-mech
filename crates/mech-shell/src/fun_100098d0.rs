// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10085194: u8;
    static mut DAT_10086214: u8;
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100a67b4: *mut i32;
    static mut DAT_100a67b8: *mut i32;
    static mut DAT_100a67bc: *mut i32;
    static mut DAT_100a67c0: *mut i32;
    static mut DAT_100a67c4: *mut i32;
    static mut DAT_100a67c8: *mut i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100180f0(match_value: *mut i32, param_2: i8) -> u8;
    fn FUN_10017df0(match_value: *mut i32, param_2: u32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000c990();
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100098d0(match_value: i32) {
    // SAFETY: globals, callees and heap records are the game's own; records
    // are freshly allocated with enough bytes for every write, and the row
    // table is walked exactly as the original does.
    unsafe {
        FUN_100180f0(DAT_100a67b4, 1);
        FUN_100180f0(DAT_100a67b8, 1);
        FUN_100180f0(DAT_100a67c8, 1);
        FUN_100180f0(DAT_100a67c4, 1);
        FUN_100180f0(DAT_100a67bc, 1);
        let mut entry_index: i32 = 0;
        let mut row = (&raw mut DAT_10085194) as *mut i32;
        FUN_100180f0(DAT_100a67c0, 1);
        loop {
            if row.read_unaligned() == match_value {
                let rec = HeapAlloc(DAT_10098224, 1, 0x1a) as *mut u8;
                rec.cast::<i32>().write_unaligned(entry_index);
                rec.byte_add(4).cast::<i32>().write_unaligned(0);
                rec.byte_add(8).write(0);
                rec.byte_add(10).cast::<u32>().write_unaligned(0xffff_ffff);
                rec.byte_add(0xe).cast::<u32>().write_unaligned(0xbdf);
                rec.byte_add(0x12).cast::<u32>().write_unaligned(100);
                FUN_10017df0(DAT_100a67b4, rec as u32);
            }
            row = row.wrapping_add(0x10);
            entry_index = entry_index.wrapping_add(1);
            if !(row < ((&raw mut DAT_10086214) as *mut i32)) {
                break;
            }
        }
        entry_index = 0;
        row = (&raw mut DAT_10085194) as *mut i32;
        loop {
            if row.read_unaligned() == match_value && 0 < row.wrapping_sub(1).read_unaligned() {
                let rec = HeapAlloc(DAT_10098224, 1, 0x1a) as *mut u8;
                rec.cast::<i32>().write_unaligned(entry_index);
                rec.byte_add(4).cast::<i32>().write_unaligned(0);
                rec.byte_add(8).write(1);
                rec.byte_add(10).cast::<i32>().write_unaligned(entry_index);
                rec.byte_add(0xe).cast::<u32>().write_unaligned(1);
                rec.byte_add(0x12).cast::<u32>().write_unaligned(0xbdf);
                rec.byte_add(0x16).cast::<u32>().write_unaligned(100);
                FUN_10017df0(DAT_100a67b8, rec as u32);
            }
            row = row.wrapping_add(0x10);
            entry_index = entry_index.wrapping_add(1);
            if !(row < ((&raw mut DAT_10086214) as *mut i32)) {
                break;
            }
        }
        entry_index = 0;
        loop {
            let rec = HeapAlloc(DAT_10098224, 1, 0x16) as *mut u8;
            rec.cast::<i32>().write_unaligned(entry_index);
            entry_index = entry_index.wrapping_add(1);
            rec.byte_add(4).cast::<i32>().write_unaligned(0);
            rec.byte_add(8).write(2);
            rec.byte_add(10).cast::<u32>().write_unaligned(0);
            rec.byte_add(0xe).cast::<u32>().write_unaligned(0);
            rec.byte_add(0x12).cast::<u32>().write_unaligned(0);
            FUN_10017df0(DAT_100a67b4, rec as u32);
            if !(entry_index < 0xb) {
                break;
            }
        }
        FUN_1000c990();
    }
}
