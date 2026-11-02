// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1002c39a() -> u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002ab38(param_1: i32) -> i32;
}

unsafe fn copy5(src: i32, dst: i32) {
    let mut s = src as usize as *const u32;
    let mut d = dst as usize as *mut u32;
    let mut n = 5;
    while n != 0 {
        // SAFETY: the caller supplies a structure large enough for these offsets.
        unsafe {
            d.write_unaligned(s.read_unaligned());
            s = s.add(1);
            d = d.add(1);
        }
        n -= 1;
    }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10035fd0(param_1: u32, param_2: i32) -> u32 {
    let _ = param_1;
    let mut local_8: i32 = 0;
    if param_2 == 0 {
        return 0;
    }
    // SAFETY: the callees and globals are the game's own; param_2 points at
    // the caller's structure, accessed at the offsets the original uses.
    unsafe {
        let iv2 = FUN_1002c39a() as i32;
        if iv2 != 1 {
            copy5(
                param_2.wrapping_add(0x7c),
                iv2.wrapping_mul(0x14)
                    .wrapping_add(0x18)
                    .wrapping_add(param_2),
            );
            copy5(
                param_2.wrapping_add(0x90),
                iv2.wrapping_add(1)
                    .wrapping_mul(0x14)
                    .wrapping_add(0x18)
                    .wrapping_add(param_2),
            );
            (param_2.wrapping_add(0xc) as usize as *mut i32).write_unaligned(iv2.wrapping_add(2));
            let mut local_14: i32 = 0;
            while local_14 < iv2.wrapping_sub(1) {
                let u3 = FUN_1002ab38(local_14.wrapping_add(1)) as u32;
                if u3 < DAT_100b38d8 {
                    let base = (&raw mut DAT_100eef40).cast::<u32>();
                    let p = base.add(u3 as usize).read_unaligned();
                    let v = (p.wrapping_add(0x176) as usize as *const i16).read_unaligned();
                    if v == 0xc {
                        let addr = local_14
                            .wrapping_mul(4)
                            .wrapping_add(4)
                            .wrapping_mul(5)
                            .wrapping_add(0x18)
                            .wrapping_add(param_2);
                        (addr as usize as *mut u32).write_unaligned(1);
                        local_8 = local_8.wrapping_add(1);
                    }
                }
                local_14 = local_14.wrapping_add(1);
            }
        }
        if iv2 == 1 || iv2.wrapping_sub(1) == local_8 {
            copy5(param_2.wrapping_add(0xa4), param_2.wrapping_add(0x18));
            copy5(param_2.wrapping_add(0x90), param_2.wrapping_add(0x2c));
            (param_2.wrapping_add(0xc) as usize as *mut u32).write_unaligned(2);
        }
    }
    1
}
