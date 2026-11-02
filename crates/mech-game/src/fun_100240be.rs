// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab378: u32;
    static mut DAT_100ab37c: u32;
    static mut DAT_100ab384: u32;
    static mut DAT_1016acc0: [u32; 1];
    static mut DAT_1016cc00: [i8; 1];
    static mut DAT_1016cc50: i32;
    static mut DAT_1016cc68: [u8; 1];
    static mut DAT_1016cc70: [u8; 1];
    static mut DAT_1016ce32: [u8; 1];
    static mut DAT_100ab388: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1002f727(param_1: i32) -> u8;
    fn FUN_10023a8c(param_1: i32) -> *mut i32;
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100240be(param_1: i32) -> i32 {
    mech_trace::enter!("100240be", param_1);
    // SAFETY: all globals and callees are the game's own memory and code;
    // list nodes and table entries are read and written where the original does.
    unsafe {
        let mut i_var1: i32 = mech_trace::gr!("100240be", "100ab384", DAT_100ab384) as i32;
        if param_1 != -1 {
            if param_1 == 0 && mech_trace::gr!("100240be", "100ab37c", DAT_100ab37c) != 0 {
                mech_trace::gw!("100240be", "1016cc50", DAT_1016cc50, 0);
                let mut local_c: i32 = mech_trace::gr!("100240be", "100ab37c", DAT_100ab37c) as i32;
                while local_c != 0 {
                    let v = mech_trace::rd!(
                        "100240be",
                        ((local_c as u32).wrapping_add(8) as *const u32),
                        read_unaligned
                    );
                    mech_trace::wr!(
                        "100240be",
                        (&raw mut DAT_1016acc0)
                            .cast::<u32>()
                            .offset(mech_trace::gr!("100240be", "1016cc50", DAT_1016cc50) as isize),
                        write_unaligned,
                        v
                    );
                    mech_trace::gw!(
                        "100240be",
                        "1016cc50",
                        DAT_1016cc50,
                        mech_trace::gr!("100240be", "1016cc50", DAT_1016cc50).wrapping_add(1)
                    );
                    local_c = mech_trace::rd!(
                        "100240be",
                        ((local_c as u32).wrapping_add(4) as *const i32),
                        read_unaligned
                    );
                }
                let mut local_8: i32 = 0;
                while local_8 < mech_trace::gr!("100240be", "1016cc50", DAT_1016cc50) {
                    let e = mech_trace::rd!(
                        "100240be",
                        (&raw mut DAT_1016acc0)
                            .cast::<u32>()
                            .offset(local_8 as isize),
                        read_unaligned
                    );
                    i_var1 = (e as i32).wrapping_mul(0x1fa);
                    let p = (&raw mut DAT_1016cc70)
                        .cast::<u8>()
                        .byte_offset(i_var1 as isize)
                        .cast::<u16>();
                    mech_trace::wr!(
                        "100240be",
                        p,
                        write_unaligned,
                        mech_trace::rd!("100240be", p, read_unaligned) & 0xfffe
                    );
                    let a = mech_trace::rd!(
                        "100240be",
                        (&raw mut DAT_1016ce32)
                            .cast::<u8>()
                            .byte_offset(i_var1 as isize)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    mech_trace::cr!("100240be", "1002f727", FUN_1002f727(a));
                    let b = mech_trace::rd!(
                        "100240be",
                        (&raw mut DAT_1016cc68)
                            .cast::<u8>()
                            .byte_offset(i_var1 as isize)
                            .cast::<i32>(),
                        read_unaligned
                    );
                    mech_trace::cr!("100240be", "10023a8c", FUN_10023a8c(b));
                    local_8 = local_8.wrapping_add(1);
                }
                mech_trace::gw!("100240be", "100ab378", DAT_100ab378, 0);
            }
            mech_trace::gw!("100240be", "100ab384", DAT_100ab384, param_1 as u32);
            mech_trace::cr!(
                "100240be",
                "sprintf",
                sprintf(
                    (&raw mut DAT_1016cc00).cast::<i8>(),
                    (&raw mut DAT_100ab388).cast::<i8>(),
                    param_1,
                )
            );
            i_var1 = param_1;
        }
        mech_trace::rt!("100240be", i_var1)
    }
}
