// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7ba8: [u8; 0];
    static mut DAT_100ab30c: u32;
    static mut DAT_100adf38: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10062e97(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's object; the addresses computed from
    // its fields and the globals are the game's own memory, accessed as the
    // original does.
    unsafe {
        let mut b_var2 = false;
        let mut local_8: i32 = 0;
        if 0 < param_1.add(0xf7).read_unaligned() {
            while !b_var2 {
                let v = param_1.add(0xf8).read_unaligned();
                param_1.add(0xf8).write_unaligned(v.wrapping_add(1));
                let v = param_1.add(0xf8).read_unaligned();
                param_1.add(0xf8).write_unaligned(v % 0xe);
                let a = param_1
                    .add(0xf8)
                    .read_unaligned()
                    .wrapping_mul(0x40)
                    .wrapping_add(4)
                    .wrapping_add(param_1.add(0x15).read_unaligned());
                let i_var1 = (a as *const i32).read_unaligned();
                if -1 < i_var1 {
                    if DAT_100ab30c != 0 {
                        let base = param_1.read_unaligned();
                        let w = (base.wrapping_add(4) as *const u32).read_unaligned();
                        if w == DAT_100adf38 {
                            return;
                        }
                    }
                    let a = param_1
                        .add(0xf8)
                        .read_unaligned()
                        .wrapping_mul(0x40)
                        .wrapping_add(param_1.add(0x15).read_unaligned());
                    if (a as *const i32).read_unaligned() == -1 {
                        param_1.add(0xf8).write_unaligned(0);
                    }
                    let a = param_1
                        .add(0xf8)
                        .read_unaligned()
                        .wrapping_mul(0x40)
                        .wrapping_add(8)
                        .wrapping_add(param_1.add(0x15).read_unaligned());
                    let mut cond = false;
                    if (a as *const i32).read_unaligned() != -1 {
                        let p = (&raw mut DAT_100a7ba8)
                            .cast::<u8>()
                            .byte_offset(i_var1.wrapping_mul(0x68) as isize);
                        if (p.read() & 4) != 0 {
                            cond = true;
                        }
                    }
                    if !cond {
                        local_8 = local_8.wrapping_add(1);
                        if local_8 == 0xe {
                            cond = true;
                        }
                    }
                    if cond {
                        b_var2 = true;
                    }
                }
            }
        }
    }
}
