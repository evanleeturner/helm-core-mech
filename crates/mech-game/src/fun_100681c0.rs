// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100eeef0: u32;
    static mut DAT_100eeef4: u32;
    static mut DAT_100eeef8: u32;
    static mut DAT_100eeefc: u32;
    static mut DAT_100eef00: u32;
    static mut DAT_100eef04: u32;
    static mut DAT_100eef08: u32;
    static mut DAT_10114db8: u32;
    static mut DAT_10114df4: u32;
    static mut DAT_10114e08: u32;
    static mut DAT_10114e18: u32;
    static mut DAT_10114e2c: u32;
    static mut DAT_10114e30: u32;
    static mut DAT_10114e34: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100681c0() {
    // SAFETY: the globals are the game's own; each pointer global holds an
    // address of a structure whose field at offset 0x30 is read.
    unsafe {
        DAT_100eeef4 = ((DAT_10114df4.wrapping_add(0x30)) as *const u32).read_unaligned();
        DAT_100eeef0 = ((DAT_10114db8.wrapping_add(0x30)) as *const u32).read_unaligned();
        DAT_100eeef8 = ((DAT_10114e08.wrapping_add(0x30)) as *const u32).read_unaligned();
        DAT_100eeefc = ((DAT_10114e18.wrapping_add(0x30)) as *const u32).read_unaligned();
        DAT_100eef00 = ((DAT_10114e2c.wrapping_add(0x30)) as *const u32).read_unaligned();
        DAT_100eef04 = ((DAT_10114e30.wrapping_add(0x30)) as *const u32).read_unaligned();
        DAT_100eef08 = ((DAT_10114e34.wrapping_add(0x30)) as *const u32).read_unaligned();
    }
}
