// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a1c: *mut u32;
    static mut DAT_100b9a2c: *mut u32;
    static mut DAT_100b9f74: *mut i32;
    static mut DAT_100b9f78: u32;
    static mut DAT_100b9f7c: *mut i32;
    static mut DAT_100b9f80: u32;
    static mut DAT_100c967c: u32;
    static mut DAT_100c9680: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10083e28() {
    // SAFETY: the globals are the game's own memory; the object pointers are
    // live objects whose first word is a vtable address, with the function
    // pointer at offset 8, called as the original does.
    unsafe {
        DAT_100b9a2c.write_unaligned(0);
        if !DAT_100b9f74.is_null() {
            let obj = DAT_100b9f74;
            let vt = obj.read_unaligned() as usize as *const u8;
            let f: unsafe extern "C" fn(*mut i32) =
                core::mem::transmute(vt.add(8).cast::<usize>().read_unaligned());
            f(obj);
            DAT_100b9f74 = core::ptr::null_mut();
            DAT_100b9f78 = 0;
            DAT_100b9f80 = 0;
            if DAT_100b9a1c.read_unaligned() == 0 {
                DAT_100b9f7c = core::ptr::null_mut();
            }
        }
        if !DAT_100b9f7c.is_null() && (DAT_100c967c != 0x140 || DAT_100c9680 != 200) {
            let obj = DAT_100b9f7c;
            let vt = obj.read_unaligned() as usize as *const u8;
            let f: unsafe extern "C" fn(*mut i32) =
                core::mem::transmute(vt.add(8).cast::<usize>().read_unaligned());
            f(obj);
            DAT_100b9f7c = core::ptr::null_mut();
        }
    }
}
