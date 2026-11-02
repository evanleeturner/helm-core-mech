// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100544ee(param_1: *mut i32, param_2: *mut i32) -> i32 {
    // SAFETY: both params point at the caller's int arrays; the pointers
    // stored in them are the game's own, read in the original's order.
    unsafe {
        let p1 = |i: usize| param_1.add(i).read_unaligned();
        let p2 = |i: usize| param_2.add(i).read_unaligned();
        let skip = p2(0) == 0
            || p2(2) < p1(5)
            || p1(2) < p2(5)
            || p2(3) < p1(6)
            || p1(3) < p2(6)
            || p2(4) < p1(7)
            || p1(4) < p2(7)
            || ((((p2(0) as *const i16).read_unaligned()) as i32 as u32) & 0x8000) == 0
            || ((((p1(0) as *const i16).read_unaligned()) as i32 as u32) & 0x8000) == 0;
        if skip {
            (p1(1) < p2(1)) as i32
        } else {
            let mut local_30: i32 = 0;
            loop {
                let inner = ((p1(0) as usize).wrapping_add(4) as *const i32).read_unaligned();
                let lim = core::ptr::read_volatile((inner as usize).wrapping_add(2) as *const u16);
                if !(local_30 < lim as i32) {
                    break;
                }
                local_30 = local_30.wrapping_add(1);
            }
            1
        }
    }
}
