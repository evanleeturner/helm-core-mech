// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut c_void;
    static mut DAT_100a365c: i32;
    static mut DAT_100a3660: u32;
    static mut DAT_100a3664: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100576d0() {
    // SAFETY: the globals and the entry table are the game's own memory;
    // each entry is 0x78 bytes and its handler table holds a callback at +0xc.
    unsafe {
        let mut entry_index: i32 = 0;
        if 0 < DAT_100a365c {
            let mut entry_offset: i32 = 0;
            loop {
                let entry = DAT_100a3664.byte_add(entry_offset as usize);
                let table = entry.cast::<i32>().read_unaligned();
                if table != 0 {
                    let func_ptr =
                        ((table as u32).wrapping_add(0xc) as *const u32).read_unaligned();
                    let func: extern "C" fn(i32) = core::mem::transmute(func_ptr as usize);
                    let arg = entry.cast::<i32>().add(0x1d).read_unaligned();
                    func(arg);
                }
                entry_offset = entry_offset.wrapping_add(0x78);
                entry_index = entry_index.wrapping_add(1);
                if !(entry_index < DAT_100a365c) {
                    break;
                }
            }
        }
        if !DAT_100a3664.is_null() {
            HeapFree(DAT_10098224, 1, DAT_100a3664);
        }
        DAT_100a3664 = core::ptr::null_mut();
        DAT_100a365c = 0;
        DAT_100a3660 = 0;
    }
}
