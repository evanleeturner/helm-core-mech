// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10064ea3(param_1: u32) -> u32;
    fn FUN_10064e6a(param_1: u8) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10064d61(param_1: *mut u32) -> u32 {
    // SAFETY: param_1 points at the caller's record; the reads happen in the
    // original's order and the callees are the game's own.
    unsafe {
        if FUN_10064ea3(param_1.read_unaligned()) != 0 {
            return 2;
        }
        if FUN_10064ea3(param_1.add(1).read_unaligned()) != 0 {
            return 2;
        }
        let b = param_1.add(2).read_unaligned() as u8 as u32;
        let v = (((b >> 3) & 1) << 3) | (b & 0x80) | (((b >> 4) & 7) << 4) | (b & 7);
        if FUN_10064e6a(v as u8) != 0 {
            return 2;
        }
        if FUN_10064e6a(param_1.byte_add(9).cast::<u8>().read()) != 0 {
            return 2;
        }
        if FUN_10064e6a(param_1.byte_add(10).cast::<u8>().read()) != 0 {
            return 2;
        }
        0
    }
}
