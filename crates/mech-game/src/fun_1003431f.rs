// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_101eb480: u32;
    static mut DAT_101eb484: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10034268(
        param_1: *mut i32,
        param_2: *mut i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
    ) -> u8;
    fn FUN_10019df1(param_1: i32, param_2: i32, param_3: i32) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003431f(param_1: *mut i32) {
    let mut local_c: i32 = 0;
    let mut local_8: i32 = 0;
    // SAFETY: param_1 points at the caller's struct of at least 8 ints; the
    // callees and globals are the game's own, called and written in the
    // original order.
    unsafe {
        let p2 = param_1.add(2).read_unaligned();
        let p3 = param_1.add(3).read_unaligned();
        let p5 = param_1.add(5).read_unaligned();
        let p7 = param_1.add(7).read_unaligned();
        let _ = (p2, p3, p5, p7);

        FUN_10034268(
            &raw mut local_8,
            &raw mut local_c,
            param_1.add(2).read_unaligned().wrapping_neg(),
            param_1.add(3).read_unaligned().wrapping_neg(),
            param_1.add(5).read_unaligned(),
        );
        DAT_101eb480 = param_1.read_unaligned().wrapping_add(local_8) as u32;
        DAT_101eb484 = param_1.add(1).read_unaligned().wrapping_add(local_c) as u32;

        FUN_10034268(
            &raw mut local_8,
            &raw mut local_c,
            param_1.add(2).read_unaligned(),
            param_1.add(3).read_unaligned().wrapping_neg(),
            param_1.add(5).read_unaligned(),
        );
        FUN_10019df1(
            param_1.read_unaligned().wrapping_add(local_8),
            param_1.add(1).read_unaligned().wrapping_add(local_c),
            param_1.add(7).read_unaligned(),
        );

        FUN_10034268(
            &raw mut local_8,
            &raw mut local_c,
            param_1.add(2).read_unaligned(),
            param_1.add(3).read_unaligned(),
            param_1.add(5).read_unaligned(),
        );
        FUN_10019df1(
            param_1.read_unaligned().wrapping_add(local_8),
            param_1.add(1).read_unaligned().wrapping_add(local_c),
            param_1.add(7).read_unaligned(),
        );

        FUN_10034268(
            &raw mut local_8,
            &raw mut local_c,
            param_1.add(2).read_unaligned().wrapping_neg(),
            param_1.add(3).read_unaligned(),
            param_1.add(5).read_unaligned(),
        );
        FUN_10019df1(
            param_1.read_unaligned().wrapping_add(local_8),
            param_1.add(1).read_unaligned().wrapping_add(local_c),
            param_1.add(7).read_unaligned(),
        );

        FUN_10034268(
            &raw mut local_8,
            &raw mut local_c,
            param_1.add(2).read_unaligned().wrapping_neg(),
            param_1.add(3).read_unaligned().wrapping_neg(),
            param_1.add(5).read_unaligned(),
        );
        FUN_10019df1(
            param_1.read_unaligned().wrapping_add(local_8),
            param_1.add(1).read_unaligned().wrapping_add(local_c),
            param_1.add(7).read_unaligned(),
        );
    }
}
