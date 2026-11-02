// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a7160: u32;
    static mut DAT_100a7b30: [u8; 0];
    static mut DAT_100adcd0: u32;
    static mut DAT_100b38fc: u32;
    static mut DAT_100b3904: u32;
    static mut DAT_100c4f30: u32;
    static mut DAT_100c4f34: u32;
    static mut DAT_10215658: u32;
    static mut DAT_10215668: u32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_100498c5(param_1: i32) -> i32;
    fn FUN_10051f43(param_1: i32) -> i32;
    fn FUN_10003d80(param_1: *mut i32) -> u8;
    fn FUN_1000a3fd(param_1: i32, param_2: *mut u32, param_3: *mut i32, param_4: i32) -> u32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_1000898c() -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000a5b7() {
    // SAFETY: the globals and callees are the game's own; the pointers read
    // from the game's tables are dereferenced exactly where the original does.
    unsafe {
        let base = (&raw mut DAT_100a7b30).cast::<u8>();
        let puvar1 = base
            .byte_add(DAT_10215658.wrapping_mul(4) as usize)
            .cast::<*mut u32>()
            .read_unaligned();
        if puvar1.is_null() {
            return;
        }
        let pi_var2 = puvar1.byte_add(20).cast::<*mut i32>().read_unaligned();
        if pi_var2.is_null() {
            return;
        }
        let mut i_var3 = FUN_100498c5(4);
        if i_var3 != 0 || DAT_10215668 != 0 {
            DAT_10215668 = 0;
            FUN_1000898c();
            return;
        }
        let mut skip_to_end = false;
        if DAT_100a7160 == 0 {
            let p = pi_var2.byte_add(4).read_unaligned() as *const i32;
            i_var3 = FUN_10051f43(p.read_unaligned());
            DAT_100c4f30 = DAT_100b38fc.wrapping_add(i_var3 as u32);
            DAT_100a7160 = 1;
        } else if DAT_100a7160 != 1 {
            if DAT_100a7160 != 2 {
                return;
            }
            skip_to_end = true;
        }
        if !skip_to_end {
            if DAT_100b38fc < DAT_100c4f30 {
                FUN_1000898c();
                return;
            }
            DAT_100a7160 = 2;
            let p = pi_var2.byte_add(4).read_unaligned() as *const i32;
            i_var3 = FUN_10051f43(p.read_unaligned());
            DAT_100c4f34 = DAT_100b38fc.wrapping_add(i_var3 as u32);
            FUN_10003d80(pi_var2);
        }
        DAT_100adcd0 = 0;
        let p = pi_var2.byte_add(4).read_unaligned() as *const i32;
        let arg = p
            .read_unaligned()
            .wrapping_sub(DAT_100b3904 as i32)
            .wrapping_add(-1);
        i_var3 = FUN_10051f43(arg);
        let q = pi_var2.read_unaligned() as *mut u8;
        q.byte_add(8).cast::<i32>().write_unaligned(i_var3);
        FUN_1000a3fd(1, puvar1, pi_var2, 1);
        if DAT_100c4f34 < DAT_100b38fc {
            DAT_100a7160 = 0;
        }
    }
}
