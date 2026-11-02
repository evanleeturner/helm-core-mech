// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100cded4: u8;
    static mut DAT_100cded8: u8;
    static mut DAT_100cdedc: u8;
    static mut DAT_100cdee0: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000fc30(param_1: i32, param_2: i32, param_3: i32) -> i32;
    fn FUN_1007bf28(param_1: i32) -> u8;
    fn FUN_10002570(param_1: u32, param_2: i32) -> u32;
    fn FUN_10003d60(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007c7bc(param_1: i32, param_2: i32, param_3: i32, param_4: i32) {
    let i_var1 = param_1.wrapping_mul(0x24) as isize;
    // SAFETY: the globals are the game's own tables, addressed by symbol and
    // offset by the entry stride; the callees are the game's functions.
    unsafe {
        let p0 = (&raw mut DAT_100cded4).byte_offset(i_var1).cast::<i32>();
        let p1 = (&raw mut DAT_100cded8).byte_offset(i_var1).cast::<i32>();
        let p2 = (&raw mut DAT_100cdedc).byte_offset(i_var1).cast::<i32>();
        let p3 = (&raw mut DAT_100cdee0).byte_offset(i_var1).cast::<i32>();
        if p0.read_unaligned() != 0 {
            let mut i_var2 = FUN_1000fc30(param_2, param_3, param_4);
            if i_var2 != 0 {
                FUN_1007bf28(param_1);
                let i_var3 = FUN_1000fc30(
                    p1.read_unaligned(),
                    p2.read_unaligned(),
                    p3.read_unaligned(),
                );
                if i_var2 < i_var3 {
                    i_var2 = FUN_10002570((i_var2 as u32).wrapping_mul(2), i_var3) as i32;
                    let v = FUN_10003d60(p1.read_unaligned(), i_var2);
                    p1.write_unaligned(v);
                    let v = FUN_10003d60(p2.read_unaligned(), i_var2);
                    p2.write_unaligned(v);
                    let v = FUN_10003d60(p3.read_unaligned(), i_var2);
                    p3.write_unaligned(v);
                }
                p1.write_unaligned(p1.read_unaligned().wrapping_add(param_2));
                p2.write_unaligned(p2.read_unaligned().wrapping_add(param_3));
                p3.write_unaligned(p3.read_unaligned().wrapping_add(param_4));
            }
        }
    }
}
