// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100660e1(param_1: i32, param_2: *mut i32, param_3: *mut i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10015bc6(param_1: i32) {
    let base = param_1 as usize as *mut u8;
    // SAFETY: param_1 is the address of the caller's game object; fields are
    // written and read at the original's offsets, and the callee is the game's own.
    unsafe {
        base.byte_add(0x84).cast::<u32>().write_unaligned(0);
        base.byte_add(0x88).cast::<u32>().write_unaligned(0x3333);
        base.byte_add(0x8c).cast::<u32>().write_unaligned(0xea4e);
        base.byte_add(0x90).cast::<u32>().write_unaligned(0x5555);
        FUN_100660e1(
            base.byte_add(0xa4).cast::<i32>().read_unaligned(),
            base.byte_add(0x8c).cast::<i32>(),
            base.byte_add(0x8c).cast::<i32>(),
        );
        FUN_100660e1(
            base.byte_add(0xa4).cast::<i32>().read_unaligned(),
            base.byte_add(0x84).cast::<i32>(),
            base.byte_add(0x84).cast::<i32>(),
        );
    }
}
