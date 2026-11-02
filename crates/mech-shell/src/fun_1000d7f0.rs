// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d7f0(param_1: i32) {
    let mut p = (param_1 as u32).wrapping_add(0x760) as *mut u8;
    let mut count: i32 = 8;
    // SAFETY: param_1 addresses the game's own structure; every access is at
    // the offsets the original uses, within its 8 entries of 0x12 bytes.
    unsafe {
        loop {
            let u0 = p.byte_add(16).cast::<u16>().read_unaligned();
            p.byte_add(16).cast::<u16>().write_unaligned(u0 & 0xff00);

            let base = p.cast::<i32>().read_unaligned();

            let mut v = p
                .byte_add(8)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_add(base)
                / 5;
            if v < 1 {
                v = 0;
            } else if 0xf < v {
                v = 0xf;
            }
            let mut u1: u16 = ((v as u16) & 0xf) | (u0 & 0xff00);
            p.byte_add(16).cast::<u16>().write_unaligned(u1);

            let mut w = p
                .byte_add(12)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_add(base)
                / 5;
            if w < 1 {
                w = 0;
            } else if 0xf < w {
                w = 0xf;
            }
            u1 |= ((w as u16) & 0xf).wrapping_shl(4);
            p.byte_add(16).cast::<u16>().write_unaligned(u1);
            p.byte_add(16).cast::<u16>().write_unaligned(u1 | 0x800);

            count = count.wrapping_sub(1);
            p = p.byte_add(0x12);
            if count == 0 {
                break;
            }
        }
    }
}
