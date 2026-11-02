// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10025f6e(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: *mut i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10060161(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: *mut i32,
) {
    // SAFETY: param_1 is the caller's object address, read at offsets 0x4c and
    // 0x1c as the original does; the callee is the game's own.
    unsafe {
        let a = ((param_1 as u32).wrapping_add(0x4c) as *const i32).read_unaligned();
        let b = ((param_1 as u32).wrapping_add(0x1c) as *const i32).read_unaligned();
        FUN_10025f6e(a, b, param_2, param_3, param_4, param_5);
    }
}
