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
    mech_trace::enter!("1001a9f9");
    // SAFETY: the globals and callees are the game's own; the pointers
    // passed are the game's own buffers, used as the original does.
    unsafe {
        if mech_trace::gr!("1001a9f9", "100ab07c", DAT_100ab07c) != 3 {
            mech_trace::gw!(
                "1001a9f9",
                "100ab080",
                DAT_100ab080,
                mech_trace::gr!("1001a9f9", "100ab07c", DAT_100ab07c)
            );
            mech_trace::cr!(
                "1001a9f9",
                "1001a6ca",
                FUN_1001a6ca(
                    mech_trace::gr!("1001a9f9", "100aaf68", DAT_100aaf68) as *mut u32,
                    (&raw mut DAT_101eb330).cast::<u32>(),
                )
            );
            mech_trace::gw!("1001a9f9", "100ab078", DAT_100ab078, 0x20000);
            mech_trace::cr!("1001a9f9", "1001a635", FUN_1001a635(0));
        }
        let pu_var1 = mech_trace::cr!("1001a9f9", "1004baf4", FUN_1004baf4());
        if pu_var1.is_null() {
            mech_trace::cr!(
                "1001a9f9",
                "1001a5e1",
                FUN_1001a5e1(mech_trace::gr!("1001a9f9", "100ab080", DAT_100ab080) as i32)
            );
            if mech_trace::gr!("1001a9f9", "100ab080", DAT_100ab080) == 2 {
                mech_trace::cr!(
                    "1001a9f9",
                    "1001a74a",
                    FUN_1001a74a(
                        mech_trace::gr!("1001a9f9", "100aaf68", DAT_100aaf68) as *mut u32,
                        (&raw mut DAT_101eb330).cast::<u32>(),
                    )
                );
            }
        } else {
            mech_trace::cr!(
                "1001a9f9",
                "1001a74a",
                FUN_1001a74a(
                    mech_trace::gr!("1001a9f9", "100aaf68", DAT_100aaf68) as *mut u32,
                    pu_var1
                )
            );
        }
    }
}
