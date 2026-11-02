// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7248: u32;
    static mut DAT_100b0b38: [u8; 0x300];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10012270() {
    mech_trace::enter!("10012270");
    // SAFETY: the globals are the game's own memory; the table is 0xc0 dwords
    // and DAT_100a7248 holds an address whose +0x14 slot is a code pointer.
    unsafe {
        let base = (&raw mut DAT_100b0b38).cast::<u32>();
        let mut table_cursor = base;
        let mut dwords_left: i32 = 0xc0;
        while dwords_left != 0 {
            mech_trace::wr!("10012270", table_cursor, write_unaligned, 0);
            table_cursor = table_cursor.add(1);
            dwords_left = dwords_left.wrapping_sub(1);
        }
        let f = mech_trace::rd!(
            "10012270",
            ((mech_trace::gr!("10012270", "100a7248", DAT_100a7248) as usize).wrapping_add(0x14)
                as *const usize),
            read_unaligned
        );
        let func: unsafe extern "C" fn(i32, i32, *mut u8, i32) =
            core::mem::transmute::<usize, unsafe extern "C" fn(i32, i32, *mut u8, i32)>(f);
        mech_trace::cr!(
            "10012270",
            "ind",
            func(0, 0x100, (&raw mut DAT_100b0b38).cast::<u8>(), 1)
        );
    }
}
