// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
    fn FUN_1005982d(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10067569(param_1: *mut i32, param_2: *mut i32, param_3: i32) {
    // SAFETY: the global and callees are the game's own; param_1 and param_2
    // point at the caller's int arrays, read where the original reads.
    unsafe {
        let mut buf: [i32; 5] = [0; 5];
        let local_8 = (DAT_100aaf68.wrapping_add(0x44) as *const i32).read_unaligned();
        let p1_0 = param_1.read_unaligned();
        let p1_1 = param_1.add(1).read_unaligned();
        let p1_2 = param_1.add(2).read_unaligned();
        let p1_3 = param_1.add(3).read_unaligned();
        let p1_4 = param_1.add(4).read_unaligned();
        let mut local_18 = (p1_3.wrapping_sub(p1_1).wrapping_add(1)) >> 1;
        let mut local_14 = (p1_4.wrapping_sub(p1_2).wrapping_add(1)) >> 1;
        let local_c = (p1_3.wrapping_sub(p1_1).wrapping_add(1) / 2).wrapping_add(-1);
        let local_10 = FUN_10003d60(local_c, local_8);
        let p2_0 = param_2.read_unaligned();
        let p2_1 = param_2.add(1).read_unaligned();
        let p2_2 = param_2.add(2).read_unaligned();
        let p2_3 = param_2.add(3).read_unaligned();
        buf[0] = p1_0;
        buf[1] = p1_1.wrapping_add(p2_0);
        buf[2] = p1_2.wrapping_add(p2_1);
        buf[3] = p1_1.wrapping_add(p2_2);
        buf[4] = p1_2.wrapping_add(p2_3);
        local_18 = local_18.wrapping_sub(p2_0);
        local_14 = local_14.wrapping_sub(p2_1);
        FUN_1005982d(
            buf.as_mut_ptr(),
            local_18,
            local_14,
            local_c,
            local_10,
            param_3,
        );
    }
}
