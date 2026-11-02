// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6748: *mut c_void;
    static mut DAT_100b5a98: u8;
}

#[allow(non_snake_case)]
unsafe extern "fastcall" {
    fn FUN_10040680(param_1: i32);
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006ec30(param_1: *mut c_void);
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10050b80(
        this: *mut c_void,
        param_1: i32,
        param_2: i32,
        param_3: *mut i8,
        param_4: i32,
    ) -> *mut i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1003a460() {
    mech_trace::enter!("1003a460");
    // SAFETY: the globals and callees are the game's own; the table at
    // DAT_100b5a98 is walked in 0x211-byte records exactly as the original does.
    unsafe {
        let mut pc_var5: *mut i8 = (&raw mut DAT_100b5a98).cast::<i8>();
        let mut i_var6: i32 = 0x5c;
        loop {
            let mut i_var4: i32 = -1;
            let mut pc_var7: *mut i8 = pc_var5;
            loop {
                if i_var4 == 0 {
                    break;
                }
                i_var4 = i_var4.wrapping_sub(1);
                let c_var1 = mech_trace::rd!("1003a460", pc_var7, read);
                pc_var7 = pc_var7.wrapping_add(1);
                if c_var1 == 0 {
                    break;
                }
            }
            if i_var4 != -2 {
                let pv_var2: *mut c_void = mech_trace::rd!(
                    "1003a460",
                    pc_var5.byte_add(0x209).cast::<*mut c_void>(),
                    read_unaligned
                );
                if !pv_var2.is_null() {
                    FUN_10040680(pv_var2 as usize as i32);
                    FUN_1006ec30(pv_var2);
                }
                let pi_var3 = mech_trace::cr!(
                    "1003a460",
                    "10050b80",
                    FUN_10050b80(
                        mech_trace::gr!("1003a460", "100a6748", DAT_100a6748),
                        0x2a,
                        i_var6,
                        pc_var5,
                        0x100c49d0
                    )
                );
                mech_trace::wr!(
                    "1003a460",
                    pc_var5.byte_add(0x209).cast::<*mut i32>(),
                    write_unaligned,
                    pi_var3
                );
            }
            pc_var5 = pc_var5.wrapping_byte_add(0x211);
            i_var6 = i_var6.wrapping_add(0x23);
            if 0x100b6f41usize < pc_var5 as usize {
                return;
            }
        }
    }
}
