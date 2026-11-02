// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1ac: u32;
    static mut DAT_100adf34: u32;
    static mut DAT_10114db0: [u32; 0x22];
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000a942();
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100105ba(param_1: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1005d5c1() {
    // SAFETY: the globals, the callees and the object table are the game's own
    // memory; each object pointer in the table addresses a live object whose
    // slot at offset 0xc0 holds either zero or a callback taking the object.
    unsafe {
        if (DAT_100adf34 as *const u8).byte_add(9).read() != 0 || DAT_100aa1ac == 0 {
            FUN_1000a942();
        }
        let mut local_8: i32 = 0;
        while local_8 < 0x22 {
            let obj = DAT_10114db0[local_8 as usize];
            if ((obj as *const u8)
                .byte_add(0xc0)
                .cast::<i32>()
                .read_unaligned())
                != 0
            {
                let obj2 = DAT_10114db0[local_8 as usize];
                let fp = (obj2 as *const u8)
                    .byte_add(0xc0)
                    .cast::<u32>()
                    .read_unaligned();
                let f: unsafe extern "C" fn(u32) = core::mem::transmute(fp as usize);
                f(DAT_10114db0[local_8 as usize]);
            }
            local_8 = local_8.wrapping_add(1);
        }
        FUN_100105ba(-1);
    }
}
