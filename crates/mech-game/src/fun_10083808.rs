// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
    static mut DAT_100ef034: [u8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1000dd4e(param_1: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10083808() -> u32 {
    mech_trace::enter!("10083808");
    let mut local_10: u32 = 0;
    // SAFETY: the globals and the callee are the game's own memory; the
    // table entries are addresses of game structures read where the original reads.
    unsafe {
        let tbl = (&raw mut DAT_100eef40).cast::<u32>();
        let base1 = mech_trace::rd!(
            "10083808",
            tbl.add(mech_trace::gr!("10083808", "100adf38", DAT_100adf38) as usize),
            read_unaligned
        );
        let u_var1 = mech_trace::rd!(
            "10083808",
            ((base1.wrapping_add(0xdc)) as *const u32),
            read_unaligned
        ) & 0xf00;
        let base2 = mech_trace::rd!(
            "10083808",
            tbl.add(mech_trace::gr!("10083808", "100adf38", DAT_100adf38) as usize),
            read_unaligned
        );
        let u_var2 = mech_trace::rd!(
            "10083808",
            ((base2.wrapping_add(0xdc)) as *const u32),
            read_unaligned
        ) & 0xff;
        if u_var1 == 0x200 {
            let base3 = mech_trace::rd!("10083808", tbl.add(u_var2 as usize), read_unaligned);
            local_10 = mech_trace::rd!(
                "10083808",
                ((base3.wrapping_add(0x40)) as *const u32),
                read_unaligned
            );
        } else if u_var1 == 0x400 {
            let p = (&raw mut DAT_100ef034)
                .cast::<u8>()
                .add(u_var2.wrapping_mul(0x40) as usize);
            let v = mech_trace::rd!("10083808", p.cast::<i32>(), read_unaligned);
            local_10 = mech_trace::cr!("10083808", "1000dd4e", FUN_1000dd4e(v));
        }
    }
    mech_trace::rt!("10083808", local_10)
}
