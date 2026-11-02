// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100add20: u8;
    static mut DAT_100ade20: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10036aeb(param_1: u32) {
    mech_trace::enter!("10036aeb", param_1);
    let mut param_1 = param_1;
    // SAFETY: the globals are the game's own tables; the original writes a
    // zero dword at the indexed slot of the selected table.
    unsafe {
        let p_var1: *mut u8;
        if (param_1 & 0x80) == 0 {
            p_var1 = &raw mut DAT_100ade20;
        } else {
            param_1 ^= 0x80;
            p_var1 = &raw mut DAT_100add20;
        }
        mech_trace::wr!(
            "10036aeb",
            p_var1
                .byte_add(param_1.wrapping_mul(4) as usize)
                .cast::<u32>(),
            write_unaligned,
            0
        );
    }
}
