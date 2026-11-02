// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut unaff_ESI: *mut u8;
    static mut unaff_EDI: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005b173() -> u8 {
    // SAFETY: the globals stand in for the caller's ESI and EDI registers;
    // they point at the caller's buffer and state block, read and written
    // where the original does.
    unsafe {
        let mut pb_var2: *mut u8 = unaff_ESI;
        let counter = unaff_EDI.byte_add(0x10).cast::<i32>();
        if counter.read_unaligned() == 0 {
            pb_var2 = unaff_ESI.add(1);
            counter
                .cast::<u32>()
                .write_unaligned(u32::from(unaff_ESI.read()));
        }
        let b_var1 = pb_var2.read();
        counter.write_unaligned(counter.read_unaligned().wrapping_add(-1));
        b_var1
    }
}
