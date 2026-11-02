// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100baa08: u32;
    static mut DAT_100c8dc0: [u8; 1];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10088072() -> u32;
    fn FUN_100882a5() -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10088873() {
    // SAFETY: the callees and globals are the game's own; the table entry
    // holds a pointer to an object whose slot at +0x1c is a function pointer.
    unsafe {
        FUN_10088072();
        FUN_100882a5();
        let base = (&raw mut DAT_100c8dc0).cast::<u8>();
        let entry = base.byte_add(DAT_100baa08.wrapping_mul(4) as usize);
        let obj = entry.cast::<i32>().read_unaligned();
        let slot = (obj as usize).wrapping_add(0x1c) as *const usize;
        let f: unsafe extern "C" fn() = core::mem::transmute(slot.read_unaligned());
        f();
    }
}
