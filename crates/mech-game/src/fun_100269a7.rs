// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_1016a6d0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10026e24(param_1: *mut u32) -> u8;
    fn FUN_1004d47f(param_1: i32) -> u32;
    fn FUN_1002b4cf(param_1: *mut u32, param_2: i16, param_3: i16, param_4: i16) -> u8;
    fn FUN_10008371(param_1: i32, param_2: i32) -> u8;
    fn FUN_10026c76(param_1: *mut i32) -> u8;
    fn FUN_10042289(param_1: *mut i32) -> u8;
}

#[inline(always)]
unsafe fn rd<T>(p: *mut i32, off: usize) -> T {
    // SAFETY: caller guarantees p points at a structure covering off.
    unsafe { p.byte_add(off).cast::<T>().read_unaligned() }
}

#[inline(always)]
unsafe fn wr<T>(p: *mut i32, off: usize, v: T) {
    // SAFETY: caller guarantees p points at a structure covering off.
    unsafe { p.byte_add(off).cast::<T>().write_unaligned(v) }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100269a7(param_1: *mut i32) {
    // SAFETY: param_1 is the game's record; globals and callees are the
    // game's own; all accesses mirror the original's offsets and order.
    unsafe {
        let base = (&raw mut DAT_1016a6d0).cast::<u8>() as i32;
        let p1: i32 = rd(param_1, 4);
        wr::<i32>(param_1, 0x144, base.wrapping_add(p1.wrapping_mul(0x18)));
        wr::<u16>(param_1, 0x176, 0);
        wr::<u32>(param_1, 0x1a2, 0x1000);
        wr::<u32>(param_1, 0x192, 0);
        FUN_10026e24(param_1.cast::<u32>());
        let tbl = (&raw mut DAT_100eef40).cast::<u32>();
        let ent: u32 = tbl.add(DAT_100adf38 as usize).read_unaligned();
        let cur: i32 = ((ent as usize + 8) as *const i32).read_unaligned();
        let id: i32 = rd(param_1, 8);
        if cur == id {
            wr::<u32>(param_1, 0x182, 0);
        } else {
            wr::<u32>(param_1, 0x182, 1);
        }
        let kind: i32 = rd(param_1, 0x10);
        if kind == 2 {
            let id: i32 = rd(param_1, 8);
            let u = FUN_1004d47f(id);
            FUN_1002b4cf(param_1.cast::<u32>(), 0, 0, u as i16);
            let ent: u32 = tbl.add(DAT_100adf38 as usize).read_unaligned();
            let i_var1 = ent as i32;
            let v: i32 = rd(param_1, 0x15c);
            wr::<u8>(param_1, 0x186, v as u8);
            if rd::<u8>(param_1, 0x186) == 0 {
                wr::<u8>(param_1, 0x186, 1);
            }
            let w: u16 = rd(param_1, 0x15e);
            wr::<u32>(param_1, 0x19e, u32::from(w));
            if rd::<i32>(param_1, 0x19e) == 0 {
                wr::<u32>(param_1, 0x19e, 0xfa);
            }
            let w: u16 = rd(param_1, 0x160);
            wr::<u32>(param_1, 0x196, u32::from(w));
            if rd::<i32>(param_1, 0x196) == 0 {
                wr::<u32>(param_1, 0x196, 0xfa);
            }
            let w: u16 = rd(param_1, 0x162);
            wr::<u32>(param_1, 0x19a, u32::from(w));
            if rd::<i32>(param_1, 0x19a) == 0 {
                wr::<u32>(param_1, 0x19a, 0xfa);
            }
            let v: i32 = rd(param_1, 0x164);
            wr::<u8>(param_1, 0x187, v as u8);
            if rd::<u8>(param_1, 0x187) == 0 {
                wr::<u8>(param_1, 0x187, 1);
            }
            let b: u8 = rd(param_1, 0x166);
            wr::<u8>(param_1, 0x188, b);
            if rd::<u8>(param_1, 0x188) == 0 {
                wr::<u8>(param_1, 0x188, 0);
            }
            let v: i32 = rd(param_1, 0x168);
            wr::<u8>(param_1, 0x189, v as u8);
            if rd::<u8>(param_1, 0x189) == 0 {
                wr::<u8>(param_1, 0x189, 0);
            }
            let w: u16 = rd(param_1, 0x16a);
            wr::<u32>(param_1, 0x18a, u32::from(w));
            let id: i32 = rd(param_1, 8);
            let other: i32 = ((i_var1 as usize + 8) as *const i32).read_unaligned();
            if id == other {
                let a: i32 = rd(param_1, 0xc);
                let b: i32 = rd(param_1, 0x18a);
                FUN_10008371(a, b);
            }
            FUN_10026c76(param_1);
            let b: u8 = rd(param_1, 0x186);
            wr::<u8>(param_1, 0x186, b.wrapping_add(1));
            let v: i32 = rd(param_1, 0x19e);
            wr::<i32>(param_1, 0x19e, v.wrapping_mul(100));
            let v: i32 = rd(param_1, 0x196);
            wr::<i32>(param_1, 0x196, v.wrapping_mul(100));
            let v: i32 = rd(param_1, 0x19a);
            wr::<i32>(param_1, 0x19a, v.wrapping_mul(100));
            FUN_10042289(param_1);
        }
    }
}
