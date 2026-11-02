// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a724c: *mut u32;
    static mut DAT_100a725c: *mut u32;
    static mut DAT_100a817c: *mut i32;
    static mut DAT_100a8180: u32;
    static mut DAT_100a8184: *mut i32;
    static mut DAT_100a8188: u32;
    static mut DAT_100c605c: u32;
    static mut DAT_100c6060: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100624a0() {
    // SAFETY: the globals are the game's own memory; the pointers they hold
    // are valid objects with vtables, used exactly as the original does.
    unsafe {
        DAT_100a725c.write_unaligned(0);
        if !DAT_100a817c.is_null() {
            let obj = DAT_100a817c;
            let vtable = obj.read_unaligned();
            let f = ((vtable as u32).wrapping_add(8) as *const usize).read_unaligned();
            let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f);
            f(obj);
            DAT_100a817c = core::ptr::null_mut();
            DAT_100a8180 = 0;
            DAT_100a8188 = 0;
            if DAT_100a724c.read_unaligned() == 0 {
                DAT_100a8184 = core::ptr::null_mut();
            }
        }
        if !DAT_100a8184.is_null() && (DAT_100c605c != 0x140 || DAT_100c6060 != 200) {
            let obj = DAT_100a8184;
            let vtable = obj.read_unaligned();
            let f = ((vtable as u32).wrapping_add(8) as *const usize).read_unaligned();
            let f: unsafe extern "C" fn(*mut i32) = core::mem::transmute(f);
            f(obj);
            DAT_100a8184 = core::ptr::null_mut();
        }
    }
}
