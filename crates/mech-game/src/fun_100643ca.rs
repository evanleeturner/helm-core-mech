// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100643ca() {
    let mut local_10: i32 = 0;
    // SAFETY: the globals are the game's own; each table entry is a pointer to
    // a game object, whose fields at +0x20 and +0x2c are read where the
    // original reads them.
    unsafe {
        while local_10 < DAT_100b38d8 {
            let tbl = (&raw mut DAT_100eef40).cast::<u32>();
            let obj = tbl.add(local_10 as usize).read() as *const u8;
            let f = obj
                .byte_add(0x2c)
                .cast::<Option<unsafe extern "C" fn(i32)>>()
                .read_unaligned();
            if f.is_some() && obj.byte_add(0x20).cast::<i32>().read_unaligned() != 0 {
                let obj2 = tbl.add(local_10 as usize).read() as *const u8;
                let f2 = obj2
                    .byte_add(0x2c)
                    .cast::<Option<unsafe extern "C" fn(i32)>>()
                    .read_unaligned();
                let obj3 = tbl.add(local_10 as usize).read() as *const u8;
                let arg = obj3.byte_add(0x20).cast::<i32>().read_unaligned();
                if let Some(func) = f2 {
                    func(arg);
                }
            }
            local_10 = local_10.wrapping_add(1);
        }
    }
}
