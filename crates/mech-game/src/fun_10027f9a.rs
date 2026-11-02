// SPDX-License-Identifier: MIT

use core::mem::MaybeUninit;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10083a59(
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: *mut i32,
        param_5: *mut u32,
        param_6: *mut u32,
        param_7: *mut i32,
    ) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10027f9a(param_1: *mut i32, param_2: *mut i32) -> u32 {
    let mut local_c: MaybeUninit<u32> = MaybeUninit::uninit();
    let mut local_8: MaybeUninit<i32> = MaybeUninit::uninit();
    // SAFETY: param_1 and param_2 point at the caller's three-int vectors;
    // the callee is the game's own and receives pointers to live locals.
    unsafe {
        let a = param_1
            .read_unaligned()
            .wrapping_sub(param_2.read_unaligned());
        let b = param_1
            .byte_add(4)
            .read_unaligned()
            .wrapping_sub(param_2.byte_add(4).read_unaligned());
        let c = param_1
            .byte_add(8)
            .read_unaligned()
            .wrapping_sub(param_2.byte_add(8).read_unaligned());
        let p8 = local_8.as_mut_ptr();
        FUN_10083a59(
            a as u32,
            b as u32,
            c as u32,
            p8,
            p8.cast::<u32>(),
            local_c.as_mut_ptr(),
            p8,
        );
        local_c.assume_init()
    }
}
