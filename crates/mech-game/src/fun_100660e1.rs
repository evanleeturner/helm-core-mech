// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(a: i32, b: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100660e1(frame: i32, source: *mut i32, out: *mut i32) -> *mut i32 {
    // SAFETY: frame is the caller's address of a structure of ints; source and
    // out point at the caller's two-int arrays; the callee is the game's own.
    unsafe {
        let i_var3 = ((frame + 0x10) as *const i32).read_unaligned();
        let i_var1 = ((frame + 8) as *const i32).read_unaligned();
        let a = ((frame + 0xc) as *const i32).read_unaligned();
        let b = ((frame + 4) as *const i32).read_unaligned();
        let i_var2 = FUN_10003d60(a.wrapping_sub(b), source.read_unaligned());
        out.write_unaligned(i_var2);
        let i_var3 = FUN_10003d60(i_var3.wrapping_sub(i_var1), source.add(1).read_unaligned());
        out.add(1).write_unaligned(i_var3);
        out
    }
}
