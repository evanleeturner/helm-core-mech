// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eeda0: [u8; 16];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1007015c(param_1: u32, param_2: u32, param_3: i32) -> u32 {
    let _ = param_1;
    let _ = param_2;
    // SAFETY: the global is the game's own 16-byte array; param_3 is a
    // caller-supplied address of at least 16 writable bytes, written only
    // when non-null and the flag bit is clear.
    unsafe {
        let base = (&raw mut DAT_100eeda0).cast::<u8>();
        if (*base.add(15) & 1) == 0 && param_3 != 0 {
            let mut local_8: i32 = 0;
            while local_8 < 4 {
                let v = base
                    .byte_add(local_8.wrapping_mul(4) as usize)
                    .cast::<u32>()
                    .read_unaligned();
                ((param_3.wrapping_add(local_8.wrapping_mul(4))) as *mut u32).write_unaligned(v);
                local_8 = local_8.wrapping_add(1);
            }
        }
    }
    0
}
