// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009f378: u32;
    static mut DAT_1009fc2c: [u8; 0];
    static mut DAT_1009fc20: [u8; 0];
    static mut DAT_1009fc18: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100576a0(param_1: i32) -> i32;
    fn FID_conflict__fwprintf(file: *mut u8, format: *const u8, ...) -> i32;
    fn FUN_10053470(param_1: *mut u8, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10053500(param_1: *mut u8, param_2: u32, param_3: i32) {
    // SAFETY: the callees and globals are the game's own; param_3 points at
    // the caller's structure and iVar2 at the object the callee returns, read
    // where the original reads them.
    unsafe {
        let i_var2 =
            FUN_100576a0(((param_3 as usize).wrapping_add(4) as *const i32).read_unaligned());
        let i_var1 = ((param_3 as usize).wrapping_add(0x10) as *const i32).read_unaligned();
        if -1 < i_var1 {
            DAT_1009f378 = DAT_1009f378.wrapping_add(1);
            FID_conflict__fwprintf(param_1, (&raw const DAT_1009fc18).cast::<u8>(), param_2);
            let flags = ((param_3 as usize).wrapping_add(8) as *const u32).read_unaligned();
            let neg = ((flags & 0x8000_0000) == 0) as i32;
            let ch = ((neg.wrapping_neg() & 0xfe).wrapping_add(0x2d)) as i8;
            let table = ((i_var2 as usize).wrapping_add(0x68) as *const i32).read_unaligned();
            let val = ((table as usize).wrapping_add((i_var1 as usize).wrapping_mul(4))
                as *const u32)
                .read_unaligned();
            FID_conflict__fwprintf(
                param_1,
                (&raw const DAT_1009fc20).cast::<u8>(),
                ch as i32,
                i_var2.wrapping_add(4),
                val,
            );
            FUN_10053470(param_1, param_3);
            FID_conflict__fwprintf(param_1, (&raw const DAT_1009fc2c).cast::<u8>());
        }
    }
}
