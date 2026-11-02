// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut LAB_1005b280: [u8; 1];
    static mut DAT_100c3368: u32;
    static mut DAT_100c336c: u32;
    static mut DAT_100c3370: u32;
    static mut DAT_100c3680: [i8; 1];
    static mut DAT_100d08dc: *mut c_void;
    static mut DAT_100d08e8: *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn DialogBoxParamA(
        h_instance: *mut c_void,
        template: *const c_void,
        parent: *mut c_void,
        proc_: *mut c_void,
        param: i32,
    ) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005b1b0(message_text: *mut i8) {
    // SAFETY: the globals and the dialog callee are the game's own; message_text
    // points at a NUL-terminated string, and the segment buffer and slots are
    // written exactly where the original writes them.
    unsafe {
        let mut msg = message_text;
        let mut buf = (&raw mut DAT_100c3680) as *mut i8;
        let mut count: i32 = 0;
        DAT_100c3370 = 0;
        DAT_100c336c = 0;
        DAT_100c3368 = 0;
        if *msg != 0 {
            let mut slot = (&raw mut DAT_100c3368) as *mut u32;
            loop {
                if *msg == b'#' as i8 {
                    break;
                }
                slot.write(buf as u32);
                slot = slot.add(1);
                count = count.wrapping_add(1);
                let mut c = *msg;
                let mut bar = false;
                while c != 0 {
                    c = *msg;
                    if c == b'|' as i8 {
                        bar = true;
                        break;
                    }
                    if c == b'#' as i8 {
                        break;
                    }
                    *buf = c;
                    msg = msg.add(1);
                    buf = buf.add(1);
                    c = *msg;
                }
                if bar || *msg == b'|' as i8 {
                    msg = msg.add(1);
                }
                *buf = 0;
                buf = buf.add(1);
                if *msg == 0 {
                    break;
                }
            }
        }
        let first = DAT_100c3368;
        if count == 1 {
            DAT_100c3368 = 0;
            DAT_100c336c = first;
        }
        count = 0;
        let mut c = *msg;
        while c != 0 {
            msg = msg.add(1);
            c = *msg;
            while c != 0 && *msg != b'|' as i8 {
                c = *msg.add(1);
                msg = msg.add(1);
            }
            count = count.wrapping_add(1);
            c = *msg;
        }
        let id = 0x81u32.wrapping_sub((count == 2) as u32);
        DialogBoxParamA(
            DAT_100d08dc,
            id as usize as *const c_void,
            DAT_100d08e8,
            (&raw mut LAB_1005b280) as *mut c_void,
            0,
        );
    }
}
