// SPDX-License-Identifier: MIT
use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e760: *mut c_void;
    static mut DAT_1008e764: *mut c_void;
    static mut DAT_1008e460: i32;
    static mut DAT_100c42c0: i32;
    static mut DAT_100c42c4: [i8; 0];
    static mut DAT_1008e464: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __wfopen(filename: *const c_void, mode: *const c_void) -> *mut c_void;
    fn _qsort(
        base: *mut c_void,
        num: u32,
        width: u32,
        comp: unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
    );
    fn FUN_1006f790(ptr: *mut c_void, size: u32, count: u32, stream: *mut c_void) -> u32;
    fn _fclose(stream: *mut c_void) -> i32;
    fn FUN_1000fcb0(a: *const c_void, b: *const c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000fce0() {
    mech_trace::enter!("1000fce0");
    // SAFETY: all globals and callees are the game's own; the table copy
    // reads and writes the same 0x45-byte entries the original does.
    unsafe {
        let high_cfg_file = mech_trace::cr!(
            "1000fce0",
            "__wfopen",
            __wfopen(
                mech_trace::gr!("1000fce0", "1008e764", DAT_1008e764) as *const c_void,
                mech_trace::gr!("1000fce0", "1008e760", DAT_1008e760) as *const c_void
            )
        );
        let mut entries_copied: i32 = 0;
        if high_cfg_file.is_null() {
            mech_trace::gw!(
                "1000fce0",
                "100c42c0",
                DAT_100c42c0,
                mech_trace::gr!("1000fce0", "1008e460", DAT_1008e460)
            );
            if mech_trace::gr!("1000fce0", "1008e460", DAT_1008e460) > 0 {
                let mut entry_offset: i32 = 0;
                loop {
                    entries_copied = entries_copied.wrapping_add(1);
                    let mut src = (&raw mut DAT_1008e464)
                        .cast::<i8>()
                        .byte_add(entry_offset as usize);
                    let mut dst = (&raw mut DAT_100c42c4)
                        .cast::<i8>()
                        .byte_add(entry_offset as usize);
                    let mut dwords_left: i32 = 0x11;
                    while dwords_left != 0 {
                        mech_trace::wr!(
                            "1000fce0",
                            dst.cast::<u32>(),
                            write_unaligned,
                            mech_trace::rd!("1000fce0", src.cast::<u32>(), read_unaligned)
                        );
                        src = src.byte_add(4);
                        dst = dst.byte_add(4);
                        dwords_left -= 1;
                    }
                    mech_trace::dw!("1000fce0", dst, mech_trace::dr!("1000fce0", src));
                    entry_offset = entry_offset.wrapping_add(0x45);
                    if !(entries_copied < mech_trace::gr!("1000fce0", "100c42c0", DAT_100c42c0)) {
                        break;
                    }
                }
            }
            _qsort(
                (&raw mut DAT_100c42c4).cast::<c_void>(),
                mech_trace::gr!("1000fce0", "100c42c0", DAT_100c42c0) as u32,
                0x45,
                FUN_1000fcb0,
            );
            return;
        }
        mech_trace::cr!(
            "1000fce0",
            "1006f790",
            FUN_1006f790(
                (&raw mut DAT_100c42c0).cast::<c_void>(),
                0x2b6,
                1,
                high_cfg_file,
            )
        );
        mech_trace::cr!("1000fce0", "_fclose", _fclose(high_cfg_file));
    }
}
