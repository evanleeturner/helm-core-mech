// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008255f(param_1: u32, param_2: u32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002c354(param_1: i32) {
    // SAFETY: param_1 points at the caller's structure; the fields are read
    // and written where the original does, and the callee is the game's own.
    unsafe {
        let p = param_1 as usize as *mut u8;
        if p.byte_add(0x1a2).cast::<i32>().read_unaligned() == 0 {
            let a = p.byte_add(4).cast::<u32>().read_unaligned();
            let b = p.byte_add(0x1a2).cast::<u32>().read_unaligned();
            FUN_1008255f(a, b);
            p.byte_add(0x1a2).cast::<u32>().write_unaligned(0x1000);
        }
    }
}
