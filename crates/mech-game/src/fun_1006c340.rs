// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1006c340(flags: u32) -> u32 {
    let mut flags = flags;
    if (flags & 0x730) != 0 {
        flags |= 0x730;
    }
    if (flags & 3) != 0 {
        flags |= 3;
    }
    flags
}
