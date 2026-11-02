// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002a1e0(param_1: i32, param_2: i32) -> i32;
    fn FUN_10008156(param_1: i32, param_2: i32);
    fn FUN_1002a5d4(param_1: i32);
    fn FUN_10042d09(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10029853(param_1: i32) {
    let base = param_1 as *mut u8;
    let mut local_8: i16 = 0;
    // SAFETY: param_1 points at the game's object; all offsets are read and
    // written where the original does, and the callees are the game's own.
    unsafe {
        let p4c = base.byte_add(0x4c).cast::<i32>().read_unaligned() as *mut u8;
        p4c.byte_add(0x25).write(0);
        let p4c = base.byte_add(0x4c).cast::<i32>().read_unaligned() as *mut u8;
        p4c.byte_add(0x27).write(0);

        let sel = base.byte_add(0x176).cast::<i16>().read_unaligned();
        match sel {
            2 => {
                let w = base.byte_add(0x17a).cast::<u16>().read_unaligned();
                let r = FUN_1002a1e0(w as i32, 2);
                if r == 0 {
                    let v = base.byte_add(0x17e).cast::<u32>().read_unaligned();
                    local_8 = v as i16;
                } else {
                    let b = base.byte_add(0x17e).read();
                    if (b & 0x10) != 0 {
                        FUN_10008156(10, base.byte_add(0xc).cast::<i32>().read_unaligned());
                    }
                }
            }
            3 | 6 => {
                if sel == 3 {
                    FUN_10042d09(param_1);
                    let p = base.byte_add(0x4c).cast::<i32>().read_unaligned() as *mut u8;
                    p.byte_add(0xc).cast::<u32>().write_unaligned(0);
                    let p = base.byte_add(0x4c).cast::<i32>().read_unaligned() as *mut u8;
                    p.cast::<u32>().write_unaligned(0);
                    let p = base.byte_add(0x4c).cast::<i32>().read_unaligned() as *mut u8;
                    p.byte_add(4).cast::<u32>().write_unaligned(0);
                }
                let w = base.byte_add(0x17a).cast::<u16>().read_unaligned();
                let r = FUN_1002a1e0(w as i32, 8);
                if r != 0 && (base.byte_add(0x17e).read() & 0x10) != 0 {
                    FUN_10008156(10, base.byte_add(0xc).cast::<i32>().read_unaligned());
                }
            }
            4 => {}
            5 | 7 => {
                if sel == 7 {
                    let w = base.byte_add(0x17a).cast::<u16>().read_unaligned();
                    let r = FUN_1002a1e0(w as i32, 4);
                    if r != 0 && (base.byte_add(0x17e).read() & 0x10) != 0 {
                        FUN_10008156(10, base.byte_add(0xc).cast::<i32>().read_unaligned());
                    }
                }
                FUN_1002a5d4(param_1);
            }
            _ => {}
        }

        let mut v = base.byte_add(0x17e).cast::<u32>().read_unaligned();
        v &= 0xffff_ffef;
        base.byte_add(0x17e).cast::<u32>().write_unaligned(v);
        let mut v = base.byte_add(0x17e).cast::<u32>().read_unaligned();
        v |= local_8 as i32 as u32;
        base.byte_add(0x17e).cast::<u32>().write_unaligned(v);
    }
}
