// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10025c83(param_1: *mut i32, param_2: i32, param_3: *mut i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060134(param_1: i32, param_2: *mut i32) -> u32 {
    // SAFETY: param_1 is the address of a game object; the fields at 0x4c and
    // 0x1c are read where the original reads them, and the callee is the game's own.
    unsafe {
        let p = (param_1 as usize).wrapping_add(0x4c) as *const *mut i32;
        let a = p.read_unaligned();
        let q = (param_1 as usize).wrapping_add(0x1c) as *const i32;
        let b = q.read_unaligned();
        let u_var1 = FUN_10025c83(a, b, param_2);
        u_var1 & 1
    }
}
