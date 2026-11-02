// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1005ade4: [i16; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005adf0(param_1: *mut i16, param_2: i32) -> *mut i16;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b02a(param_1: i32) -> u32 {
    // SAFETY: the callee and the global table are the game's own; the returned
    // pointer addresses at least two 16-bit words, read where the original does.
    unsafe {
        let ps_var1 = FUN_1005adf0((&raw mut DAT_1005ade4).cast::<i16>(), param_1);
        let w0 = ps_var1.read_unaligned() as u16;
        let w1 = ps_var1.byte_add(2).read_unaligned() as u16;
        ((u32::from(w0) & 0xff) << 24)
            | (u32::from(w0 >> 8) << 16)
            | ((u32::from(w1) & 0xff) << 8)
            | u32::from(w1 >> 8)
    }
}
