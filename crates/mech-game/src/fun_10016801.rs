// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eef40: [i32; 1];
    static mut DAT_100adf38: i32;
    static mut DAT_100aa578: [i32; 2];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10056489(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
        param_7: i32,
    ) -> u32;
    fn FUN_10011e60(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_10067d51(param_1: *mut i32) -> i32;
    fn FUN_10016b61(
        param_1: *mut i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    );
}

#[inline(always)]
unsafe fn rd(base: i32, off: i32) -> i32 {
    // SAFETY: the caller guarantees base + off addresses a readable i32 in the game's memory.
    unsafe { (base.wrapping_add(off) as usize as *const i32).read_unaligned() }
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016801(param_1: i32) {
    // SAFETY: param_1 points at the game's own structure; all offsets, globals
    // and callees are the game's memory, accessed in the original's order.
    unsafe {
        let pi_var1 = rd(param_1, 0xa4) as usize as *mut i32;
        let idx = DAT_100adf38;
        let elem = (&raw mut DAT_100eef40)
            .cast::<i32>()
            .offset(idx as isize)
            .read_unaligned();
        let pi_var2 = rd(elem, 0x20) as usize as *const i32;

        FUN_10056489(
            pi_var1,
            rd(param_1, 0x3c),
            rd(param_1, 0x40),
            rd(param_1, 0x3c),
            rd(param_1, 0x48),
            0,
            10,
        );
        FUN_10056489(
            pi_var1,
            rd(param_1, 0x44),
            rd(param_1, 0x40),
            rd(param_1, 0x44),
            rd(param_1, 0x48),
            0,
            10,
        );
        FUN_10056489(
            pi_var1,
            rd(param_1, 0x3c),
            rd(param_1, 0x40),
            rd(param_1, 0x44),
            rd(param_1, 0x40),
            0,
            10,
        );
        FUN_10056489(
            pi_var1,
            rd(param_1, 0x3c),
            rd(param_1, 0x48),
            rd(param_1, 0x44),
            rd(param_1, 0x48),
            0,
            10,
        );

        let x = rd(pi_var2.read_unaligned(), 0x4c);
        let mut local_24 = FUN_10011e60(
            (rd(x, 8) as u32).wrapping_shl(0x10) as i32,
            rd(param_1, 0x38),
            0x400,
        );
        let x = rd(pi_var2.read_unaligned(), 0x4c);
        if (x.wrapping_add(0x30) as usize as *const i8).read() != 0 {
            local_24 /= -2;
        }
        DAT_100aa578[1] = local_24;
        let mut local_18 = FUN_10067d51((&raw mut DAT_100aa578).cast::<i32>());
        local_18 >>= 0x10;
        let mut local_20 = rd(param_1, 0x50);
        let mut local_8 = 0xf;
        let x = rd(pi_var2.read_unaligned(), 0x4c);
        if (x.wrapping_add(0x30) as usize as *const i8).read() == 0 {
            if rd(param_1, 0x38) <= local_18 {
                local_18 = rd(param_1, 0x38).wrapping_sub(1);
            }
        } else {
            local_18 = local_18.wrapping_neg();
            if rd(param_1, 0x38) / 2 < local_18 {
                local_18 = rd(param_1, 0x38) / 2;
            }
            local_8 = 7;
            local_20 = local_20.wrapping_add(local_18);
        }
        FUN_10016b61(
            pi_var1,
            rd(param_1, 0x4c),
            local_20,
            rd(param_1, 0x34).wrapping_sub(1),
            local_18.wrapping_add(1),
            local_8,
        );
    }
}
