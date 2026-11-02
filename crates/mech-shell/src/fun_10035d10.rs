// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10092000: i32;
    static mut DAT_10092004: i32;
    static mut DAT_10091ffc: i32;
    static mut DAT_100a6798: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10030160(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10035d10() {
    // SAFETY: the callee and the globals are the game's own; DAT_100a6798
    // holds an address of a structure whose field at offset 4 is read.
    unsafe {
        let u_var1 = FUN_10030160(DAT_10092000, DAT_10092004);
        if (u_var1 as u8) as i8 == 1 {
            DAT_10091ffc = DAT_10091ffc.wrapping_add(1);
            let limit = ((DAT_100a6798 as u32).wrapping_add(4) as *const i32).read_unaligned();
            if limit <= DAT_10091ffc {
                DAT_10091ffc = 0;
            }
        }
    }
}
