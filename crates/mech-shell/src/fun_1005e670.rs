// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a50f4: u32;
    static mut DAT_100a62c0: u32;
    static mut DAT_100a62cc: u32;
    static mut DAT_100ab6e8: [u8; 0];
    static mut DAT_100c6064: u32;
    static mut DAT_100a6364: [i8; 0];
    static mut DAT_100a6394: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1005b430() -> u32;
    fn FUN_1000d930() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict__fwprintf(file: *mut u8, format: *const i8, ...) -> i32;
    fn FUN_1006fde0(param_1: u32) -> u8;
    fn FID_conflict__wprintf(format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005e670() {
    // SAFETY: the callees and globals are the game's own memory; the
    // pointers passed are addresses of the game's file object and strings.
    unsafe {
        DAT_100a62c0 = FUN_1005b430();
        let u_var1 = FUN_1000d930();
        DAT_100a62c0 = DAT_100a62c0 & u_var1;
        if DAT_100a62cc == 0 {
            DAT_100a62cc = 1;
        } else {
            FID_conflict__fwprintf(
                (&raw mut DAT_100ab6e8).cast::<u8>(),
                (&raw const DAT_100a6364).cast::<i8>(),
            );
            FUN_1006fde0(0);
        }
        if DAT_100a62c0 == 0 {
            FID_conflict__wprintf((&raw const DAT_100a6394).cast::<i8>());
            DAT_100a50f4 = 1;
        }
        DAT_100c6064 = 0;
    }
}
