// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b4ee0: u32;
    static mut DAT_100b4ee8: u32;
    static mut DAT_100b4ef0: u32;
    static mut DAT_100b4eec: u32;
    static mut DAT_100b4fd8: u32;
    static mut DAT_100b4fdc: u32;
    static mut DAT_100b4fe0: u32;
    static mut DAT_100b4f70: u32;
    static mut DAT_10074068: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007af73(
    param_1: *mut i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: u32,
    param_6: u32,
    param_7: u32,
    param_8: u32,
) {
    mech_trace::enter!(
        "1007af73", param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8
    );
    // SAFETY: the globals are the game's own memory; param_1 points at a
    // pointer to two u32 values, and the table at DAT_10074068 holds
    // function pointers indexed by param_4, as in the original.
    unsafe {
        mech_trace::gw!("1007af73", "100b4eec", DAT_100b4eec, param_2 as u32);
        mech_trace::gw!("1007af73", "100b4fd8", DAT_100b4fd8, param_6);
        mech_trace::gw!("1007af73", "100b4fdc", DAT_100b4fdc, param_7);
        mech_trace::gw!("1007af73", "100b4fe0", DAT_100b4fe0, param_8);
        mech_trace::gw!("1007af73", "100b4f70", DAT_100b4f70, param_5);

        let base = mech_trace::rd!("1007af73", param_1, read_unaligned) as usize as *const u32;
        let local_1 = mech_trace::rd!("1007af73", base.add(1), read_unaligned).wrapping_add(1);
        let local_2 = mech_trace::rd!("1007af73", base, read_unaligned);
        let local_3 = param_3.wrapping_mul(0x18).wrapping_add(param_2) as u32;

        let table = (&raw mut DAT_10074068) as *const u32;
        let fp = mech_trace::rd!(
            "1007af73",
            table
                .wrapping_offset(param_4 as isize)
                .cast::<Option<unsafe extern "C" fn()>>(),
            read_unaligned
        );
        if let Some(f) = fp {
            mech_trace::cr!("1007af73", "ind", f());
        }

        mech_trace::gw!("1007af73", "100b4ee0", DAT_100b4ee0, local_1);
        mech_trace::gw!("1007af73", "100b4ee8", DAT_100b4ee8, local_2);
        mech_trace::gw!("1007af73", "100b4ef0", DAT_100b4ef0, local_3);
    }
}
