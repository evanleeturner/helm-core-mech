// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0714: u32;
    static mut DAT_1015f2e0: [u8; 0];
    static mut DAT_1015f2f0: [u8; 0];
    static mut DAT_1015f310: [u8; 0];
    static mut DAT_1015f330: [u8; 0];
}

unsafe extern "C" {
    fn strncpy(dest: *mut i8, source: *const i8, count: usize) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100323e1(param_1: i32) -> u32 {
    mech_trace::enter!("100323e1", param_1);
    // SAFETY: the globals are the game's own memory; param_1 is the caller's
    // record address, read and written at the same offsets as the original.
    unsafe {
        let i_var1 = mech_trace::gr!("100323e1", "100b0714", DAT_100b0714);
        let base = param_1 as usize as *const u8;
        let u_var2 = (mech_trace::rd!("100323e1", base.byte_add(4).cast::<i32>(), read_unaligned)
            as u32)
            .wrapping_sub(0x18)
            / 0xc;
        mech_trace::rt!(
            "100323e1",
            if mech_trace::gr!("100323e1", "100b0714", DAT_100b0714) < 0x20 && u_var2 < 9 {
                mech_trace::gw!(
                    "100323e1",
                    "100b0714",
                    DAT_100b0714,
                    mech_trace::gr!("100323e1", "100b0714", DAT_100b0714).wrapping_add(1)
                );
                let idx = i_var1 as i32;
                mech_trace::cr!(
                    "100323e1",
                    "strncpy",
                    strncpy(
                        (&raw mut DAT_1015f2e0 as *mut u8)
                            .byte_add(idx.wrapping_mul(0x1c) as usize)
                            .cast::<i8>(),
                        base.byte_add(8).cast::<i8>(),
                        0x10,
                    )
                );
                let mut local_10: i32 = 0;
                while local_10 < u_var2 as i32 {
                    let off = local_10
                        .wrapping_mul(4)
                        .wrapping_add(idx.wrapping_mul(0x70))
                        as usize;
                    let src = local_10.wrapping_mul(0xc) as usize;
                    mech_trace::wr!(
                        "100323e1",
                        (&raw mut DAT_1015f2f0 as *mut u8)
                            .byte_add(off)
                            .cast::<u32>(),
                        write_unaligned,
                        mech_trace::rd!(
                            "100323e1",
                            base.byte_add(0x18).byte_add(src).cast::<u32>(),
                            read_unaligned
                        )
                    );
                    mech_trace::wr!(
                        "100323e1",
                        (&raw mut DAT_1015f310 as *mut u8)
                            .byte_add(off)
                            .cast::<u32>(),
                        write_unaligned,
                        mech_trace::rd!(
                            "100323e1",
                            base.byte_add(0x1c).byte_add(src).cast::<u32>(),
                            read_unaligned
                        )
                    );
                    mech_trace::wr!(
                        "100323e1",
                        (&raw mut DAT_1015f330 as *mut u8)
                            .byte_add(off)
                            .cast::<u32>(),
                        write_unaligned,
                        mech_trace::rd!(
                            "100323e1",
                            base.byte_add(0x20).byte_add(src).cast::<u32>(),
                            read_unaligned
                        )
                    );
                    local_10 = local_10.wrapping_add(1);
                }
                1
            } else {
                0
            }
        )
    }
}
