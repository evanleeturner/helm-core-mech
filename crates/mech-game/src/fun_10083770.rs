// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf38: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10083770() -> u32 {
    // SAFETY: the globals are the game's own memory; the table entry is a
    // pointer to a structure whose field at offset 0xdc is read.
    unsafe {
        let base = (&raw mut DAT_100eef40).cast::<u32>();
        let ptr = base.add(DAT_100adf38 as usize).read() as usize;
        let mut local_8 = ((ptr + 0xdc) as *const u32).read_unaligned() & 0xff;
        if (((ptr + 0xdc) as *const u32).read_unaligned() & 0xf00) != 0x400 {
            local_8 = 0xffffffff;
        }
        local_8
    }
}
