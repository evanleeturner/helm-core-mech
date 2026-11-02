// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strncpy(dst: *mut i8, src: *mut i8, len: i32);
    fn FUN_1001d9b0(param_1: *mut i16, param_2: *mut u32) -> *mut i32;
    fn FUN_1001dd9f(param_1: *mut i32);
    fn FUN_100108a0(param_1: i32, param_2: *mut i8);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100327f6(param_1: i32, param_2: *mut u8) -> u32 {
    let mut local_48: [u32; 8] = [0; 8];
    let mut local_28: [u16; 8] = [0; 8];
    let local_10: *mut i16 = local_28.as_mut_ptr().cast::<i16>();
    let mut local_8: u32 = 0;

    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // record, and the local buffer holds the 16 bytes the original's frame does.
    unsafe {
        let local_18: i32 = ((param_1.wrapping_add(10)) as *const i16).read_unaligned() as i32;
        _strncpy(
            local_10.cast::<i8>().byte_add(2),
            (param_1.wrapping_add(0x24)) as *mut i8,
            0xc,
        );
        local_10.byte_add(14).cast::<i8>().write(0);
        local_10.write_unaligned(local_18 as i16);
        let local_c = FUN_1001d9b0(local_10, local_48.as_mut_ptr());
        if local_c.is_null() {
            FUN_100108a0(0x35, core::ptr::null_mut());
        } else {
            local_8 = 1;
            let f: unsafe extern "C" fn(*mut i32) -> u32 = core::mem::transmute(param_2);
            local_8 = f(local_c);
            FUN_1001dd9f(local_c);
        }
    }
    local_8
}
