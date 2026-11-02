// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160fe8: u8;
    static mut DAT_10160fec: u8;
    static mut DAT_10160ff0: u8;
    static mut DAT_10160ff4: u8;
    static mut DAT_10160ff8: u8;
    static mut DAT_10161078: u8;
    static mut DAT_1016107c: u8;
    static mut DAT_10161088: u8;
    static mut DAT_1016108c: u8;
    static mut DAT_10161090: u8;
    static mut DAT_10161094: u8;
    static mut DAT_10161098: u8;
    static mut DAT_1016109c: u8;
    static mut DAT_101610a0: u8;
    static mut DAT_101610a4: u8;
    static mut DAT_101610a8: u8;
    static mut DAT_101610ac: u8;
    static mut DAT_101610b0: u8;
    static mut DAT_101610b4: u8;
    static mut DAT_101610b8: u8;
    static mut DAT_101610bc: u8;
    static mut DAT_101610c0: u8;
    static mut DAT_101610c4: u8;
    static mut DAT_101610c8: u8;
    static mut DAT_101610cc: u8;
    static mut DAT_101610d0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003a0e9(param_1: i32, param_2: i32) {
    mech_trace::enter!("1003a0e9", param_1, param_2);
    if !(-1 < param_2 && param_2 < 9) {
        return;
    }
    let off = param_2.wrapping_mul(0x80c) as usize;
    let src = param_1 as usize as *const u8;

    macro_rules! rd {
        ($o:expr) => {
            mech_trace::rd!("1003a0e9", src.byte_add($o).cast::<u32>(), read_unaligned)
        };
    }
    macro_rules! put {
        ($d:ident, $v:expr) => {
            mech_trace::wr!(
                "1003a0e9",
                (&raw mut $d).byte_add(off).cast::<u32>(),
                write_unaligned,
                $v
            )
        };
    }

    // SAFETY: the globals are the game's own tables, indexed by the checked
    // slot number; param_1 points at the caller's request block, read and
    // copied where the original does.
    unsafe {
        let u_var1 = rd!(4);
        if u_var1 & 1 == 0 {
            put!(DAT_10160fe8, 0);
        } else {
            put!(DAT_10160fe8, rd!(8));
        }
        if u_var1 & 2 != 0 {
            put!(DAT_10160fec, rd!(0xc));
            put!(DAT_10160ff0, rd!(0x10));
            put!(DAT_10160ff4, rd!(0x14));
        }
        if u_var1 & 4 != 0 {
            let mut s = src.byte_add(0x18).cast::<u32>();
            let mut d = (&raw mut DAT_10160ff8).byte_add(off).cast::<u32>();
            let mut n: i32 = 0x20;
            while n != 0 {
                mech_trace::wr!(
                    "1003a0e9",
                    d,
                    write_unaligned,
                    mech_trace::rd!("1003a0e9", s, read_unaligned)
                );
                s = s.add(1);
                d = d.add(1);
                n -= 1;
            }
        }
        if u_var1 & 8 == 0 {
            put!(DAT_10161078, 1);
        } else {
            put!(DAT_10161078, rd!(0x98));
        }
        if u_var1 & 0x10 == 0 {
            put!(DAT_1016107c, 1);
        } else {
            put!(DAT_1016107c, rd!(0x9c));
        }
        if u_var1 & 0x20 == 0 {
            put!(DAT_10161088, 0);
        } else {
            put!(DAT_10161088, rd!(0xa8));
        }
        if u_var1 & 0x40 != 0 {
            put!(DAT_1016108c, rd!(0xac));
            put!(DAT_10161090, rd!(0xb0));
            put!(DAT_10161094, rd!(0xb4));
        }
        if u_var1 & 0x80 != 0 {
            put!(DAT_10161098, rd!(0xb8));
        }
        if u_var1 & 0x100 != 0 {
            put!(DAT_1016109c, rd!(0xbc));
            put!(DAT_101610a0, rd!(0xc0));
            put!(DAT_101610a4, rd!(0xc4));
            put!(DAT_101610a8, rd!(200));
            put!(DAT_101610ac, rd!(0xcc));
            put!(DAT_101610b0, rd!(0xd0));
            put!(DAT_101610b4, rd!(0xd4));
        }
        if u_var1 & 0x200 != 0 {
            put!(DAT_101610b8, rd!(0xd8));
            put!(DAT_101610bc, rd!(0xdc));
            put!(DAT_101610c0, rd!(0xe0));
            put!(DAT_101610c4, rd!(0xe4));
            put!(DAT_101610c8, rd!(0xe8));
            put!(DAT_101610cc, rd!(0xec));
        }
        if u_var1 & 0x400 == 0 {
            put!(DAT_101610d0, 0);
        } else {
            put!(DAT_101610d0, rd!(0xf0));
        }
    }
}
