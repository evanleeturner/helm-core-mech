// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10002570(a: u32, b: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100662fd(source: *mut u32, out: *mut u32) -> *mut u32 {
    // SAFETY: the callee is the game's own; source and out point at the
    // caller's two-word buffers, read and written where the original does.
    unsafe {
        let u_var1 = FUN_10002570(source.read_unaligned(), 0x13f);
        out.write_unaligned(u_var1);
        let u_var1 = FUN_10002570(source.add(1).read_unaligned(), 199);
        out.add(1).write_unaligned(u_var1);
        out
    }
}
