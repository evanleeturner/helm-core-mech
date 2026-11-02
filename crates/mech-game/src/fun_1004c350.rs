// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10063fbd(param_1: i32, param_2: *mut i32);
    fn FUN_1002650e(param_1: *mut i32, param_2: i32);
    fn FUN_10014768(param_1: *mut i32, param_2: *mut i32, param_3: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004c350(param_1: i32, param_2: i32) -> bool {
    let mut local_4c: [i32; 14] = [0; 14];
    let mut local_14: i32;
    // SAFETY: param_1 points at the caller's object; fields are read where the original reads them.
    let local_10: i32 = unsafe {
        let a = ((param_1 as u32).wrapping_add(0x6c) as *const i32).read_unaligned();
        let b = ((param_1 as u32).wrapping_add(0x60) as *const i32).read_unaligned();
        let c = ((param_1 as u32).wrapping_add(0xd4) as *const i32).read_unaligned();
        a.wrapping_add(b).wrapping_sub(c).wrapping_add(0x1680000) % 0x1680000
    };
    if param_2 == 0 || local_10 > 0x10dffff || local_10 < 0x5a0001 {
        // SAFETY: the callees are the game's own; the buffers are local and param_1 and
        // the record at local_14 are the caller's memory, read where the original reads.
        unsafe {
            FUN_10063fbd(param_1, local_4c.as_mut_ptr());
            let p = ((param_1 as u32).wrapping_add(0xc0) as *const i32).read_unaligned();
            FUN_1002650e(local_4c.as_mut_ptr(), p);
            local_14 = 0;
            let p4 = ((param_1 as u32).wrapping_add(4) as *const u32).read_unaligned();
            let iv = FUN_10014768(local_4c.as_mut_ptr(), &raw mut local_14, p4) as i32;
            if iv == 0 || local_14 == 0 {
                true
            } else {
                let u1 = ((param_1 as u32).wrapping_add(0xdc) as *const u32).read_unaligned();
                let flags = ((local_14 as u32).wrapping_add(2) as *const u16).read_unaligned();
                let val = ((local_14 as u32).wrapping_add(0x14) as *const u16).read_unaligned();
                if (flags & 0x100) == 0 || (u1 & 0x200) == 0 {
                    if (flags & 0x200) == 0 || (u1 & 0x400) == 0 {
                        false
                    } else {
                        u32::from(val) == (u1 & 0xff)
                    }
                } else {
                    u32::from(val) == (u1 & 0xff)
                }
            }
        }
    } else {
        false
    }
}
