// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7558: u32;
    static mut DAT_100aae80: u32;
    static mut DAT_100aafbc: u32;
    static mut DAT_100b0a08: u32;
    static mut DAT_100b352c: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003eb9c(param_1: *mut i32, param_2: i32, param_3: *mut u32) -> u8;
    fn FUN_1007b068(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: i32,
        param_4: u32,
        param_5: u32,
        param_6: u32,
        param_7: u32,
        param_8: u32,
    ) -> u8;
}

#[repr(C)]
struct Frame {
    ptrs: *mut *mut u8,
    count: i32,
    count2: i32,
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000c3a0(
    param_1: *mut i32,
    param_2: *mut u8,
    param_3: i16,
    param_4: i16,
    param_5: i32,
    param_6: *mut u32,
    _param_7: u32,
    _param_8: u32,
    param_9: i32,
) {
    mech_trace::enter!(
        "1000c3a0", param_1, param_2, param_3, param_4, param_5, param_6, _param_7, _param_8,
        param_9
    );
    let mut param_2 = param_2;
    let mut param_4 = param_4;
    let mut local_210: [*mut u8; 128] = [core::ptr::null_mut(); 128];
    let local_8: u32;

    // SAFETY: the globals are the game's own memory.
    let local_214: u32 = unsafe {
        if mech_trace::gr!("1000c3a0", "100aafbc", DAT_100aafbc) == 0 {
            0x651
        } else {
            0x451
        }
    };
    if 0x80 < param_4 {
        param_4 = 0x80;
    }
    if param_3 < param_4 {
        param_4 = param_3;
    }
    let mut local_10: i32 = 0;
    while local_10 < param_4 as i32 {
        local_210[local_10 as usize] = param_2;
        param_2 = param_2.wrapping_offset(param_3 as isize);
        local_10 = local_10.wrapping_add(1);
    }
    let mut frame = Frame {
        ptrs: local_210.as_mut_ptr(),
        count: param_4 as i32,
        count2: param_4 as i32,
    };
    // SAFETY: globals and callees are the game's own; param_6 and param_2's
    // rows point at the caller's memory, accessed as the original does.
    unsafe {
        if param_9 == 0 || mech_trace::gr!("1000c3a0", "100a7558", DAT_100a7558) == 0 {
            local_8 = mech_trace::rd!("1000c3a0", param_6.byte_add(8), read_unaligned);
            mech_trace::cr!(
                "1000c3a0",
                "1007b068",
                FUN_1007b068(
                    param_1,
                    param_6 as *mut i32,
                    param_5,
                    local_214,
                    local_8,
                    (&raw mut frame) as usize as u32,
                    mech_trace::gr!("1000c3a0", "100b352c", DAT_100b352c),
                    0,
                )
            );
        } else {
            let l8: u32;
            if mech_trace::gr!("1000c3a0", "100b0a08", DAT_100b0a08) == 0 {
                if mech_trace::gr!("1000c3a0", "100aae80", DAT_100aae80) == 0 {
                    l8 = (mech_trace::rd!("1000c3a0", local_210[0], read) as u32) & 0xf0;
                } else {
                    l8 = ((mech_trace::rd!("1000c3a0", local_210[0], read) as u32) & 0xf0) | 0xf;
                }
            } else {
                l8 = 3;
            }
            mech_trace::wr!(
                "1000c3a0",
                param_6.byte_add(8),
                write_unaligned,
                l8.wrapping_shl(0x10)
            );
            mech_trace::cr!(
                "1000c3a0",
                "1003eb9c",
                FUN_1003eb9c(param_1, param_5, param_6)
            );
        }
    }
}
