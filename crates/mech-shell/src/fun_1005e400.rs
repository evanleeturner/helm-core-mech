// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100c61b4: *mut i8;
    static mut DAT_100a50f0: i32;
    static mut DAT_10095ad4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn __ftol() -> i32;
    fn strncpy(dest: *mut i8, src: *mut i8, n: u32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005e400(
    param_1: u32,
    param_2: u32,
    param_3: u32,
    _param_4: u32,
    _param_5: u32,
    param_6: u16,
    param_7: u16,
    param_8: u16,
    param_9: u16,
    param_10: u16,
    param_11: u16,
    source: *mut i8,
) {
    // SAFETY: the globals and callees are the game's own; DAT_100c61b4 points
    // at a writable record buffer with room for the 0x50-byte entry.
    unsafe {
        DAT_100c61b4.cast::<u32>().write_unaligned(DAT_10095ad4);
        let pc_var1 = DAT_100c61b4;
        pc_var1.byte_add(4).write(b'P' as i8);
        pc_var1.byte_add(5).write(0);
        pc_var1.byte_add(6).write(0);
        pc_var1.byte_add(7).write(0);
        DAT_100c61b4
            .byte_add(8)
            .cast::<u32>()
            .write_unaligned(param_1);
        DAT_100c61b4
            .byte_add(0xc)
            .cast::<u32>()
            .write_unaligned(param_2);
        DAT_100c61b4
            .byte_add(0x10)
            .cast::<u32>()
            .write_unaligned(param_3);
        let l_var2 = __ftol();
        DAT_100c61b4
            .byte_add(0x14)
            .cast::<i32>()
            .write_unaligned(l_var2);
        DAT_100c61b4
            .byte_add(0x18)
            .cast::<u16>()
            .write_unaligned(param_6);
        DAT_100c61b4
            .byte_add(0x1a)
            .cast::<u16>()
            .write_unaligned(param_7);
        DAT_100c61b4
            .byte_add(0x1c)
            .cast::<u16>()
            .write_unaligned(param_8);
        DAT_100c61b4
            .byte_add(0x1e)
            .cast::<u16>()
            .write_unaligned(param_9);
        DAT_100c61b4
            .byte_add(0x20)
            .cast::<u16>()
            .write_unaligned(param_10);
        DAT_100c61b4
            .byte_add(0x22)
            .cast::<u16>()
            .write_unaligned(param_11);
        strncpy(DAT_100c61b4.byte_add(0x24), source, 0x15);
        DAT_100c61b4 = DAT_100c61b4.byte_add(0x50);
        if DAT_100a50f0 < 0x51 {
            DAT_100a50f0 = 0x50;
        }
    }
}
