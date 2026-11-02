// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eee70: [u8; 0x50];
    static mut DAT_100eeec0: u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10069ad8() {
    // SAFETY: the globals are the game's own memory; the buffer holds 0x50
    // bytes and every index written is below that.
    unsafe {
        let p = (&raw mut DAT_100eee70).cast::<u8>();
        let mut i: i32 = 0;
        while i < 0x50 {
            p.byte_add(i as usize).write(0x20);
            i = i.wrapping_add(1);
        }
        DAT_100eeec0 = 0;
    }
}
