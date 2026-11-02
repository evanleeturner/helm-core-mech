// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160f60: [u32; 32];
    static mut DAT_10160fe0: u8;
    static mut DAT_10160fec: u8;
    static mut DAT_10160ff0: u8;
    static mut DAT_10160ff4: u8;
    static mut DAT_10160ff8: u8;
    static mut DAT_101610c4: u8;
    static mut DAT_101610c8: u8;
    static mut DAT_101610cc: u8;
    static mut DAT_101610d4: u8;
    static mut DAT_101610d8: u8;
    static mut DAT_10161730: u8;
    static mut DAT_10161734: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10039a79() {
    const TABLE: [u32; 16] = [
        0xe8, 0xe9, 0xea, 0xeb, 0xec, 0xed, 0xee, 0xef, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd,
        0xfe,
    ];
    // SAFETY: all globals are the game's own memory; the offsets stay within
    // the game's nine 0x80c-byte records, exactly as the original indexes them.
    unsafe {
        let arr = (&raw mut DAT_10160f60).cast::<u32>();
        let mut k: usize = 0;
        while k < 16 {
            arr.add(31 - 2 * k).write(TABLE[k]);
            arr.add(30 - 2 * k).write(TABLE[k]);
            k += 1;
        }

        let mut local_14: i32 = 0;
        while local_14 < 9 {
            let i_var2 = local_14.wrapping_mul(0x80c) as usize;
            let p_var1 = (&raw mut DAT_10160fe0).byte_add(i_var2);
            (&raw mut DAT_10161734)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_10161730)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(0x2d);
            let (local_10, local_18): (u32, u32) = match local_14 {
                0 => (0xffffc000, 0xffffc000),
                1 => (0, 0xffffc000),
                2 => (0x4000, 0xffffc000),
                3 => (0xffffc000, 0),
                4 => (0, 0),
                5 => (0x4000, 0),
                6 => (0xffffc000, 0x4000),
                7 => (0, 0x4000),
                _ => (0x4000, 0x4000),
            };
            (&raw mut DAT_10160fec)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(local_10);
            (&raw mut DAT_10160ff0)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(10000);
            (&raw mut DAT_10160ff4)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(local_18);
            (&raw mut DAT_101610c4)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(20000);
            (&raw mut DAT_101610c8)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_101610cc)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(20000);
            (&raw mut DAT_101610d4)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(0);
            (&raw mut DAT_101610d8)
                .byte_add(i_var2)
                .cast::<u32>()
                .write_unaligned(0);

            let mut local_1c: usize = 0;
            while local_1c < 0x2d {
                (&raw mut DAT_10161734)
                    .byte_add(4 + i_var2 + local_1c * 4)
                    .cast::<i32>()
                    .write_unaligned(local_1c as i32);
                p_var1
                    .byte_add(local_1c * 0x24 + 0x100)
                    .cast::<u32>()
                    .write_unaligned(0xffffffff);
                p_var1
                    .byte_add(local_1c * 0x24 + 0xfc)
                    .cast::<i32>()
                    .write_unaligned(local_1c as i32);
                p_var1
                    .byte_add(local_1c * 0x24 + 0x11c)
                    .cast::<*mut u8>()
                    .write_unaligned(p_var1);
                local_1c += 1;
            }

            let mut puvar4 = (&raw mut DAT_10160f60).cast::<u32>();
            let mut puvar5 = (&raw mut DAT_10160ff8).byte_add(i_var2).cast::<u32>();
            let mut i_var3: i32 = 0x20;
            while i_var3 != 0 {
                puvar5.write_unaligned(puvar4.read());
                puvar4 = puvar4.add(1);
                puvar5 = puvar5.add(1);
                i_var3 -= 1;
            }
            local_14 += 1;
        }
    }
}
