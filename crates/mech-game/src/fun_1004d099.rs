// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: u32;
    static mut DAT_100eef40: [u32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004d099(param_1: u32, param_2: u32) -> u32 {
    let mut local_8: u32 = 0;
    // SAFETY: the globals are the game's own memory; the table entry is a
    // pointer-sized address of a record whose field at offset 0xc is written.
    unsafe {
        if param_1 < DAT_100b38d8 && param_2 < 8 {
            let base = (&raw mut DAT_100eef40).cast::<u32>();
            let rec = base.add(param_1 as usize).read_unaligned();
            (rec as usize as *mut u8)
                .byte_add(0xc)
                .cast::<u32>()
                .write_unaligned(param_2);
            local_8 = 1;
        }
    }
    local_8
}
