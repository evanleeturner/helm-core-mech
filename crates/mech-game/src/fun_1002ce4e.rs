// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c2dec: u32;
    static mut PTR_DAT_100c2be0: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn strncpy(dest: *mut i8, source: *const i8, count: u32) -> *mut i8;
    fn _isctype(c: i32, ty: i32) -> i32;
    fn atoi(s: *const i8) -> i32;
    fn FUN_1001d9b0(param_1: *mut i16, param_2: *mut u32) -> *mut u32;
    fn FUN_1002cf43(param_1: i32) -> u32;
    fn FUN_1001dd9f(param_1: *mut i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1002d233() -> *mut u8;
    fn FUN_1004726c();
    fn FUN_1003289f();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002ce4e(param_1: *mut i8) -> *mut u8 {
    let mut buf: [u8; 16] = [0; 16];
    let mut local_40: [u32; 8] = [0; 8];
    let mut local_10: *mut u8 = core::ptr::null_mut();
    let local_c: *mut i16 = buf.as_mut_ptr().cast::<i16>();
    // SAFETY: param_1 points at the caller's string; the globals and callees
    // are the game's own; buf and local_40 are local storage large enough for
    // the writes made here.
    unsafe {
        strncpy(buf.as_mut_ptr().add(2).cast::<i8>(), param_1, 0xc);
        *buf.as_mut_ptr().add(14) = 0;
        let local_44: u32 = if DAT_100c2dec < 2 {
            let addr = PTR_DAT_100c2be0.wrapping_add((*param_1 as i32).wrapping_mul(2) as u32);
            (((addr as *const u16).read_unaligned()) & 4) as u32
        } else {
            _isctype(*param_1 as i32, 4) as u32
        };
        if local_44 == 0 {
            local_c.write_unaligned(-1);
        } else {
            let i_var1 = atoi(param_1);
            local_c.write_unaligned(i_var1 as i16);
        }
        let local_8 = FUN_1001d9b0(local_c, local_40.as_mut_ptr()) as *mut i32;
        if !local_8.is_null() {
            let u_var2 = FUN_1002cf43(local_8 as i32);
            if u_var2 != 0 {
                local_10 = FUN_1002d233();
            }
            FUN_1001dd9f(local_8);
            FUN_1004726c();
            FUN_1003289f();
        }
    }
    local_10
}
