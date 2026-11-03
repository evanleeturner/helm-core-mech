// SPDX-License-Identifier: MIT

#[repr(C)]
struct tagPOINT {
    x: i32,
    y: i32,
}

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098204: u32;
    static mut DAT_100a6774: u32;
    static mut DAT_100d08d4: u32;
    static mut DAT_100d08d8: u32;
    static mut DAT_100d08e8: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10055b40(mouse_state: i32);
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn GetCursorPos(p: *mut core::ffi::c_void) -> i32;
    fn ScreenToClient(h: *mut core::ffi::c_void, p: *mut core::ffi::c_void) -> i32;
    fn GetAsyncKeyState(k: i32) -> i16;
    fn timeGetTime() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_10055bf0(mouse_state: i32) {
    mech_trace::enter!("10055bf0", mouse_state);
    let p = mouse_state as usize as *mut u8;
    // SAFETY: the globals and callees are the game's own; mouse_state points at
    // the game's mouse state block, read and written at the original's offsets.
    unsafe {
        if mech_trace::gr!("10055bf0", "10098204", DAT_10098204) != 0 {
            if mech_trace::gr!("10055bf0", "100a6774", DAT_100a6774) == 0 {
                FUN_10055b40(mouse_state);
            }
            let prev_left =
                mech_trace::rd!("10055bf0", p.byte_add(0x33).cast::<i32>(), read_unaligned);
            let prev_right =
                mech_trace::rd!("10055bf0", p.byte_add(0x37).cast::<i32>(), read_unaligned);
            let prev_middle =
                mech_trace::rd!("10055bf0", p.byte_add(0x3b).cast::<i32>(), read_unaligned);
            if mech_trace::rd!("10055bf0", p.byte_add(0x3f).cast::<i32>(), read_unaligned) != 0 {
                let mut pos = tagPOINT { x: 0, y: 0 };
                let got = mech_trace::cr!(
                    "10055bf0",
                    "GetCursorPos",
                    GetCursorPos((&raw mut pos).cast())
                );
                if got != 0 {
                    mech_trace::cr!(
                        "10055bf0",
                        "ScreenToClient",
                        ScreenToClient(
                            mech_trace::gr!("10055bf0", "100d08e8", DAT_100d08e8),
                            (&raw mut pos).cast()
                        )
                    );
                    let inside = !(pos.x < 0
                        || (mech_trace::gr!("10055bf0", "100d08d8", DAT_100d08d8) as i32) <= pos.x
                        || pos.y < 0
                        || (mech_trace::gr!("10055bf0", "100d08d4", DAT_100d08d4) as i32) <= pos.y);
                    if inside {
                        mech_trace::wr!(
                            "10055bf0",
                            p.byte_add(0x2b).cast::<i32>(),
                            write_unaligned,
                            pos.x
                        );
                        mech_trace::wr!(
                            "10055bf0",
                            p.byte_add(0x2f).cast::<i32>(),
                            write_unaligned,
                            pos.y
                        );
                        let b = mech_trace::cr!("10055bf0", "GetAsyncKeyState", GetAsyncKeyState(1))
                            as u16;
                        mech_trace::wr!(
                            "10055bf0",
                            p.byte_add(0x33).cast::<u32>(),
                            write_unaligned,
                            ((b & 0x8000) != 0) as u32
                        );
                        let b = mech_trace::cr!("10055bf0", "GetAsyncKeyState", GetAsyncKeyState(2))
                            as u16;
                        mech_trace::wr!(
                            "10055bf0",
                            p.byte_add(0x37).cast::<u32>(),
                            write_unaligned,
                            ((b & 0x8000) != 0) as u32
                        );
                        let b = mech_trace::cr!("10055bf0", "GetAsyncKeyState", GetAsyncKeyState(4))
                            as u16;
                        mech_trace::wr!(
                            "10055bf0",
                            p.byte_add(0x3b).cast::<u32>(),
                            write_unaligned,
                            ((b & 0x8000) != 0) as u32
                        );
                    }
                }
            }
            if mech_trace::rd!("10055bf0", p.byte_add(0x33).cast::<i32>(), read_unaligned) == 1
                && prev_left == 0
            {
                mech_trace::wr!(
                    "10055bf0",
                    p.byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    1
                );
            } else {
                mech_trace::wr!(
                    "10055bf0",
                    p.byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    0
                );
            }
            if mech_trace::rd!("10055bf0", p.byte_add(0x37).cast::<i32>(), read_unaligned) == 1
                && prev_right == 0
            {
                mech_trace::wr!(
                    "10055bf0",
                    p.byte_add(0x14).cast::<u32>(),
                    write_unaligned,
                    1
                );
            } else {
                mech_trace::wr!(
                    "10055bf0",
                    p.byte_add(0x14).cast::<u32>(),
                    write_unaligned,
                    0
                );
            }
            if mech_trace::rd!("10055bf0", p.byte_add(0x3b).cast::<i32>(), read_unaligned) == 1
                && prev_middle == 0
            {
                mech_trace::wr!(
                    "10055bf0",
                    p.byte_add(0x18).cast::<u32>(),
                    write_unaligned,
                    1
                );
            } else {
                mech_trace::wr!(
                    "10055bf0",
                    p.byte_add(0x18).cast::<u32>(),
                    write_unaligned,
                    0
                );
            }
            mech_trace::wr!("10055bf0", p.byte_add(0x1c), write, 0);
            if mech_trace::rd!("10055bf0", p.byte_add(0x33).cast::<i32>(), read_unaligned) == 1
                && prev_left == 0
            {
                let now = mech_trace::cr!("10055bf0", "timeGetTime", timeGetTime());
                let last =
                    mech_trace::rd!("10055bf0", p.byte_add(0x1f).cast::<u32>(), read_unaligned);
                if now.wrapping_sub(last) < 0xc9 {
                    mech_trace::wr!("10055bf0", p.byte_add(0x1c), write, 1);
                    return;
                }
                let now = mech_trace::cr!("10055bf0", "timeGetTime", timeGetTime());
                mech_trace::wr!(
                    "10055bf0",
                    p.byte_add(0x1f).cast::<u32>(),
                    write_unaligned,
                    now
                );
            }
        }
    }
}
