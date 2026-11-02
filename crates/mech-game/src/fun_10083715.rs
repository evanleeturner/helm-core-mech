// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10083715() -> u32 {
    // SAFETY: the globals are the game's own memory; the table entry is a
    // pointer to a record whose field at 0xdc is read, as in the original.
    unsafe {
        let base = (&raw mut DAT_100eef40).cast::<u32>();
        let rec = base.add(DAT_100adf38 as usize).read() as usize;
        let v = ((rec + 0xdc) as *const u32).read_unaligned();
        let mut local_8 = v & 0xff;
        let v2 = ((rec + 0xdc) as *const u32).read_unaligned();
        if (v2 & 0xf00) != 0x200 {
            local_8 = 0xffff_ffff;
        }
        local_8
    }
}
