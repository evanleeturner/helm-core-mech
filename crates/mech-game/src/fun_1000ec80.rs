// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b9a1c: *mut u8;
    static mut DAT_101f52d0: i32;
    static mut DAT_100adf80: i32;
    static mut DAT_100adcd0: i32;
    static mut DAT_100a716c: i32;
    static mut DAT_101f52e0: [i32; 5];
    static mut DAT_100c4f38: [i32; 5];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1005bfc9();
    fn FUN_100698d0();
    fn FUN_100605d6();
    fn FUN_10060517();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000ec80() {
    // SAFETY: the callees and globals are the game's own memory; the table
    // pointer in DAT_100b9a1c holds function pointers at the slots read, and
    // the 5-dword blocks are copied within their bounds.
    unsafe {
        FUN_1005bfc9();
        FUN_100698d0();
        FUN_100605d6();
        FUN_10060517();

        if DAT_101f52d0 != 0 {
            let base = &raw mut DAT_101f52e0 as *mut i32;
            let slot = DAT_100b9a1c
                .byte_add(0x20)
                .cast::<unsafe extern "C" fn(i32, i32, i32, i32)>()
                .read_unaligned();
            slot(
                base.add(1).read_unaligned().wrapping_add(1),
                base.add(2).read_unaligned().wrapping_add(1),
                base.add(3).read_unaligned(),
                base.add(4).read_unaligned(),
            );

            let mut src = &raw mut DAT_100c4f38 as *const i32;
            let mut dst = base;
            let mut i = 5i32;
            while i != 0 {
                dst.write_unaligned(src.read_unaligned());
                src = src.add(1);
                dst = dst.add(1);
                i = i.wrapping_sub(1);
            }
            DAT_100adcd0 = DAT_100a716c;
            DAT_101f52d0 = 0;
        } else if DAT_100adf80 != 0 {
            let slot = DAT_100b9a1c
                .byte_add(0x18)
                .cast::<unsafe extern "C" fn()>()
                .read_unaligned();
            slot();
        }
    }
}
