// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001be70(param_1: i32) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001bee2(param_1: *mut u16) {
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the caller's object; the callee is the game's own.
    unsafe {
        if (param_1.read_unaligned() & 0x200) != 0 {
            return;
        }
        let field = param_1.byte_add(0x4c).cast::<i32>();
        if field.read_unaligned() == 0 {
            let p = FUN_1001be70(param_1 as usize as i32);
            field.write_unaligned(p as usize as i32);
        }
        if field.read_unaligned() != 0 {
            param_1.write_unaligned(param_1.read_unaligned() | 0x200);
        }
    }
}
