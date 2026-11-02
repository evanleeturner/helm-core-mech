// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10083715() -> u32 {
    mech_trace::enter!("10083715");
    // SAFETY: the globals are the game's own memory; the table entry is a
    // pointer to a record whose field at 0xdc is read, as in the original.
    unsafe {
        let base = (&raw mut DAT_100eef40).cast::<u32>();
        let rec = mech_trace::rd!(
            "10083715",
            base.add(mech_trace::gr!("10083715", "100adf38", DAT_100adf38) as usize),
            read
        ) as usize;
        let v = mech_trace::rd!("10083715", ((rec + 0xdc) as *const u32), read_unaligned);
        let mut local_8 = v & 0xff;
        let v2 = mech_trace::rd!("10083715", ((rec + 0xdc) as *const u32), read_unaligned);
        if (v2 & 0xf00) != 0x200 {
            local_8 = 0xffff_ffff;
        }
        mech_trace::rt!("10083715", local_8)
    }
}
