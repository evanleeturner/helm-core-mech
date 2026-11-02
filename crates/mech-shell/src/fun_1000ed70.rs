// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008da60: i32;
    static mut DAT_1008da88: i32;
    static mut DAT_1008da8c: i32;
    static mut DAT_1008da9c: i32;
    static mut DAT_1008daa0: i32;
    static mut DAT_10098204: i32;
    static mut DAT_100a672c: *mut u8;
    static mut DAT_100a7248: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn SmackWait(smk: i32) -> i32;
    fn SmackDoFrame(smk: i32);
    fn SmackNextFrame(smk: i32);
    fn FUN_1000f290(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10012960(param_1: *mut u8, edx_unused: i32, param_2: *mut u32, param_3: i32);
    fn FUN_100124d0(
        param_1: *mut u8,
        edx_unused: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000ed70() {
    // SAFETY: globals and callees are the game's own; the Smacker handle and
    // the object pointers are read at the offsets the original uses.
    unsafe {
        let mut i_var1 = SmackWait(DAT_1008da60);
        if i_var1 == 0 {
            if DAT_1008da9c == 1 {
                DAT_100a672c
                    .byte_add(0x3aa)
                    .cast::<u32>()
                    .write_unaligned(1);
            }
            if ((DAT_1008da60.wrapping_add(0x68)) as *const i32).read_unaligned() != 0 {
                FUN_10012960(
                    DAT_100a672c,
                    0,
                    (DAT_1008da60.wrapping_add(0x6c)) as *mut u32,
                    0,
                );
            }
            i_var1 = -1;
            if DAT_10098204 != 0 {
                let f = (DAT_100a7248.wrapping_add(0x20) as *const usize).read_unaligned();
                let f: extern "C" fn() -> i32 = core::mem::transmute(f);
                i_var1 = f();
            }
            if i_var1 == 0 {
                SmackDoFrame(DAT_1008da60);
                FUN_100124d0(DAT_100a672c, 0, 0, 0, DAT_1008da88, DAT_1008da8c);
            }
            DAT_1008da9c = DAT_1008da9c.wrapping_add(1);
            if DAT_1008daa0 < DAT_1008da9c {
                FUN_1000f290(0);
                return;
            }
            SmackNextFrame(DAT_1008da60);
        }
    }
}
