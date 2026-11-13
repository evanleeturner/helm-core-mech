// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10098204: i32;
    static mut DAT_100a7248: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10067af3(
        x_likely: *mut i32,
        y_likely: i32,
        param_3: i32,
        param_4: *mut i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    );
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_100124d0(
        buffer_owner: *mut c_void,
        y_likely: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10012f10(
    buffer_owner_likely: *mut c_void,
    x_likely: i32,
    y_likely: i32,
    param_3: i32,
    param_4: i32,
) {
    mech_trace::enter!(
        "10012f10",
        buffer_owner_likely,
        x_likely,
        y_likely,
        param_3,
        param_4
    );
    // SAFETY: the globals and callees are the game's own; buffer_owner_likely
    // points at the caller's object, read at the offsets the original uses.
    unsafe {
        // The record the original builds on its stack (10012f30 to 10012f4b)
        // and hands `10067af3`, which reads all five words: the view at
        // `+0x42`, then the box's left, top, right and bottom.
        let mut source_view_and_box: [i32; 5] = [
            (buffer_owner_likely as i32).wrapping_add(0x42),
            x_likely,
            y_likely,
            x_likely.wrapping_add(param_3).wrapping_sub(1),
            y_likely.wrapping_add(param_4).wrapping_sub(1),
        ];
        let mut status: i32 = -1;
        if mech_trace::gr!("10012f10", "10098204", DAT_10098204) != 0 {
            let table = mech_trace::gr!("10012f10", "100a7248", DAT_100a7248);
            let f = mech_trace::rd!(
                "10012f10",
                table.byte_add(0x20).cast::<unsafe extern "C" fn() -> i32>(),
                read_unaligned
            );
            status = mech_trace::cr!("10012f10", "ind", f());
        }
        // The original copies only when the gate answers 0 (10012f5e:
        // `test eax,eax`, then `jne` past the call).
        if status == 0 {
            let flags = mech_trace::rd!(
                "10012f10",
                buffer_owner_likely.byte_add(0x3a6).cast::<u32>(),
                read_unaligned
            );
            FUN_10067af3(
                source_view_and_box.as_mut_ptr(),
                0,
                0,
                buffer_owner_likely.byte_add(0x56).cast::<i32>(),
                x_likely,
                y_likely,
                flags as i32,
            );
        }
        FUN_100124d0(buffer_owner_likely, x_likely, y_likely, param_3, param_4);
    }
}
