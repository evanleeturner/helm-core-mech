// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [u32; 1];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10064350() {
    let mut local_c: i32 = 0;
    // SAFETY: the globals are the game's own memory; the table entries are
    // pointers to live objects whose fields at 0x20 and 0x28 are read and the
    // callback at 0x28 is invoked as the original does.
    unsafe {
        while local_c < DAT_100b38d8 {
            let base = (&raw mut DAT_100eef40).cast::<u32>();
            let obj = base.offset(local_c as isize).read_unaligned();
            let p28 = (obj.wrapping_add(0x28)) as *const i32;
            if p28.read_unaligned() != 0 {
                let obj = base.offset(local_c as isize).read_unaligned();
                let p20 = (obj.wrapping_add(0x20)) as *const i32;
                if p20.read_unaligned() != 0 {
                    let obj = base.offset(local_c as isize).read_unaligned();
                    let obj2 = base.offset(local_c as isize).read_unaligned();
                    let fp = ((obj.wrapping_add(0x28)) as *const u32).read_unaligned();
                    let f: unsafe extern "C" fn(u32) = core::mem::transmute(fp as usize);
                    f(obj2);
                }
            }
            local_c = local_c.wrapping_add(1);
        }
    }
}
