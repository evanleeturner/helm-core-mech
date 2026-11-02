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
    mech_trace::enter!("100576d0");
    // SAFETY: the globals and the entry table are the game's own memory;
    // each entry is 0x78 bytes and its handler table holds a callback at +0xc.
    unsafe {
        let mut entry_index: i32 = 0;
        if 0 < mech_trace::gr!("100576d0", "100a365c", DAT_100a365c) {
            let mut entry_offset: i32 = 0;
            loop {
                let entry = mech_trace::gr!("100576d0", "100a3664", DAT_100a3664)
                    .byte_add(entry_offset as usize);
                let table = mech_trace::rd!("100576d0", entry.cast::<i32>(), read_unaligned);
                if table != 0 {
                    let func_ptr = mech_trace::rd!(
                        "100576d0",
                        ((table as u32).wrapping_add(0xc) as *const u32),
                        read_unaligned
                    );
                    let func: extern "C" fn(i32) = core::mem::transmute(func_ptr as usize);
                    let arg =
                        mech_trace::rd!("100576d0", entry.cast::<i32>().add(0x1d), read_unaligned);
                    mech_trace::cr!("100576d0", "ind", func(arg));
                }
                entry_offset = entry_offset.wrapping_add(0x78);
                entry_index = entry_index.wrapping_add(1);
                if !(entry_index < mech_trace::gr!("100576d0", "100a365c", DAT_100a365c)) {
                    break;
                }
            }
        }
        if !mech_trace::gr!("100576d0", "100a3664", DAT_100a3664).is_null() {
            mech_trace::cr!(
                "100576d0",
                "HeapFree",
                HeapFree(
                    mech_trace::gr!("100576d0", "10098224", DAT_10098224),
                    1,
                    mech_trace::gr!("100576d0", "100a3664", DAT_100a3664)
                )
            );
        }
        mech_trace::gw!("100576d0", "100a3664", DAT_100a3664, core::ptr::null_mut());
        mech_trace::gw!("100576d0", "100a365c", DAT_100a365c, 0);
        mech_trace::gw!("100576d0", "100a3660", DAT_100a3660, 0);
    }
}
