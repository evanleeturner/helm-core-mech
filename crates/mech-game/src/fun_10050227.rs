// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0864: u32;
    static mut DAT_100b0868: u32;
    static mut DAT_101155c0: u8;
    static mut DAT_101155c4: u8;
    static mut DAT_101155dc: u8;
    static mut DAT_101155e0: u8;
    static mut DAT_101155e4: u8;
    static mut DAT_101155e8: u8;
    static mut DAT_101155f0: u8;
    static mut DAT_101155f4: u8;
    static mut DAT_101155f8: u8;
    static mut DAT_10115606: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10050227() {
    mech_trace::enter!("10050227");
    let mut local_c: i32 = 0;
    while local_c < 0x30c {
        let i_var1: i32 = local_c.wrapping_mul(0x48);
        // SAFETY: the globals are the game's own table memory; every write
        // lands inside the 0x30c-entry, 0x48-byte-stride array as the original does.
        unsafe {
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155c0)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155c4)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155dc)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                0xffffffff
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155e4)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155e8)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155e0)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_10115606)
                    .byte_add(i_var1 as usize)
                    .cast::<u16>(),
                write_unaligned,
                0
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155f8)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                0
            );
            let v = mech_trace::rd!(
                "10050227",
                (&raw mut DAT_101155f8)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155f4)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                v
            );
            let v = mech_trace::rd!(
                "10050227",
                (&raw mut DAT_101155f4)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "10050227",
                (&raw mut DAT_101155f0)
                    .byte_add(i_var1 as usize)
                    .cast::<u32>(),
                write_unaligned,
                v
            );
            let mut local_10: i32 = 0;
            while local_10 < 5 {
                let a = i_var1
                    .wrapping_add(0x101155c8)
                    .wrapping_add(local_10.wrapping_mul(4));
                mech_trace::wr!("10050227", (a as *mut u32), write_unaligned, 0xffffffff);
                let b = i_var1
                    .wrapping_add(0x101155fc)
                    .wrapping_add(local_10.wrapping_mul(2));
                mech_trace::wr!("10050227", (b as *mut u16), write_unaligned, 0);
                local_10 = local_10.wrapping_add(1);
            }
        }
        local_c = local_c.wrapping_add(1);
    }
    // SAFETY: the globals are the game's own memory.
    unsafe {
        mech_trace::gw!("10050227", "100b0864", DAT_100b0864, 0);
        mech_trace::gw!("10050227", "100b0868", DAT_100b0868, 1);
    }
}
