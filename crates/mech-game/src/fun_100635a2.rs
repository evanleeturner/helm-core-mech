// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10062fc4(param_1: u32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100635a2() {
    mech_trace::enter!("100635a2");
    // SAFETY: the globals and the callee are the game's own; the table entry
    // read is a pointer to a structure whose field at offset 0x20 is read.
    unsafe {
        let idx = mech_trace::gr!("100635a2", "100adf38", DAT_100adf38) as usize;
        let base = (&raw mut DAT_100eef40).cast::<u32>();
        let entry = mech_trace::rd!("100635a2", base.add(idx), read_unaligned);
        let val = mech_trace::rd!(
            "100635a2",
            ((entry as usize + 0x20) as *const u32),
            read_unaligned
        );
        FUN_10062fc4(val);
    }
}
