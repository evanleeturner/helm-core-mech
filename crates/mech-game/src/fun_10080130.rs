// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10160ef4: i32;
    static mut DAT_10160ef8: i32;
    static mut DAT_10160efc: i32;
    static mut DAT_10160f00: i32;
    static mut DAT_10160f04: i32;
    static mut DAT_10160f08: i32;
    static mut DAT_10160f44: i32;
    static mut DAT_10160f48: i32;
    static mut DAT_10160f4c: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10035f3b() -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10080130(param_1: i32) -> *mut i32 {
    let base = param_1 as usize as *mut u8;
    // SAFETY: param_1 points at the caller's structure; the globals and the
    // callee are the game's own; the returned block is written where the
    // original writes it.
    unsafe {
        let mut p = base.byte_add(0x24).cast::<*mut i32>().read_unaligned();
        if p.is_null() {
            p = FUN_10035f3b();
            let i_var6 = base
                .byte_add(0xc)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_sub(DAT_10160f48);
            let i_var7 = base
                .byte_add(0x10)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_sub(DAT_10160f44);
            let i_var8 = base
                .byte_add(0x14)
                .cast::<i32>()
                .read_unaligned()
                .wrapping_sub(DAT_10160f4c);
            let i_var1 = base.byte_add(0x18).cast::<i32>().read_unaligned();
            let i_var2 = base.byte_add(0x1c).cast::<i32>().read_unaligned();
            let l3: i64 = (DAT_10160ef4 as i64)
                .wrapping_mul(i_var6 as i64)
                .wrapping_add((DAT_10160ef8 as i64).wrapping_mul(i_var7 as i64))
                .wrapping_add((DAT_10160efc as i64).wrapping_mul(i_var8 as i64));
            let u9 = l3 as u32;
            let l4: i64 = (DAT_10160f00 as i64)
                .wrapping_mul(i_var6 as i64)
                .wrapping_add((DAT_10160f04 as i64).wrapping_mul(i_var7 as i64))
                .wrapping_add((DAT_10160f08 as i64).wrapping_mul(i_var8 as i64));
            let u10 = l4 as u32;
            base.byte_add(0x24).cast::<*mut i32>().write_unaligned(p);
            let i_var6b = base.byte_add(0x20).cast::<i32>().read_unaligned();
            let hi3 = ((l3 as u64) >> 32) as u32 as i32;
            let v0 = ((u9 >> 0x1b) | (hi3.wrapping_mul(0x20) as u32))
                .wrapping_add(((u9 >> 0x1a) & 1 != 0) as u32);
            p.write_unaligned(v0 as i32);
            let hi4 = ((l4 as u64) >> 32) as u32 as i32;
            let v1 = ((u10 >> 0x1b) | (hi4.wrapping_mul(0x20) as u32))
                .wrapping_add(((u10 >> 0x1a) & 1 != 0) as u32);
            p.add(1).write_unaligned(v1 as i32);
            p.add(2).write_unaligned(i_var6b);
            p.add(5).write_unaligned(i_var1.wrapping_shl(0x10));
            p.add(6).write_unaligned(i_var2.wrapping_shl(0x10));
            p.add(8)
                .write_unaligned(base.byte_add(0x38).cast::<i32>().read_unaligned());
            p.add(9)
                .write_unaligned(base.byte_add(0x3c).cast::<i32>().read_unaligned());
            p.add(10)
                .write_unaligned(base.byte_add(0x40).cast::<i32>().read_unaligned());
        }
        p
    }
}
