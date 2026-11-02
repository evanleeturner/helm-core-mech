// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: *mut u8;
    static mut DAT_100ab07c: i32;
    static mut DAT_100ab0b0: i32;
    static mut DAT_100ab0b4: i32;
    static mut DAT_100ab0b8: i32;
    static mut DAT_100b38fc: i32;
    static mut DAT_100b3904: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a8a3(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: *mut i32,
        param_4: *mut i32,
        param_5: *mut i32,
        param_6: *mut i32,
    );
    fn FUN_1001a635(param_1: i32);
    fn FUN_1001b520(param_1: *mut i32, param_2: *mut i32, param_3: i32, param_4: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001b0bb() {
    let mut local_1c: i32 = 0;
    let mut local_18: i32 = 0;
    let mut local_14: i32 = 0;
    let mut local_10: i32 = 0;
    let mut local_c: i32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: the callees and globals are the game's own; the record pointer
    // refers to the game's record, accessed at the original's offsets.
    unsafe {
        FUN_1001a8a3(
            &mut local_10,
            &mut local_14,
            &mut local_18,
            &mut local_1c,
            &mut local_8,
            &mut local_c,
        );
        if DAT_100ab07c != 4 {
            DAT_100ab0b0 = 0;
            DAT_100aaf68
                .byte_add(0x10)
                .cast::<i32>()
                .write_unaligned(0x5a0000);
            DAT_100ab0b8 = DAT_100b38fc;
            FUN_1001a635(0);
        }
        FUN_1001b520(
            DAT_100aaf68.byte_add(4).cast::<i32>(),
            &raw mut DAT_100ab0b0,
            DAT_100ab0b4,
            DAT_100b3904,
        );
        DAT_100aaf68.cast::<i32>().write_unaligned(local_1c);
        DAT_100aaf68
            .byte_add(8)
            .cast::<i32>()
            .write_unaligned(local_c);
        let old = DAT_100aaf68.byte_add(0xc).cast::<i32>().read_unaligned() as u32;
        let delta = (DAT_100b38fc as u32)
            .wrapping_sub(DAT_100ab0b8 as u32)
            .wrapping_mul(300);
        DAT_100aaf68
            .byte_add(0xc)
            .cast::<i32>()
            .write_unaligned(old.wrapping_add(delta) as i32);
    }
}
