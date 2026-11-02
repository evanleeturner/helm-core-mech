// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7bc8: u8;
    static mut DAT_100a7bcc: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10062d5f(param_1: i32, param_2: i32) -> u8;
    fn FUN_10044cc4(param_1: i32, param_2: *mut i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100636e5(param_1: *mut i32) {
    // SAFETY: param_1 points at the game's object; the offsets, globals and
    // callees are the game's own memory, accessed in the original's order.
    unsafe {
        let i_var2 = param_1.add(0xf8).read_unaligned();
        FUN_10062d5f(param_1 as i32, 0);
        let i_var3 = param_1.add(0xf8).read_unaligned();
        loop {
            let i_var4 = param_1
                .add(0xf8)
                .read_unaligned()
                .wrapping_mul(0x40)
                .wrapping_add(param_1.add(0x15).read_unaligned());
            let p = i_var4 as usize as *mut u8;
            let i_var1 = p
                .byte_add(4)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_mul(0x68);
            if p.byte_add(8).cast::<i32>().read_unaligned() == 1 {
                p.byte_add(8).cast::<i32>().write_unaligned(2);
                let a = (&raw mut DAT_100a7bcc).byte_offset(i_var1 as isize);
                p.byte_add(0x24)
                    .cast::<u32>()
                    .write_unaligned(a.cast::<u32>().read_unaligned());
                p.byte_add(0x10).cast::<u32>().write_unaligned(0);
                FUN_10044cc4(i_var4, param_1);
                let b = (&raw mut DAT_100a7bc8).byte_offset(i_var1 as isize);
                if b.cast::<i32>().read_unaligned() == 0
                    || (param_1.add(0x110).cast::<u16>().read_unaligned() & 0x80) == 0
                {
                    p.byte_add(0x1c).cast::<u32>().write_unaligned(0);
                    let v = p.byte_add(0x1c).cast::<u32>().read_unaligned();
                    p.byte_add(0x18).cast::<u32>().write_unaligned(v);
                } else {
                    let base = param_1.read_unaligned() as usize as *const u8;
                    let v = base.byte_add(0xdc).cast::<u32>().read_unaligned();
                    p.byte_add(0x18).cast::<u32>().write_unaligned(v & 0xff);
                    let v = base.byte_add(0xdc).cast::<u32>().read_unaligned();
                    p.byte_add(0x1c).cast::<u32>().write_unaligned(v & 0xf00);
                }
            }
            FUN_10062d5f(param_1 as i32, 0);
            if param_1.add(0xf8).read_unaligned() == i_var3 {
                break;
            }
        }
        param_1.add(0xf8).write_unaligned(i_var2);
    }
}
