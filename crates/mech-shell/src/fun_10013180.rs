// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008eb64: u32;
    static mut DAT_10098204: u32;
    static mut DAT_100a7248: u32;
    static mut DAT_100b0e38: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10069697(x: i32, y: i32) -> u32;
    fn FUN_1006984a(
        x: *mut i32,
        y: i32,
        font: i32,
        text: i32,
        param_5: *mut u8,
        param_6: i32,
    ) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10012590(this: *mut u8, x: i32, y: i32, font: i32, text: i32) -> u32;
    fn FUN_100124d0(this: *mut u8, x: i32, y: i32, font: i32, text: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10013180(
    this: *mut u8,
    x: i32,
    y: i32,
    font: i32,
    text: *mut u8,
    param_5: *mut u8,
) -> i32 {
    mech_trace::enter!("10013180", this, x, y, font, text, param_5);
    if text.is_null() {
        return mech_trace::rt!("10013180", 0);
    }
    let mut param_5 = param_5;
    // SAFETY: the callees and globals are the game's own; text points at a
    // NUL-terminated string and font at the game's font structure.
    unsafe {
        let mut width: i32 = 0;
        let mut cur: u8 = mech_trace::dr!("10013180", text);
        let mut cursor = text;
        while cur != 0 {
            cur = mech_trace::dr!("10013180", cursor);
            cursor = cursor.add(1);
            let r = mech_trace::cr!(
                "10013180",
                "10069697",
                FUN_10069697(font, (cur as i8) as i32)
            );
            width = width.wrapping_add(r as i32);
            cur = mech_trace::dr!("10013180", cursor);
        }
        if width != 0 {
            if mech_trace::gr!("10013180", "1008eb64", DAT_1008eb64) != 0 {
                let r = mech_trace::cr!(
                    "10013180",
                    "10012590",
                    FUN_10012590(
                        this,
                        x,
                        y,
                        width,
                        mech_trace::rd!(
                            "10013180",
                            ((font as u32).wrapping_add(8) as *const i32),
                            read_unaligned
                        ),
                    )
                );
                if r == 0 {
                    return mech_trace::rt!("10013180", width);
                }
            }
            if param_5.is_null() {
                param_5 = &raw mut DAT_100b0e38 as *mut u8;
            }
            let mut call_result: i32 = -1;
            if mech_trace::gr!("10013180", "10098204", DAT_10098204) != 0 {
                let addr = mech_trace::gr!("10013180", "100a7248", DAT_100a7248).wrapping_add(0x20)
                    as *const u32;
                let f: unsafe extern "C" fn() -> i32 = core::mem::transmute(mech_trace::rd!(
                    "10013180",
                    addr,
                    read_unaligned
                )
                    as usize);
                call_result = mech_trace::cr!("10013180", "ind", f());
            }
            if call_result == 0 {
                mech_trace::cr!(
                    "10013180",
                    "1006984a",
                    FUN_1006984a(
                        this.byte_add(0x56) as *mut i32,
                        x,
                        y,
                        font,
                        text,
                        param_5 as i32,
                    )
                );
            }
            mech_trace::cr!(
                "10013180",
                "100124d0",
                FUN_100124d0(
                    this,
                    x,
                    y,
                    width,
                    mech_trace::rd!(
                        "10013180",
                        ((font as u32).wrapping_add(8) as *const i32),
                        read_unaligned
                    ),
                )
            );
            return mech_trace::rt!("10013180", width);
        }
        mech_trace::rt!("10013180", 0)
    }
}
