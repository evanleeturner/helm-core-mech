// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10064438() {
    let mut local_c: i32 = 0;
    // SAFETY: the globals are the game's own memory; the table entries are
    // addresses of game structs whose fields at +0x20 and +0x30 are read as
    // the original does, and the +0x30 field is a cdecl callback.
    unsafe {
        while local_c < DAT_100b38d8 {
            let tbl = (&raw mut DAT_100eef40).cast::<u32>();
            let e1 = tbl.add(local_c as usize).read();
            if (e1.wrapping_add(0x30) as *const i32).read_unaligned() != 0 {
                let e2 = tbl.add(local_c as usize).read();
                if (e2.wrapping_add(0x20) as *const i32).read_unaligned() != 0 {
                    let e3 = tbl.add(local_c as usize).read();
                    let f = (e3.wrapping_add(0x30) as *const u32).read_unaligned();
                    let e4 = tbl.add(local_c as usize).read();
                    let arg = (e4.wrapping_add(0x20) as *const u32).read_unaligned();
                    let func = core::mem::transmute::<u32, unsafe extern "C" fn(u32)>(f);
                    func(arg);
                }
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
