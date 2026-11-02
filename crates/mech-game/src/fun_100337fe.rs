// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7b64: u32;
    static mut DAT_100adb04: u32;
    static mut DAT_100adb08: u32;
    static mut DAT_100adb0c: u32;
    static mut DAT_100adb48: u32;
    static mut DAT_100adf80: u32;
    static mut DAT_100b9a18: u32;
    static mut DAT_100b9a1c: u32;
    static mut DAT_100c6b38: u32;
    static mut DAT_100c6b3c: u32;
    static mut DAT_100c6b40: [u8; 0];
    static mut DAT_100c6b58: u32;
    static mut DAT_101f5300: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10058e74(param_1: *mut i32, param_2: u8) -> u32;
    fn FUN_10056fc4(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100337fe() {
    mech_trace::enter!("100337fe");
    // SAFETY: globals and callees are the game's own; the vtable-like
    // pointers are read from the game's objects and called as the original does.
    unsafe {
        if mech_trace::gr!("100337fe", "100a7b64", DAT_100a7b64) != 0
            && mech_trace::gr!("100337fe", "100adb04", DAT_100adb04) != 0
            && mech_trace::gr!("100337fe", "100adb08", DAT_100adb08) != 0
            && 1 < mech_trace::gr!("100337fe", "100adb0c", DAT_100adb0c)
        {
            let local_8: i32;
            if mech_trace::gr!("100337fe", "100adf80", DAT_100adf80) == 0 {
                local_8 = -1;
            } else {
                let f = mech_trace::rd!(
                    "100337fe",
                    (mech_trace::gr!("100337fe", "100b9a18", DAT_100b9a18).wrapping_add(0x20)
                        as *const usize),
                    read_unaligned
                );
                let f: extern "C" fn() -> i32 = core::mem::transmute(f);
                local_8 = mech_trace::cr!("100337fe", "ind", f());
            }
            if local_8 == 0 {
                mech_trace::gw!(
                    "100337fe",
                    "100c6b58",
                    DAT_100c6b58,
                    mech_trace::gr!("100337fe", "101f5300", DAT_101f5300)
                );
                mech_trace::cr!(
                    "100337fe",
                    "10058e74",
                    FUN_10058e74((&raw mut DAT_100c6b40).cast::<i32>(), 0)
                );
                mech_trace::cr!(
                    "100337fe",
                    "10056fc4",
                    FUN_10056fc4(
                        (&raw mut DAT_100c6b40).cast::<i32>(),
                        mech_trace::gr!("100337fe", "100adb04", DAT_100adb04) as i32,
                        0,
                        0,
                        0,
                    )
                );
                mech_trace::cr!(
                    "100337fe",
                    "10056fc4",
                    FUN_10056fc4(
                        (&raw mut DAT_100c6b40).cast::<i32>(),
                        mech_trace::gr!("100337fe", "100adb08", DAT_100adb08) as i32,
                        mech_trace::gr!("100337fe", "100adb48", DAT_100adb48) as i32,
                        mech_trace::gr!("100337fe", "100c6b3c", DAT_100c6b3c) as i32,
                        mech_trace::gr!("100337fe", "100c6b38", DAT_100c6b38) as i32,
                    )
                );
                if mech_trace::gr!("100337fe", "100adf80", DAT_100adf80) != 0 {
                    let g = mech_trace::rd!(
                        "100337fe",
                        (mech_trace::gr!("100337fe", "100b9a1c", DAT_100b9a1c).wrapping_add(0x18)
                            as *const usize),
                        read_unaligned
                    );
                    let g: extern "C" fn() = core::mem::transmute(g);
                    mech_trace::cr!("100337fe", "ind", g());
                }
                mech_trace::gw!(
                    "100337fe",
                    "100adb48",
                    DAT_100adb48,
                    mech_trace::gr!("100337fe", "100adb48", DAT_100adb48).wrapping_add(1)
                        % mech_trace::gr!("100337fe", "100adb0c", DAT_100adb0c)
                );
            }
        }
    }
}
