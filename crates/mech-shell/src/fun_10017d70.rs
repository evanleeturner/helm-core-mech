// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098224: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(mem: i32, param_2: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, p: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10017d70(mem: *mut i32) -> u32 {
    if mem.is_null() {
        return 0;
    }
    // SAFETY: mem points at the game's collection; its fields are read and
    // written at the offsets the original uses, and the callees are the game's own.
    unsafe {
        let mut item_index: i32 = 0;
        if 0 < mem.read_unaligned() {
            loop {
                let item = FUN_10018060(mem as i32, item_index) as i32;
                if item != 0 {
                    let cb = mem.byte_add(0x15).cast::<usize>().read_unaligned();
                    if cb != 0 {
                        let f: unsafe extern "C" fn(i32) = core::mem::transmute(cb);
                        f(item);
                    }
                }
                item_index = item_index.wrapping_add(1);
                if !(item_index < mem.read_unaligned()) {
                    break;
                }
            }
        }
        let p = mem
            .byte_add(0x11)
            .cast::<*mut core::ffi::c_void>()
            .read_unaligned();
        if !p.is_null() {
            HeapFree(DAT_10098224, 1, p);
            mem.byte_add(0x11).cast::<u32>().write_unaligned(0);
        }
        HeapFree(DAT_10098224, 1, mem.cast());
    }
    0
}
