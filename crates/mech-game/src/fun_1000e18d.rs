// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut u8;
    static mut DAT_100af0d0: u32;
    static mut DAT_100af13c: *mut i8;
    static mut DAT_100a75f0: [i8; 13];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncpy(dest: *mut i8, source: *const i8, count: u32) -> *mut i8;
    fn FUN_1007bbed(
        param_1: *mut i16,
        param_2: *mut i32,
        param_3: *mut i8,
        param_4: u32,
        param_5: *mut u32,
        param_6: *mut i32,
    ) -> *mut u16;
    fn FUN_1000e242(param_1: *mut i32, param_2: i32, param_3: i32) -> u8;
    fn FUN_1008664f(param_1: u32, param_2: *mut i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapFree(h: *mut u8, flags: u32, mem: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000e18d(param_1: *mut i32) {
    let mut buf: [u8; 16] = [0; 16];
    let mut local_8: u32 = 0;
    let local_c: *mut i16 = buf.as_mut_ptr().cast::<i16>();
    // SAFETY: buf is a 16-byte local mirroring the original stack frame
    // (short at 0, 14 chars after it); the globals and callees are the game's own.
    unsafe {
        local_c.write_unaligned(-1);
        _strncpy(
            buf.as_mut_ptr().add(2).cast::<i8>(),
            (&raw mut DAT_100a75f0).cast::<i8>(),
            0xc,
        );
        buf.as_mut_ptr().add(14).write(0);
        let lp_mem = FUN_1007bbed(
            local_c,
            DAT_100af0d0 as *mut i32,
            DAT_100af13c,
            2,
            &raw mut local_8,
            core::ptr::null_mut(),
        );
        if !lp_mem.is_null() {
            FUN_1000e242(param_1, lp_mem as i32, local_8 as i32);
            if local_c.read_unaligned() == -1 {
                HeapFree(DAT_100adf74, 1, lp_mem.cast::<u8>());
            } else {
                FUN_1008664f(
                    local_c.read_unaligned() as i32 as u32,
                    DAT_100af0d0 as *mut i32,
                );
            }
        }
    }
}
