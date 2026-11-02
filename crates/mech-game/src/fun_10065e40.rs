// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101f52f4: i32;
    static mut DAT_101f52d4: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10065e40(_unused_arg1: u32, source: i32, out: i32) -> i32 {
    let src = source as *const u8;
    let dst = out as *mut u8;
    // SAFETY: the callee and the globals are the game's own; source and out
    // point at the caller's structures, read and written where the original does.
    unsafe {
        let i_var1 = FUN_10003d60(DAT_101f52f4, src.byte_add(4).cast::<i32>().read_unaligned());
        dst.byte_add(4).cast::<i32>().write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(DAT_101f52d4, src.byte_add(8).cast::<i32>().read_unaligned());
        dst.byte_add(8).cast::<i32>().write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(
            DAT_101f52f4,
            src.byte_add(0xc).cast::<i32>().read_unaligned(),
        );
        dst.byte_add(0xc).cast::<i32>().write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(
            DAT_101f52d4,
            src.byte_add(0x10).cast::<i32>().read_unaligned(),
        );
        dst.byte_add(0x10).cast::<i32>().write_unaligned(i_var1);
    }
    out
}
