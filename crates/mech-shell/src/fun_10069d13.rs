// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut unaff_EDI: i32;
    static mut unaff_ESI: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069d13() -> u8 {
    // SAFETY: the globals are the game's own; unaff_ESI points at the input
    // bytes and unaff_EDI at a structure whose field at +0x10 is read and
    // written where the original does.
    unsafe {
        let mut pbvar2: *mut u8 = unaff_ESI;
        let counter = (unaff_EDI.wrapping_add(0x10)) as *mut i32;
        if counter.read_unaligned() == 0 {
            pbvar2 = unaff_ESI.wrapping_add(1);
            counter.write_unaligned(u32::from(unaff_ESI.read()) as i32);
        }
        let bvar1 = pbvar2.read();
        counter.write_unaligned(counter.read_unaligned().wrapping_sub(1));
        bvar1
    }
}
