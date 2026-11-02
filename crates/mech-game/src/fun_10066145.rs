// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(a: u32, b: i32) -> i32;
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10066145(param_1: i32, param_2: u32) -> i32 {
    // SAFETY: the callees are the game's own; param_1 points at the caller's
    // struct, with fields read and written at the original's offsets.
    unsafe {
        let p = param_1 as *mut u8;
        let i_var1 = FUN_10002570(param_2, 0xd555);
        let mut v = FUN_10003d60(p.byte_add(4).cast::<i32>().read_unaligned(), 0x13f);
        p.byte_add(4).cast::<i32>().write_unaligned(v);
        v = FUN_10003d60(p.byte_add(8).cast::<i32>().read_unaligned(), 199);
        p.byte_add(8).cast::<i32>().write_unaligned(v);
        v = FUN_10003d60(p.byte_add(8).cast::<i32>().read_unaligned(), i_var1);
        p.byte_add(8).cast::<i32>().write_unaligned(v);
        v = FUN_10003d60(p.byte_add(0xc).cast::<i32>().read_unaligned(), 0x13f);
        p.byte_add(0xc).cast::<i32>().write_unaligned(v);
        v = FUN_10003d60(p.byte_add(0x10).cast::<i32>().read_unaligned(), 199);
        p.byte_add(0x10).cast::<i32>().write_unaligned(v);
        v = FUN_10003d60(p.byte_add(0x10).cast::<i32>().read_unaligned(), i_var1);
        p.byte_add(0x10).cast::<i32>().write_unaligned(v);
    }
    param_1
}
