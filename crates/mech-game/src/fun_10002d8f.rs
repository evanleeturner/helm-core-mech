// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a40f8: u32;
    static mut DAT_100a40fc: [i8; 4];
    static mut DAT_100adf74: u32;
    static mut DAT_101e8db0: [u8; 1];
    static mut DAT_101e8db4: [u8; 1];
    static mut PTR_DAT_100af0a8: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1008668b(param_1: i32, mem: u32, param_3: *mut i32) -> *mut u16;
    fn FUN_1008664f(param_1: u32, mem: *mut i32) -> u8;
    fn _strncmp(a: *mut i8, b: *mut i8, n: u32) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn _AIL_end_sample_4(a: u32) -> u32;
    fn HeapFree(h: u32, flags: u32, mem: *mut c_void) -> i32;
    fn _AIL_init_sample_4(a: u32) -> u32;
    fn _AIL_set_sample_user_data_12(a: u32, b: u32, c: u32) -> u32;
    fn _AIL_register_EOS_callback_8(a: u32, b: u32) -> u32;
    fn FUN_100033ce(param_1: u32);
}

unsafe fn at(off: u32) -> *mut u8 {
    // SAFETY: the base is the game's own table pointer.
    unsafe { DAT_100a40f8.wrapping_add(off) as *mut u8 }
}
unsafe fn rd32(off: u32) -> u32 {
    // SAFETY: offset into the game's table.
    unsafe { at(off).cast::<u32>().read_unaligned() }
}
unsafe fn wr32(off: u32, v: u32) {
    // SAFETY: offset into the game's table.
    unsafe { at(off).cast::<u32>().write_unaligned(v) }
}
unsafe fn rd16(off: u32) -> u16 {
    // SAFETY: offset into the game's table.
    unsafe { at(off).cast::<u16>().read_unaligned() }
}
unsafe fn wr16(off: u32, v: u16) {
    // SAFETY: offset into the game's table.
    unsafe { at(off).cast::<u16>().write_unaligned(v) }
}
unsafe fn rd8(off: u32) -> u8 {
    // SAFETY: offset into the game's table.
    unsafe { at(off).read() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10002d8f(
    mut param_1: u32,
    mem: *mut u8,
    param_3: u16,
    param_4: i16,
    mut param_5: i32,
) -> u32 {
    // SAFETY: all globals, tables and callees belong to the game; offsets
    // are applied exactly as the original does.
    unsafe {
        if !mem.is_null() {
            param_1 = 0xffff_ffff;
        }
        let a4 = (&raw mut DAT_101e8db4) as *mut u8 as u32;
        let a0 = (&raw mut DAT_101e8db0) as *mut u8 as u32;
        if (param_1 as i32) < 1 {
            if mem.is_null() {
                return 0xffff_fffa;
            }
        } else {
            let p4 = a4.wrapping_add(param_1.wrapping_mul(8)).wrapping_add(2) as *const i16;
            let p0 = a0.wrapping_add(param_1.wrapping_mul(8)).wrapping_add(2) as *const i16;
            if p4.read_unaligned() != -1
                && p0.read_unaligned() != -1
                && p0.read_unaligned() <= p4.read_unaligned()
            {
                return 0xffff_fff6;
            }
        }
        let mut local_c: u32 = 0xffff_ffff;
        let mut local_14: u32 = 0xffff_ffff;
        if param_4 != -1 {
            let idx = param_4 as i32 as u32;
            if rd32(idx.wrapping_mul(4).wrapping_add(0x48)) != 0 {
                return 0xffff_fff8;
            }
            local_14 = idx;
        }
        if local_14 == 0xffff_ffff {
            let mut local_10: u32 = 0;
            while local_10 < rd32(0x44) && local_14 == 0xffff_ffff {
                let o2 = local_10.wrapping_mul(2);
                let o4 = local_10.wrapping_mul(4);
                let mut lab = true;
                if rd8(0x4e9u32.wrapping_add(o2)) != 0 {
                    if (rd16(0x4e8u32.wrapping_add(o2)) & 0x100) == 0 {
                        if (rd16(0x4e8u32.wrapping_add(o2)) & 0x400) != 0
                            && (param_3 & 0x400) != 0
                            && rd32(0x4a8u32.wrapping_add(o4)) == param_1
                        {
                            return 0xffff_fff6;
                        }
                        if (rd16(0x4e8u32.wrapping_add(o2)) & 0x200) != 0
                            && (param_3 & 0x200) != 0
                            && rd32(0x4a8u32.wrapping_add(o4)) == param_1
                        {
                            _AIL_end_sample_4(rd32(4u32.wrapping_add(o4)));
                            wr32(0x48u32.wrapping_add(o4), 0);
                            wr16(0x4e8u32.wrapping_add(o2), 0);
                        }
                    } else {
                        if (param_3 & 0x100) != 0 {
                            return 0xffff_fff7;
                        }
                        lab = false;
                    }
                }
                if lab {
                    if rd32(0x48u32.wrapping_add(o4)) == 0 {
                        local_14 = local_10;
                    } else if (rd8(0x4e8u32.wrapping_add(o2)) as i8 as i32) < (param_3 as i32) {
                        local_c = local_10;
                    }
                }
                local_10 = local_10.wrapping_add(1);
            }
            if local_14 == 0xffff_ffff {
                if local_c == 0xffff_ffff {
                    return 0xffff_fff8;
                }
                local_14 = local_c;
                _AIL_end_sample_4(rd32(4u32.wrapping_add(local_c.wrapping_mul(4))));
                wr32(0x48u32.wrapping_add(local_c.wrapping_mul(4)), 0);
            }
        }
        let l4 = local_14.wrapping_mul(4);
        if mem.is_null() {
            let p = FUN_1008668b(0, param_1, PTR_DAT_100af0a8 as *mut i32);
            wr32(0x368u32.wrapping_add(l4), p as u32);
            if rd32(0x368u32.wrapping_add(l4)) == 0 {
                return 0xffff_fffa;
            }
        } else {
            wr32(0x368u32.wrapping_add(l4), mem as u32);
        }
        let p3 = rd32(0x368u32.wrapping_add(l4)) as *const u8;
        let l14 = local_14.wrapping_mul(0xe);
        wr32(l14.wrapping_add(0x88), p3.cast::<u32>().read_unaligned());
        wr32(
            l14.wrapping_add(0x8c),
            p3.byte_add(4).cast::<u32>().read_unaligned(),
        );
        wr32(
            l14.wrapping_add(0x90),
            p3.byte_add(8).cast::<u32>().read_unaligned(),
        );
        wr16(
            l14.wrapping_add(0x94),
            p3.byte_add(12).cast::<u16>().read_unaligned(),
        );
        wr32(
            0x368u32.wrapping_add(l4),
            rd32(0x368u32.wrapping_add(l4)).wrapping_add(0xe),
        );
        let r = _strncmp(
            at(l14.wrapping_add(0x88)) as *mut i8,
            (&raw mut DAT_100a40fc) as *mut i8,
            4,
        );
        if r == 0 {
            wr16(0x4e8u32.wrapping_add(local_14.wrapping_mul(2)), param_3);
            wr32(0x4a8u32.wrapping_add(l4), param_1);
            wr32(0x168u32.wrapping_add(l4), rd32(0x90u32.wrapping_add(l14)));
            wr32(
                0x1a8u32.wrapping_add(l4),
                rd16(0x94u32.wrapping_add(l14)) as u32,
            );
            let divisor = rd32(0x1a8u32.wrapping_add(l4));
            let q: u32;
            core::arch::asm!(
                "div {d:e}",
                d = in(reg) divisor,
                inout("eax") 0x2000u32 => q,
                inout("edx") 0u32 => _,
                options(nomem, nostack)
            );
            wr32(0x1e8u32.wrapping_add(l4), q);
            wr32(
                0x268u32.wrapping_add(l4),
                rd32(0x1e8u32.wrapping_add(l4)).wrapping_mul(rd32(0x1a8u32.wrapping_add(l4))),
            );
            let l8 = local_14.wrapping_mul(8);
            wr32(
                0x2acu32.wrapping_add(l8),
                rd32(0x2a8u32.wrapping_add(l8)).wrapping_add(rd32(0x268u32.wrapping_add(l4))),
            );
            wr32(0x228u32.wrapping_add(l4), rd32(0x1e8u32.wrapping_add(l4)));
            if rd32(0x168u32.wrapping_add(l4)) < rd32(0x228u32.wrapping_add(l4)) {
                wr32(0x228u32.wrapping_add(l4), rd32(0x168u32.wrapping_add(l4)));
            }
            wr32(0x328u32.wrapping_add(l4), 0);
            let u4 = rd32(4u32.wrapping_add(l4));
            _AIL_init_sample_4(u4);
            if param_5 == -1 {
                param_5 = l4.wrapping_add(DAT_100a40f8).wrapping_add(0x48) as i32;
            }
            if param_5 != 0 {
                _AIL_set_sample_user_data_12(u4, 0, param_5 as u32);
            }
            _AIL_set_sample_user_data_12(u4, 1, param_1);
            _AIL_set_sample_user_data_12(u4, 2, local_14);
            _AIL_set_sample_user_data_12(u4, 3, 0);
            _AIL_set_sample_user_data_12(u4, 4, mem as u32);
            _AIL_register_EOS_callback_8(u4, FUN_100033ce as usize as u32);
            if (param_1 as i32) > 0 {
                let ps = a4.wrapping_add(param_1.wrapping_mul(8)).wrapping_add(2) as *mut i16;
                if ps.read_unaligned() != -1 {
                    ps.write_unaligned(ps.read_unaligned().wrapping_add(1));
                }
            }
        } else {
            if mem.is_null() {
                FUN_1008664f(param_1, PTR_DAT_100af0a8 as *mut i32);
            } else {
                HeapFree(DAT_100adf74, 1, mem as *mut c_void);
            }
            wr32(0x368u32.wrapping_add(l4), 0);
            local_14 = 0xffff_fff9;
        }
        local_14
    }
}
