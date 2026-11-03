// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_1005f8c0(this: *mut u8, flag: u32) -> bool;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn FUN_1005f810(param_1: *mut u8) {
    mech_trace::enter!("1005f810", param_1);
    // SAFETY: the callee is the game's own; param_1 points at the game's
    // object, whose field at offset 0x10c is read and written as the original does.
    unsafe {
        if !mech_trace::cr!("1005f810", "1005f8c0", FUN_1005f8c0(param_1, 0x4000_0000)) {
            if mech_trace::cr!("1005f810", "1005f8c0", FUN_1005f8c0(param_1, 0x20)) {
                let p = param_1.byte_add(0x10c).cast::<u32>();
                mech_trace::wr!(
                    "1005f810",
                    p,
                    write_unaligned,
                    (mech_trace::rd!("1005f810", p, read_unaligned) & 0xffff_ffdf) | 0x100
                );
            }
        }
    }
}
