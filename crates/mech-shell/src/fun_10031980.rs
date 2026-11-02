// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000a6f0(param_1: i32);
    fn FUN_1000a890(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10031980(param_1: i32) {
    // SAFETY: param_1 is the address of a game object; the field at offset
    // 0x22c is read and written where the original does, and the callees are
    // the game's own.
    unsafe {
        let p = (param_1 as usize).wrapping_add(0x22c) as *mut i32;
        p.write_unaligned(p.read_unaligned().wrapping_sub(100));
        FUN_1000a6f0(param_1);
        FUN_1000a890(param_1);
    }
}
