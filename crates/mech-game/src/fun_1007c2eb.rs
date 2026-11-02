// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab318: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100b6800: u8;
    static mut DAT_100cf0d0: u8;
    static mut DAT_100cf0d4: u8;
    static mut DAT_100cf0d8: u8;
    static mut DAT_100cf0e0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007c4d2(param_1: i32) -> u8;
    fn FUN_1007c28e(param_1: *mut i32, param_2: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007c2eb() {
    mech_trace::enter!("1007c2eb");
    // SAFETY: the globals and callees are the game's own; the arrays are
    // read and written at the offsets the original uses.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < 0x80 {
            mech_trace::cr!("1007c2eb", "1007c4d2", FUN_1007c4d2(local_8));
            local_8 = local_8.wrapping_add(1);
        }
        local_8 = 0;
        while local_8 < 0x80 {
            let off = local_8.wrapping_mul(0x14) as isize as usize;
            let a = (&raw mut DAT_100cf0d0).byte_add(off);
            let b = (&raw mut DAT_100cf0d4).byte_add(off);
            let c = (&raw mut DAT_100cf0d8).byte_add(off);
            let e = (&raw mut DAT_100cf0e0).byte_add(off);
            if mech_trace::rd!("1007c2eb", a.cast::<i32>(), read_unaligned) == 1
                && 0xe24
                    < (mech_trace::gr!("1007c2eb", "100b38fc", DAT_100b38fc) as i32)
                        .wrapping_sub(mech_trace::rd!("1007c2eb", c.cast::<i32>(), read_unaligned))
                && mech_trace::gr!("1007c2eb", "100ab318", DAT_100ab318) == 0
            {
                mech_trace::cr!(
                    "1007c2eb",
                    "1007c28e",
                    FUN_1007c28e(
                        mech_trace::rd!("1007c2eb", b.cast::<*mut i32>(), read_unaligned),
                        mech_trace::rd!("1007c2eb", e.cast::<*mut u8>(), read_unaligned),
                    )
                );
                let mut src = (&raw mut DAT_100b6800).cast::<u32>();
                let mut dst = a.cast::<u32>();
                let mut i = 5;
                while i != 0 {
                    mech_trace::wr!(
                        "1007c2eb",
                        dst,
                        write_unaligned,
                        mech_trace::rd!("1007c2eb", src, read_unaligned)
                    );
                    src = src.byte_add(4);
                    dst = dst.byte_add(4);
                    i -= 1;
                }
            }
            local_8 = local_8.wrapping_add(1);
        }
    }
}
