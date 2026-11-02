// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100cded4: [u8; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1007bdc0() -> bool {
    let mut local_8: i32 = 0;
    // SAFETY: DAT_100cded4 is the game's own table of 0x80 entries of 0x24
    // bytes each; only its first dword per entry is read.
    unsafe {
        let base = (&raw mut DAT_100cded4).cast::<u8>();
        while local_8 < 0x80
            && base
                .byte_add((local_8.wrapping_mul(0x24)) as u32 as usize)
                .cast::<i32>()
                .read_unaligned()
                != 0
        {
            local_8 = local_8.wrapping_add(1);
        }
    }
    local_8 == 0x80
}
