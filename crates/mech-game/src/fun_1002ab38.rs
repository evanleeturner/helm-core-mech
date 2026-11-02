// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002ab38(param_1: i32) -> i32 {
    mech_trace::enter!("1002ab38", param_1);
    let mut local_10: i32 = 0;
    // SAFETY: the globals are the game's own memory; the table entries are
    // pointers to records the original reads at offsets 8 and 0xc.
    unsafe {
        let table = (&raw mut DAT_100eef40).cast::<u32>();
        loop {
            if mech_trace::gr!("1002ab38", "100b38d8", DAT_100b38d8) <= local_10 {
                return mech_trace::rt!("1002ab38", -1);
            }
            let a = mech_trace::rd!(
                "1002ab38",
                table.add(mech_trace::gr!("1002ab38", "100adf38", DAT_100adf38) as usize),
                read_unaligned
            );
            let b = mech_trace::rd!("1002ab38", table.offset(local_10 as isize), read_unaligned);
            if mech_trace::rd!(
                "1002ab38",
                (a.wrapping_add(8) as *const i32),
                read_unaligned
            ) == mech_trace::rd!(
                "1002ab38",
                (b.wrapping_add(8) as *const i32),
                read_unaligned
            ) {
                let b2 =
                    mech_trace::rd!("1002ab38", table.offset(local_10 as isize), read_unaligned);
                if mech_trace::rd!(
                    "1002ab38",
                    (b2.wrapping_add(0xc) as *const i32),
                    read_unaligned
                ) == param_1
                {
                    break;
                }
            }
            local_10 = local_10.wrapping_add(1);
        }
    }
    mech_trace::rt!("1002ab38", local_10)
}
