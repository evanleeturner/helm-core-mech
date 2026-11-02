// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: u32;
    static mut DAT_100c6d30: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e369(param_1: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003dce5(param_1: *mut u8) {
    mech_trace::enter!("1003dce5", param_1);
    // SAFETY: the globals and the callee are the game's own; param_1 points at
    // a game node whose fields at 0xc and 0x10 are read and written as the
    // original does.
    unsafe {
        if !param_1.is_null()
            && mech_trace::gr!("1003dce5", "100af098", DAT_100af098) != param_1 as u32
            && param_1 != (&raw mut DAT_100c6d30)
        {
            mech_trace::cr!("1003dce5", "1003e369", FUN_1003e369(param_1 as i32));
            let p10 = mech_trace::rd!(
                "1003dce5",
                param_1.byte_add(0x10).cast::<i32>(),
                read_unaligned
            );
            if p10 != 0 {
                let v = mech_trace::rd!(
                    "1003dce5",
                    param_1.byte_add(0xc).cast::<u32>(),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1003dce5",
                    (p10 as u32 as *mut u8).byte_add(0xc).cast::<u32>(),
                    write_unaligned,
                    v
                );
            }
            let p0c = mech_trace::rd!(
                "1003dce5",
                param_1.byte_add(0xc).cast::<i32>(),
                read_unaligned
            );
            if p0c != 0 {
                let v = mech_trace::rd!(
                    "1003dce5",
                    param_1.byte_add(0x10).cast::<u32>(),
                    read_unaligned
                );
                mech_trace::wr!(
                    "1003dce5",
                    (p0c as u32 as *mut u8).byte_add(0x10).cast::<u32>(),
                    write_unaligned,
                    v
                );
            }
            mech_trace::wr!(
                "1003dce5",
                param_1.byte_add(0xc).cast::<u32>(),
                write_unaligned,
                0
            );
            let v = mech_trace::rd!(
                "1003dce5",
                param_1.byte_add(0xc).cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "1003dce5",
                param_1.byte_add(0x10).cast::<u32>(),
                write_unaligned,
                v
            );
        }
    }
}
