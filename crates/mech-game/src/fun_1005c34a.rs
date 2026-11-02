// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_10114db0: [i32; 17];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000f47f();
    fn FUN_1000f1f0();
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut c_void, flags: u32, size: u32) -> *mut c_void;
    fn HeapFree(h: *mut c_void, flags: u32, mem: *mut c_void) -> i32;
}

unsafe fn rd(addr: i32) -> i32 {
    // SAFETY: caller passes an address inside the game's own structures.
    unsafe { mech_trace::rd!("1005c34a", (addr as usize as *const i32), read_unaligned) }
}

unsafe fn tbl(idx: i32) -> i32 {
    // SAFETY: the global table is the game's own memory.
    unsafe {
        mech_trace::rd!(
            "1005c34a",
            (&raw const DAT_10114db0).cast::<i32>().offset(idx as isize),
            read
        )
    }
}

unsafe fn setup(idx: i32, local_c: i32, local_14: i32) {
    // SAFETY: the table entry points at a game object holding two function
    // pointers at +0xcc and +200; slots +0xf0 and +0xf4 are written as the original does.
    unsafe {
        let obj = tbl(idx) as usize as *mut u8;
        let f1: extern "C" fn(i32, i32) = core::mem::transmute(mech_trace::rd!(
            "1005c34a",
            obj.byte_add(0xcc).cast::<usize>(),
            read_unaligned
        ));
        mech_trace::cr!(
            "1005c34a",
            "ind",
            f1(
                tbl(idx),
                (rd(local_c.wrapping_add(4)) as u32)
                    .wrapping_mul(0x68)
                    .wrapping_add(0x100a7c00) as i32,
            )
        );
        let f2: extern "C" fn(i32, i32) = core::mem::transmute(mech_trace::rd!(
            "1005c34a",
            obj.byte_add(200).cast::<usize>(),
            read_unaligned
        ));
        mech_trace::cr!("1005c34a", "ind", f2(tbl(idx), local_14));
        mech_trace::wr!(
            "1005c34a",
            obj.byte_add(0xf0).cast::<u32>(),
            write_unaligned,
            FUN_1000f47f as usize as u32
        );
        mech_trace::wr!(
            "1005c34a",
            obj.byte_add(0xf4).cast::<u32>(),
            write_unaligned,
            FUN_1000f1f0 as usize as u32
        );
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1005c34a(param_1: i32) {
    mech_trace::enter!("1005c34a", param_1);
    // SAFETY: all memory touched belongs to the game; param_1 points at its
    // structure whose +0x54 field holds a table of 0xe entries of 0x40 bytes.
    unsafe {
        let mut b_var1 = true;
        let mut local_14: i32;
        let mut local_c: i32;
        let mut local_8: i32;
        let mut local_20: i32;
        let mut local_2c: i32;
        let mut local_10: i32;

        local_14 = 0;
        while local_14 < 0xe {
            let a = rd(param_1.wrapping_add(0x54))
                .wrapping_add(0x30)
                .wrapping_add((local_14 as u32).wrapping_mul(0x40) as i32);
            mech_trace::wr!(
                "1005c34a",
                (a as usize as *mut i32),
                write_unaligned,
                local_14
            );
            local_14 += 1;
        }
        local_c = rd(param_1.wrapping_add(0x54));
        local_8 = 10;
        local_14 = 0;
        while local_14 < 0xe {
            let k = rd(local_c.wrapping_add(0x28));
            if k == 4 || k == 1 || k == 6 {
                local_8 = local_8.wrapping_add(1);
            }
            local_c = local_c.wrapping_add(0x40);
            local_14 += 1;
        }
        local_c = rd(param_1.wrapping_add(0x54));
        local_20 = 3;
        local_14 = 0;
        while local_14 < 0xe {
            let k = rd(local_c.wrapping_add(0x28));
            if k == 5 || k == 3 || k == 7 {
                setup(local_20, local_c, local_14);
                local_20 = local_20.wrapping_add(1);
            }
            if local_20 == 10 {
                local_20 = local_8;
            }
            local_c = local_c.wrapping_add(0x40);
            local_14 += 1;
        }
        local_c = rd(param_1.wrapping_add(0x54));
        local_2c = 10;
        local_14 = 0;
        while local_14 < 0xe {
            let k = rd(local_c.wrapping_add(0x28));
            if k == 4 || k == 1 || k == 6 {
                setup(local_2c, local_c, local_14);
                local_2c = local_2c.wrapping_add(1);
            }
            if 0x10 < local_2c {
                local_2c = local_20;
            }
            local_c = local_c.wrapping_add(0x40);
            local_14 += 1;
        }
        local_c = rd(param_1.wrapping_add(0x54));
        local_14 = 0;
        while local_14 < 0xe {
            let k = rd(local_c.wrapping_add(0x28));
            if k == 2 || k == 0 {
                if b_var1 && local_20 < local_2c {
                    local_10 = local_2c;
                    if local_2c == 0x11 {
                        local_10 = local_20;
                        local_20 = local_20.wrapping_add(1);
                    } else {
                        local_2c = local_2c.wrapping_add(1);
                    }
                    b_var1 = false;
                } else {
                    local_10 = local_20;
                    if local_20 == 10 {
                        local_10 = local_2c;
                        local_2c = local_2c.wrapping_add(1);
                    } else {
                        local_20 = local_20.wrapping_add(1);
                    }
                    b_var1 = true;
                }
                setup(local_10, local_c, local_14);
            }
            local_c = local_c.wrapping_add(0x40);
            local_14 += 1;
        }

        let lp_mem = mech_trace::cr!(
            "1005c34a",
            "HeapAlloc",
            HeapAlloc(
                mech_trace::gr!("1005c34a", "100adf74", DAT_100adf74),
                9,
                0x380
            )
        ) as *mut u32;
        let mut local_18 = lp_mem;
        local_14 = 3;
        while local_14 < 10 {
            let obj = tbl(local_14) as usize as *mut u8;
            let v = mech_trace::rd!("1005c34a", obj.byte_add(0xc).cast::<i32>(), read_unaligned);
            if v != -1 {
                let mut p4 = (((v as u32).wrapping_mul(0x40) as i32)
                    .wrapping_add(rd(param_1.wrapping_add(0x54)))
                    as usize) as *const u32;
                let mut p5 = local_18;
                let mut n = 0x10;
                while n != 0 {
                    mech_trace::wr!(
                        "1005c34a",
                        p5,
                        write_unaligned,
                        mech_trace::rd!("1005c34a", p4, read_unaligned)
                    );
                    p4 = p4.wrapping_add(1);
                    p5 = p5.wrapping_add(1);
                    n -= 1;
                }
                if local_14 != 9 {
                    local_18 = local_18.wrapping_add(0x20);
                }
            }
            local_14 += 1;
        }
        local_18 = lp_mem.wrapping_add(0x10);
        local_14 = 10;
        while local_14 < 0x11 {
            let obj = tbl(local_14) as usize as *mut u8;
            let v = mech_trace::rd!("1005c34a", obj.byte_add(0xc).cast::<i32>(), read_unaligned);
            if v != -1 {
                let mut p4 = (((v as u32).wrapping_mul(0x40) as i32)
                    .wrapping_add(rd(param_1.wrapping_add(0x54)))
                    as usize) as *const u32;
                let mut p5 = local_18;
                let mut n = 0x10;
                while n != 0 {
                    mech_trace::wr!(
                        "1005c34a",
                        p5,
                        write_unaligned,
                        mech_trace::rd!("1005c34a", p4, read_unaligned)
                    );
                    p4 = p4.wrapping_add(1);
                    p5 = p5.wrapping_add(1);
                    n -= 1;
                }
                if local_14 != 0x10 {
                    local_18 = local_18.wrapping_add(0x20);
                }
            }
            local_14 += 1;
        }
        let mut local_24 = 0i32;
        local_14 = 3;
        while local_14 < 10 {
            let obj = tbl(local_14) as usize as *mut u8;
            if mech_trace::rd!("1005c34a", obj.byte_add(0xc).cast::<i32>(), read_unaligned) != -1 {
                mech_trace::wr!(
                    "1005c34a",
                    obj.byte_add(0xc).cast::<i32>(),
                    write_unaligned,
                    local_24
                );
                local_24 = local_24.wrapping_add(2);
            }
            local_14 += 1;
        }
        local_24 = 1;
        local_14 = 10;
        while local_14 < 0x11 {
            let obj = tbl(local_14) as usize as *mut u8;
            if mech_trace::rd!("1005c34a", obj.byte_add(0xc).cast::<i32>(), read_unaligned) != -1 {
                mech_trace::wr!(
                    "1005c34a",
                    obj.byte_add(0xc).cast::<i32>(),
                    write_unaligned,
                    local_24
                );
                local_24 = local_24.wrapping_add(2);
            }
            local_14 += 1;
        }
        let mut p4 = lp_mem as *const u32;
        let mut p5 = rd(param_1.wrapping_add(0x54)) as usize as *mut u32;
        let mut n = 0xe0;
        while n != 0 {
            mech_trace::wr!(
                "1005c34a",
                p5,
                write_unaligned,
                mech_trace::rd!("1005c34a", p4, read_unaligned)
            );
            p4 = p4.wrapping_add(1);
            p5 = p5.wrapping_add(1);
            n -= 1;
        }
        mech_trace::cr!(
            "1005c34a",
            "HeapFree",
            HeapFree(
                mech_trace::gr!("1005c34a", "100adf74", DAT_100adf74),
                1,
                lp_mem as *mut c_void
            )
        );
        local_14 = 0;
        while local_14 < 0xe {
            let pi = rd(param_1.wrapping_add(0x54))
                .wrapping_add((local_14 as u32).wrapping_mul(0x40) as i32)
                as usize as *mut i32;
            if mech_trace::rd!("1005c34a", pi, read_unaligned) == 0 {
                mech_trace::wr!("1005c34a", pi.wrapping_add(1), write_unaligned, -1);
            }
            local_14 += 1;
        }
    }
}
