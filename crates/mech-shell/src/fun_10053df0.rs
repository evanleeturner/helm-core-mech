// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100bfed8: [i32; 64];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100576a0(param_1: i32) -> i32;
    fn __strcmpi(a: *const i8, b: *const i8) -> i32;
    fn _strncpy(a: *mut i8, b: *const i8, n: u32) -> *mut i8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10053df0(param_1: i32) {
    let mut slot_had_index = [0i32; 16];
    // SAFETY: the global table and callees are the game's own; param_1 is the
    // caller's record table, accessed at the same offsets as the original.
    unsafe {
        let base: *mut i32 = (&raw mut DAT_100bfed8).cast::<i32>();
        let end: *mut i32 = base.add(64);

        let mut slot_cursor = base;
        let mut p5: *mut i32 = slot_had_index.as_mut_ptr();
        loop {
            if slot_cursor.read() == -1 {
                p5.write(0);
            } else {
                p5.write(1);
                slot_cursor.write(-1);
            }
            p5 = p5.add(1);
            slot_cursor = slot_cursor.add(4);
            if slot_cursor >= end {
                break;
            }
        }

        let mut entry_index: i32 = 0;
        let mut p5 = base;
        loop {
            let i2 = FUN_100576a0(entry_index);
            if i2 == 0 {
                break;
            }
            let mut i6: i32 = 0;
            let mut slot_name: *mut i8 = base.add(1).cast::<i8>();
            let limit: *mut i8 = base.cast::<i8>().add(0x104);
            loop {
                if ((i2.wrapping_add(0x50)) as *const i8).read() == 0 {
                    break;
                }
                let r = __strcmpi(slot_name, (i2.wrapping_add(0x50)) as *const i8);
                if r == 0 && p5.read() == -1 {
                    base.offset((i6.wrapping_mul(4)) as isize)
                        .write(entry_index);
                    break;
                }
                slot_name = slot_name.add(0x10);
                i6 = i6.wrapping_add(1);
                if slot_name >= limit {
                    break;
                }
            }
            p5 = p5.add(4);
            entry_index = entry_index.wrapping_add(1);
            if p5 >= end {
                break;
            }
        }

        let mut slot_cursor = base;
        let mut p5: *mut i32 = slot_had_index.as_mut_ptr();
        let had_end: *mut i32 = p5.add(16);
        loop {
            if p5.read() != 0 && slot_cursor.read() == -1 && (slot_cursor.add(1).read() as i8) != 0
            {
                let mut i2: i32 = 0;
                loop {
                    let mut i6 = FUN_100576a0(i2);
                    if i6 != 0 && ((i6.wrapping_add(4)) as *const i8).read() == 0x6a {
                        i6 = 0;
                        let mut scan = base;
                        loop {
                            if scan.read() == i2 {
                                break;
                            }
                            scan = scan.add(4);
                            i6 = i6.wrapping_add(1);
                            if scan >= end {
                                break;
                            }
                        }
                        if i6 == 0x10 {
                            slot_cursor.write(i2);
                            break;
                        }
                    }
                    i2 = i2.wrapping_add(1);
                    if i2 >= 0x10 {
                        break;
                    }
                }
            }
            slot_cursor = slot_cursor.add(4);
            p5 = p5.add(1);
            if p5 >= had_end {
                break;
            }
        }

        let mut i2: i32 = 0x94;
        let mut p5: *mut i32 = (param_1.wrapping_add(4)) as *mut i32;
        loop {
            if p5.read() > -1 {
                let idx = p5.read();
                let i6 = FUN_100576a0(base.offset(idx.wrapping_mul(4) as isize).read());
                if i6 != 0 {
                    if p5.sub(1).read() == 0 && p5.add(4).read() == -1 {
                        if ((i6.wrapping_add(0x5c)) as *const i32).read_unaligned()
                            <= p5.add(3).read()
                        {
                            p5.write(-1);
                        }
                    } else if ((i6.wrapping_add(0x60)) as *const i32).read_unaligned()
                        <= p5.add(3).read()
                    {
                        p5.write(-1);
                    }
                    if p5.read() > -1 {
                        let v = base.offset(p5.read().wrapping_mul(4) as isize).read();
                        p5.write(v);
                    }
                }
            }
            p5 = p5.add(6);
            i2 = i2.wrapping_sub(1);
            if i2 == 0 {
                let mut i2: i32 = 0;
                let mut p5 = base;
                loop {
                    let i6 = FUN_100576a0(i2);
                    if i6 == 0 {
                        p5.write(-1);
                        p5.add(1).cast::<u8>().write(0);
                    } else {
                        p5.write(i2);
                        _strncpy(
                            p5.add(1).cast::<i8>(),
                            (i6.wrapping_add(0x50)) as *const i8,
                            0x10,
                        );
                    }
                    p5 = p5.add(4);
                    i2 = i2.wrapping_add(1);
                    if p5 >= end {
                        break;
                    }
                }
                return;
            }
        }
    }
}
