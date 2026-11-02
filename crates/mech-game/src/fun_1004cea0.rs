// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1015f2e0: u8;
    static mut DAT_101600e0: u8;
    static mut DAT_101600e8: u8;
    static mut DAT_101600f8: u8;
    static mut DAT_101604c0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1004cea0() {
    mech_trace::enter!("1004cea0");
    let mut local_8: u32 = 0;
    while local_8 < 0x10 {
        // SAFETY: the globals are the game's own tables; the offsets stay
        // within the 0x10-entry arrays the original indexes.
        unsafe {
            mech_trace::wr!(
                "1004cea0",
                (&raw mut DAT_101600e0)
                    .byte_add(local_8.wrapping_mul(0x3c) as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "1004cea0",
                (&raw mut DAT_101600f8)
                    .byte_add(local_8.wrapping_mul(0x3c) as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "1004cea0",
                (&raw mut DAT_101600e8)
                    .byte_add(local_8.wrapping_mul(0x3c) as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            let mut src = (&raw mut DAT_1015f2e0).cast::<u32>() as *const u32;
            let mut dst = (&raw mut DAT_101604c0)
                .byte_add(local_8.wrapping_mul(0x70) as usize)
                .cast::<u32>();
            let mut count: i32 = 0x1c;
            while count != 0 {
                mech_trace::wr!(
                    "1004cea0",
                    dst,
                    write_unaligned,
                    mech_trace::rd!("1004cea0", src, read_unaligned)
                );
                src = src.add(1);
                dst = dst.add(1);
                count = count.wrapping_sub(1);
            }
        }
        local_8 = local_8.wrapping_add(1);
    }
}
