// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ada50: i32;
    static mut DAT_10168914: u8;
    static mut DAT_10168918: u8;
    static mut DAT_1016891c: u8;
    static mut DAT_10168920: u8;
    static mut DAT_10168924: u8;
    static mut DAT_10168928: u8;
    static mut DAT_1016892c: u8;
    static mut DAT_10168930: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f37a(param_1: i32, param_2: u32, param_3: u32, param_4: u32) -> u8;
    fn FUN_1002fbc0(param_1: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003109e(param_1: i32, param_2: u8, param_3: *const u32) {
    // SAFETY: the globals and callees are the game's own; param_3 points at the
    // caller's record, and the table entries are read and written where the
    // original does.
    unsafe {
        if -1 < param_1 && param_1 < DAT_100ada50 {
            let i_var1 = param_1.wrapping_mul(0x34) as isize;
            let f14 = (&raw mut DAT_10168914).byte_offset(i_var1).cast::<u32>();
            let f18 = (&raw mut DAT_10168918).byte_offset(i_var1).cast::<u32>();
            let f1c = (&raw mut DAT_1016891c).byte_offset(i_var1).cast::<u32>();
            let f20 = (&raw mut DAT_10168920).byte_offset(i_var1).cast::<u32>();
            let f24 = (&raw mut DAT_10168924).byte_offset(i_var1).cast::<u32>();
            let f28 = (&raw mut DAT_10168928).byte_offset(i_var1).cast::<u32>();
            let f2c = (&raw mut DAT_1016892c).byte_offset(i_var1).cast::<u32>();
            let f30 = (&raw mut DAT_10168930)
                .byte_offset(i_var1)
                .cast::<*mut i32>();
            if (param_2 & 1) != 0 {
                f14.write_unaligned(param_3.read_unaligned());
                f18.write_unaligned(param_3.add(1).read_unaligned());
                f1c.write_unaligned(param_3.add(2).read_unaligned());
                let pi_var2 = f30.read_unaligned();
                if !pi_var2.is_null() {
                    let i_var3 = pi_var2.add(0x1b).read_unaligned();
                    if i_var3 != 0 {
                        (i_var3.wrapping_add(0x34) as *mut u32)
                            .write_unaligned(param_3.read_unaligned());
                        (i_var3.wrapping_add(0x38) as *mut u32)
                            .write_unaligned(param_3.add(1).read_unaligned());
                        (i_var3.wrapping_add(0x3c) as *mut u32)
                            .write_unaligned(param_3.add(2).read_unaligned());
                    }
                    FUN_1002f37a(
                        pi_var2 as i32,
                        f14.read_unaligned(),
                        f18.read_unaligned(),
                        f1c.read_unaligned(),
                    );
                    FUN_1002fbc0(pi_var2);
                }
            }
            if (param_2 & 2) != 0 {
                f20.write_unaligned(param_3.add(3).read_unaligned());
                f28.write_unaligned(0);
            }
            if (param_2 & 4) != 0 {
                f24.write_unaligned(param_3.add(4).read_unaligned());
                f2c.write_unaligned(1);
            }
        }
    }
}
