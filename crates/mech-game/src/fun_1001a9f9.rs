// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab078: u32;
    static mut DAT_100ab07c: u32;
    static mut DAT_100ab080: u32;
    static mut DAT_101eb330: [u8; 1];
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a6ca(param_1: *mut u32, param_2: *mut u32) -> u32;
    fn FUN_1001a635(param_1: i32) -> u8;
    fn FUN_1001a74a(param_1: *mut u32, param_2: *mut u32) -> u32;
    fn FUN_1001a5e1(param_1: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004baf4() -> *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1001a9f9() {
    // SAFETY: the globals and callees are the game's own; the pointers
    // passed are the game's own buffers, used as the original does.
    unsafe {
        if DAT_100ab07c != 3 {
            DAT_100ab080 = DAT_100ab07c;
            FUN_1001a6ca(
                DAT_100aaf68 as *mut u32,
                (&raw mut DAT_101eb330).cast::<u32>(),
            );
            DAT_100ab078 = 0x20000;
            FUN_1001a635(0);
        }
        let pu_var1 = FUN_1004baf4();
        if pu_var1.is_null() {
            FUN_1001a5e1(DAT_100ab080 as i32);
            if DAT_100ab080 == 2 {
                FUN_1001a74a(
                    DAT_100aaf68 as *mut u32,
                    (&raw mut DAT_101eb330).cast::<u32>(),
                );
            }
        } else {
            FUN_1001a74a(DAT_100aaf68 as *mut u32, pu_var1);
        }
    }
}
