// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005d5c1() -> u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10030fd2(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100227cb(param_1: *mut i32) {
    if !param_1.is_null() {
        // SAFETY: param_1 points at a pointer to the game's object; the globals
        // and callees are the game's own, accessed in the original's order.
        unsafe {
            let base = param_1.read_unaligned() as usize;
            if (base as *const u8)
                .byte_add(4)
                .cast::<u32>()
                .read_unaligned()
                == DAT_100adf38
            {
                FUN_1005d5c1();
            }
            let base = param_1.read_unaligned() as usize;
            let v = (base as *const u8)
                .byte_add(0x140)
                .cast::<i32>()
                .read_unaligned();
            FUN_10030fd2(v);
            let base = param_1.read_unaligned() as usize;
            (base as *mut u8)
                .byte_add(0x140)
                .cast::<i32>()
                .write_unaligned(-1);
        }
    }
}
