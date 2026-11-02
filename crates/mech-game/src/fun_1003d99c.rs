// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aefc0: u32;
    static mut DAT_100c6d48: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1003e369(param_1: i32) -> u8;
    fn FUN_1003e3e3(param_1: i32, param_2: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003d99c(param_1: i32, param_2: i32, param_3: i32) {
    // SAFETY: all pointers are the game's own structures and globals; reads,
    // writes and calls occur in the same order as the original.
    unsafe {
        if param_1 != 0 && DAT_100aefc0 != 0 {
            let p1 = param_1 as usize;
            if ((p1 + 0x6c) as *const i32).read_unaligned() != 0 {
                FUN_1003e369(((p1 + 0x6c) as *const i32).read_unaligned());
                let base = (&raw mut DAT_100c6d48) as *mut u8 as i32;
                let addr = base
                    .wrapping_add(param_2.wrapping_mul(0x120))
                    .wrapping_add(param_3.wrapping_mul(0x18));
                FUN_1003e3e3(((p1 + 0x6c) as *const i32).read_unaligned(), addr);
                let o = ((p1 + 0x6c) as *const i32).read_unaligned();
                ((o as usize + 0x44) as *mut i32).write_unaligned(param_2);
                let o = ((p1 + 0x6c) as *const i32).read_unaligned();
                ((o as usize + 0x48) as *mut i32).write_unaligned(param_3);
                let mut local_8 = ((p1 + 4) as *const i32).read_unaligned();
                while local_8 != 0 {
                    let l = local_8 as usize;
                    let c = ((l + 0x6c) as *const i32).read_unaligned();
                    if c != 0 {
                        let flags = (c as usize as *const u16).read_unaligned();
                        let w = ((c as usize + 2) as *const u16).read_unaligned();
                        if (flags & 0x1000) == 0 || ((w as u8) & 0xf0) != 0x70 {
                            FUN_1003d99c(local_8, param_2, param_3);
                        }
                    }
                    local_8 = ((l + 8) as *const i32).read_unaligned();
                }
            }
        }
    }
}
