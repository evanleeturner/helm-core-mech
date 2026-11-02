// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001bdc6(param_1: *mut u16);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005ff34(
    param_1: *mut u16,
    param_2: i32,
    param_3: u32,
    param_4: i32,
) -> u32 {
    let _ = param_3;
    // SAFETY: the callee is the game's own; param_1 points at the caller's
    // object, whose field at offset 0x4c holds a pointer to a bounds record.
    unsafe {
        FUN_1001bdc6(param_1);
        let p_var1 = param_1.add(0x26).cast::<*const i32>().read_unaligned();
        if p_var1.is_null() {
            return 0;
        }
        let b0 = p_var1.read_unaligned();
        let b1 = p_var1.add(1).read_unaligned();
        let b4 = p_var1.add(4).read_unaligned();
        let b5 = p_var1.add(5).read_unaligned();
        if param_2 < b0 || b1 < param_2 || param_4 < b4 || b5 < param_4 {
            0
        } else {
            1
        }
    }
}
