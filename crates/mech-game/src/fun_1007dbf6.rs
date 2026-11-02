// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf34: u32;
    static mut DAT_100adf38: u32;
    static mut DAT_100b9420: u32;
    static mut DAT_100eef40: [u32; 1];
    static mut DAT_10160be4: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100069d3(param_1: *mut i32, param_2: i32) -> u8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1004839c() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007dbf6(
    param_1: u32,
    param_2: u32,
    param_3: i32,
    param_4: u32,
    param_5: u32,
    param_6: i32,
) {
    mech_trace::enter!(
        "1007dbf6", param_1, param_2, param_3, param_4, param_5, param_6
    );
    let _ = (param_1, param_2, param_4, param_5);
    if param_6 != 0 {
        // SAFETY: param_6 is a caller-provided address of a structure; the
        // globals and callees are the game's own memory and code.
        unsafe {
            let field = mech_trace::rd!(
                "1007dbf6",
                ((param_6 as u32).wrapping_add(0x10) as *const i32),
                read_unaligned
            );
            if (field == param_3 && mech_trace::gr!("1007dbf6", "10160be4", DAT_10160be4) == 0xd)
                || mech_trace::gr!("1007dbf6", "10160be4", DAT_10160be4).wrapping_sub(0x31)
                    == param_3 as u32
            {
                mech_trace::gw!("1007dbf6", "100b9420", DAT_100b9420, 1);
                mech_trace::wr!(
                    "1007dbf6",
                    (mech_trace::gr!("1007dbf6", "100adf34", DAT_100adf34).wrapping_add(1)
                        as *mut u8),
                    write,
                    0
                );
                let base = (&raw mut DAT_100eef40).cast::<u32>();
                let entry = mech_trace::rd!(
                    "1007dbf6",
                    base.wrapping_add(
                        mech_trace::gr!("1007dbf6", "100adf38", DAT_100adf38) as usize
                    ),
                    read_unaligned
                );
                let p = mech_trace::rd!(
                    "1007dbf6",
                    (entry.wrapping_add(0x20) as *const u32),
                    read_unaligned
                );
                mech_trace::cr!("1007dbf6", "100069d3", FUN_100069d3(p as *mut i32, 0));
                mech_trace::cr!("1007dbf6", "1004839c", FUN_1004839c());
            }
        }
    }
}
