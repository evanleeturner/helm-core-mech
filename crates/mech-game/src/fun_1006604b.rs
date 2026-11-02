// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006604b(frame: i32, source: *mut i32, out: *mut i32) -> *mut i32 {
    // SAFETY: frame is an address of the caller's structure; source and out
    // point at the caller's four-int arrays; the callee is the game's own.
    unsafe {
        let i_var1 = ((frame.wrapping_add(0xc) as *const i32).read_unaligned() as u32)
            .wrapping_sub((frame.wrapping_add(4) as *const i32).read_unaligned() as u32)
            as i32;
        let i_var2 = ((frame.wrapping_add(0x10) as *const i32).read_unaligned() as u32)
            .wrapping_sub((frame.wrapping_add(8) as *const i32).read_unaligned() as u32)
            as i32;
        let i_var3 = FUN_10003d60(i_var1, source.read_unaligned());
        out.write_unaligned(i_var3);
        let i_var3 = FUN_10003d60(i_var2, source.add(1).read_unaligned());
        out.add(1).write_unaligned(i_var3);
        let i_var1 = FUN_10003d60(i_var1, source.add(2).read_unaligned());
        out.add(2).write_unaligned(i_var1);
        let i_var1 = FUN_10003d60(i_var2, source.add(3).read_unaligned());
        out.add(3).write_unaligned(i_var1);
    }
    out
}
