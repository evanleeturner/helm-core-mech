// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38fc: u32;
    static mut DAT_100b32b8: *mut u32;
    static s_TLISADATANTKANFLOBJICID_100ad6c4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100814bb(size: u32, tag: *const i8) -> *mut u8;
    fn FUN_1005d9bf(param_1: *mut i32, param_2: *mut u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005d8f5(
    param_1: *mut i32,
    param_2: *mut u8,
    param_3: u32,
    param_4: u32,
) -> *mut u32 {
    // SAFETY: the callee and the globals are the game's own; param_1 points at
    // the caller's list head, and the allocated block holds 0x18 bytes.
    unsafe {
        let mut p_var1 = FUN_100814bb(
            0x18,
            (&raw const s_TLISADATANTKANFLOBJICID_100ad6c4).cast::<i8>(),
        )
        .cast::<u32>();
        if p_var1.is_null() {
            return core::ptr::null_mut();
        }
        p_var1.write_unaligned(param_2 as u32);
        p_var1.add(1).write_unaligned(0);
        p_var1.add(2).write_unaligned(param_3);
        p_var1.add(3).write_unaligned(DAT_100b38fc);
        p_var1
            .add(4)
            .write_unaligned(p_var1.add(3).read_unaligned());
        p_var1
            .add(5)
            .write_unaligned(param_1.read_unaligned() as u32);
        param_1.write_unaligned(p_var1 as i32);
        DAT_100b32b8 = param_1.read_unaligned() as *mut u32;
        if !param_2.is_null() {
            let f: unsafe extern "C" fn(i32, u32, u32, u32) -> i32 = core::mem::transmute(param_2);
            let i_var2 = f(0, param_4, DAT_100b38fc, param_3);
            if i_var2 == 0 {
                FUN_1005d9bf(param_1, DAT_100b32b8);
                p_var1 = core::ptr::null_mut();
            }
        }
        p_var1
    }
}
