// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7850: i32;
    static mut DAT_100a6740: *mut u8;
    static mut DAT_100c4af8: [u8; 0x60];
    static mut DAT_1008517c: [*mut i8; 0];
    static mut DAT_100a808c: [i8; 0];
    static mut DAT_100a8094: [i8; 0];
    static mut DAT_100a80a4: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000d230(param_1: i32) -> i32;
    fn FUN_1000d260(param_1: i32, param_2: i32) -> i32;
    fn FUN_100609d0(param_1: i32, param_2: i32, param_3: i32, param_4: i32);
    fn FUN_1006e7b0(param_1: *mut i8, param_2: *const i8, ...) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050c30(
        this_: *mut u8,
        param_1: i32,
        param_2: i32,
        param_3: *mut i8,
        param_4: i32,
    ) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006074e(in_zf: i32, in_stack_00000120: *mut i32) -> *mut i32 {
    if in_zf != 0 {
        return core::ptr::null_mut();
    }
    let mut buf = [0i8; 0x104];
    let mut count: i32 = 0;
    let mut row_idx: i32 = 0;
    let mut base: i32 = 0;
    // SAFETY: the globals, the callees and the game object table are the
    // game's own memory; indices stay within the 8x12 grid; in_stack_00000120
    // points at the caller's parent frame slots.
    unsafe {
        let flags = (&raw mut DAT_100c4af8).cast::<u8>();
        loop {
            let mut col: i32 = 0;
            loop {
                let obj = DAT_100a7850;
                if *flags.add(col.wrapping_add(base) as usize) != 1 {
                    *flags.add(base.wrapping_add(col) as usize) = 1;
                    let id = ((obj
                        .wrapping_add(0x368)
                        .wrapping_add(base.wrapping_add(col).wrapping_mul(4)))
                        as *const i32)
                        .read_unaligned();
                    if -1 < id {
                        let kind = FUN_1000d230(id);
                        if kind != 3 && kind != 2 {
                            let idx = FUN_1000d260(kind, id);
                            FUN_100609d0(DAT_100a7850, row_idx, col, id);
                            let mut r = row_idx;
                            let mut i11 = base;
                            while i11 < 0x60 {
                                let mut i2 = col;
                                while i2 < 0xc {
                                    if *flags.add(i2.wrapping_add(i11) as usize) != 1 {
                                        let id1 = ((DAT_100a7850
                                            .wrapping_add(0x368)
                                            .wrapping_add(i11.wrapping_add(i2).wrapping_mul(4)))
                                            as *const i32)
                                            .read_unaligned();
                                        if -1 < id1 {
                                            let k7 = FUN_1000d230(id1);
                                            if k7 != 3 && k7 != 2 {
                                                let i8 = FUN_1000d260(k7, id1);
                                                let o3 = DAT_100a7850;
                                                if i8 == idx && k7 == kind && kind != 2 && kind != 3
                                                {
                                                    count = count.wrapping_add(1);
                                                    *flags.add(i2.wrapping_add(i11) as usize) = 1;
                                                    FUN_100609d0(o3, r, i2, id1);
                                                }
                                            }
                                        }
                                    }
                                    i2 = i2.wrapping_add(1);
                                }
                                r = r.wrapping_add(1);
                                i11 = i11.wrapping_add(0xc);
                            }
                            if kind == 0 {
                                let name = (&raw const DAT_1008517c)
                                    .cast::<*mut i8>()
                                    .offset(idx.wrapping_mul(0x10) as isize)
                                    .read();
                                FUN_1006e7b0(
                                    buf.as_mut_ptr(),
                                    (&raw const DAT_100a808c).cast::<i8>(),
                                    count.wrapping_add(1),
                                    name,
                                );
                                return FUN_10050c30(
                                    DAT_100a6740,
                                    in_stack_00000120.read_unaligned(),
                                    in_stack_00000120.add(1).read_unaligned(),
                                    buf.as_mut_ptr(),
                                    in_stack_00000120.add(5).read_unaligned(),
                                );
                            }
                            if kind == 1 {
                                count = count.wrapping_add(1);
                                let name = (&raw const DAT_1008517c)
                                    .cast::<*mut i8>()
                                    .offset(idx.wrapping_mul(0x10) as isize)
                                    .read();
                                let fmt: *const i8 = if count == 1 {
                                    (&raw const DAT_100a8094).cast::<i8>()
                                } else {
                                    (&raw const DAT_100a80a4).cast::<i8>()
                                };
                                FUN_1006e7b0(buf.as_mut_ptr(), fmt, count, name);
                                return FUN_10050c30(
                                    DAT_100a6740,
                                    in_stack_00000120.read_unaligned(),
                                    in_stack_00000120.add(1).read_unaligned(),
                                    buf.as_mut_ptr(),
                                    in_stack_00000120.add(5).read_unaligned(),
                                );
                            }
                        }
                    }
                }
                col = col.wrapping_add(1);
                if col >= 0xc {
                    break;
                }
            }
            base = base.wrapping_add(0xc);
            row_idx = row_idx.wrapping_add(1);
            if 0x5f < base {
                return core::ptr::null_mut();
            }
        }
    }
}
