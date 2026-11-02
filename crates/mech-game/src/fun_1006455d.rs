// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1006455d() {
    let mut local_c: i32 = 0;
    // SAFETY: the globals are the game's own memory; each table entry is a
    // pointer to an object whose fields at 0x3c and 0x20 are read where the
    // original reads them, and the callback is the game's own function.
    unsafe {
        while local_c < DAT_100b38d8 {
            let base = (&raw mut DAT_100eef40).cast::<u32>();
            let obj = base.offset(local_c as isize).read_unaligned();
            if ((obj.wrapping_add(0x3c)) as *const i32).read_unaligned() != 0 {
                let obj2 = base.offset(local_c as isize).read_unaligned();
                let fp = ((obj2.wrapping_add(0x3c)) as *const u32).read_unaligned();
                let obj3 = base.offset(local_c as isize).read_unaligned();
                let arg = ((obj3.wrapping_add(0x20)) as *const u32).read_unaligned();
                let f: extern "C" fn(u32) = core::mem::transmute::<u32, extern "C" fn(u32)>(fp);
                f(arg);
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
