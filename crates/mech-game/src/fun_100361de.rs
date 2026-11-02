// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10036142(param_1: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100361de(param_1: u32, param_2: i32) -> u32 {
    let _ = param_1;
    if param_2 == 0 {
        return 0;
    }
    // SAFETY: param_2 is the address of the caller's structure; the reads and
    // writes stay at the offsets the original uses, and the callee is the game's own.
    unsafe {
        let i_var2 = FUN_10036142((param_2.wrapping_add(8) as *const i32).read_unaligned());
        if i_var2 == 0 {
            let n = (param_2.wrapping_add(0xc) as *const i32).read_unaligned();
            let src = n
                .wrapping_mul(0x14)
                .wrapping_add(0x18)
                .wrapping_add(param_2) as *const u32;
            let dst = param_2.wrapping_add(0x18) as *mut u32;
            let mut i = 0usize;
            while i < 5 {
                dst.add(i).write_unaligned(src.add(i).read_unaligned());
                i += 1;
            }
            let n = (param_2.wrapping_add(0xc) as *const i32).read_unaligned();
            let src = n
                .wrapping_add(-1)
                .wrapping_mul(0x14)
                .wrapping_add(0x18)
                .wrapping_add(param_2) as *const u32;
            let dst = param_2.wrapping_add(0x2c) as *mut u32;
            let mut i = 0usize;
            while i < 5 {
                dst.add(i).write_unaligned(src.add(i).read_unaligned());
                i += 1;
            }
            (param_2.wrapping_add(0xc) as *mut i32).write_unaligned(2);
        }
    }
    1
}
