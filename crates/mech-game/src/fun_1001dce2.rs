// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab4c0: u32;
    static mut DAT_100ab2a0: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100108a0(param_1: u32, param_2: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001dce2(param_1: i32) -> i32 {
    let mut local_10: i32 = 0;
    // SAFETY: param_1 is the caller's structure address; all reads follow the
    // original's pointers and the globals/callee are the game's own.
    unsafe {
        let base = param_1 as usize;
        let cur: i32 = ((base + 0x1b) as *const i32).read_unaligned();
        let first: u32 = (cur as usize as *const u32).read_unaligned();
        let mut local_c: i32 = if first == DAT_100ab4c0 {
            0xc
        } else {
            ((cur as usize).wrapping_add(4) as *const i32).read_unaligned()
        };
        let cur2: i32 = ((base + 0x1b) as *const i32).read_unaligned();
        local_c = cur2.wrapping_add(local_c);
        let f17: i32 = ((base + 0x17) as *const i32).read_unaligned();
        let f0f: i32 = ((base + 0xf) as *const i32).read_unaligned();
        if local_c.wrapping_sub(f17) < f0f {
            let f13: i32 = ((base + 0x13) as *const i32).read_unaligned();
            let v: i32 = ((local_c as usize).wrapping_add(4) as *const i32).read_unaligned();
            let v2: u32 = ((local_c as usize).wrapping_add(4) as *const u32).read_unaligned();
            if f13 < v || v2 < 8 {
                FUN_100108a0(0x2c, &raw mut DAT_100ab2a0);
            } else {
                ((base + 0x1b) as *mut i32).write_unaligned(local_c);
                local_10 = local_c;
            }
        }
    }
    local_10
}
