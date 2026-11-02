// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002fcce(mem: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut core::ffi::c_void, flags: u32, mem: *mut core::ffi::c_void) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002fe1e(mem: *mut i32, param_2: *mut u8) {
    // SAFETY: mem points at the game's node structure; its child list,
    // callback and flags are read as the original does, and the callee,
    // callback and HeapFree are the game's own.
    unsafe {
        FUN_1002fcce(mem);
        let mut local_8: *mut i32 = mem.add(1).read_unaligned() as usize as *mut i32;
        while !local_8.is_null() {
            let pi_var1 = local_8.add(2).read_unaligned() as usize as *mut i32;
            FUN_1002fe1e(local_8, param_2);
            local_8 = pi_var1;
        }
        let cb_arg = mem.add(0x1b).read_unaligned();
        if cb_arg != 0 && !param_2.is_null() {
            let f: unsafe extern "C" fn(i32) =
                core::mem::transmute::<*mut u8, unsafe extern "C" fn(i32)>(param_2);
            f(cb_arg);
        }
        let flags = mem.add(0x1d).cast::<u8>().read();
        if (flags & 4) != 0 {
            HeapFree(DAT_100adf74, 1, mem.cast());
        }
    }
}
