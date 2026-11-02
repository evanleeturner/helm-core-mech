// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e3f0: [u32; 8];
    static mut DAT_100a6ba4: u8;
    static mut DAT_100b0588: u32;
    static mut DAT_100d08ec: *mut core::ffi::c_void;
    static mut DAT_1008e410: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000e8f0(param_1: u32, param_2: u32, param_3: u32) -> u32;
    fn FUN_100166c0(param_1: *mut core::ffi::c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000f8d0() -> u32 {
    // SAFETY: the globals and callees are the game's own memory and code;
    // the scan stays within the 8-entry table at DAT_1008e3f0.
    unsafe {
        let base = (&raw mut DAT_1008e3f0).cast::<u32>();
        let end = (&raw mut DAT_1008e410).cast::<u32>();
        let mut entry_ptr = base;
        let mut entry_index: i32 = 0;
        loop {
            if entry_ptr.read() != 0 {
                break;
            }
            entry_ptr = entry_ptr.add(1);
            entry_index = entry_index.wrapping_add(1);
            if !(entry_ptr < end) {
                break;
            }
        }
        if entry_index == 8 {
            return 1;
        }
        let slot = base.offset(entry_index as isize);
        FUN_1000e8f0(slot.read(), DAT_100b0588, DAT_100b0588);
        let menu_handle = DAT_100d08ec;
        slot.write(0);
        FUN_100166c0(menu_handle);
        DAT_100a6ba4 = 1;
        0
    }
}
