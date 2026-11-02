// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b6800: u8;
    static mut DAT_100cf0d0: u8;
    static mut DAT_100cf0d4: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007c28e(param_1: *mut i32, param_2: *mut u8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007cb42(param_1: *mut i32, param_2: *mut u8) {
    mech_trace::enter!("1007cb42", param_1, param_2);
    if param_1.is_null() {
        return;
    }
    // SAFETY: the globals and the callee are the game's own memory; the table
    // at DAT_100cf0d0/DAT_100cf0d4 holds 0x80 entries of 0x14 bytes, and the
    // source block at DAT_100b6800 is 5 dwords, accessed as the original does.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 <= 0x7f {
            let off = (local_8 as usize).wrapping_mul(0x14);
            let entry = mech_trace::rd!(
                "1007cb42",
                (&raw mut DAT_100cf0d4).byte_add(off).cast::<*mut i32>(),
                read_unaligned
            );
            if entry == param_1 {
                let mut src = (&raw mut DAT_100b6800).cast::<u32>();
                let mut dst = (&raw mut DAT_100cf0d0).byte_add(off).cast::<u32>();
                let mut count = 5;
                while count != 0 {
                    mech_trace::wr!(
                        "1007cb42",
                        dst,
                        write_unaligned,
                        mech_trace::rd!("1007cb42", src, read_unaligned)
                    );
                    src = src.add(1);
                    dst = dst.add(1);
                    count -= 1;
                }
                break;
            }
            local_8 += 1;
        }
        mech_trace::cr!("1007cb42", "1007c28e", FUN_1007c28e(param_1, param_2));
    }
}
