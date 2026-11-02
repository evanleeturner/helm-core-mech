// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aa1b8: i16;
    static mut DAT_100aa1f0: i32;
    static mut DAT_100aa1f4: i32;
    static mut DAT_100aa208: *mut i32;
    static mut DAT_100aa20c: i32;
    static mut DAT_100aa4a4: u16;
    static mut DAT_100aa4a6: u8;
    static mut DAT_100b3908: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10006e00(param_1: *mut core::ffi::c_void, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10013faa() -> i32 {
    // SAFETY: the globals and callee are the game's own memory and code; the
    // object in DAT_100aa208 carries a vtable whose slots +0x10 and +0x20 hold
    // stdcall function pointers, called as the original does.
    unsafe {
        let mut local_8: [u8; 3] = [0; 3];
        let s_tmp: u16 = DAT_100aa4a4;
        local_8.as_mut_ptr().cast::<u16>().write_unaligned(s_tmp);
        local_8[2] = DAT_100aa4a6;

        let mut i_var1: i32 = DAT_100aa1b8 as i32;
        if i_var1 == 0 {
            if !DAT_100aa208.is_null() && DAT_100aa20c != 0 {
                if DAT_100aa1f4 != 0 {
                    FUN_10006e00(local_8.as_mut_ptr().cast(), 2);
                }
                let vt = DAT_100aa208.read_unaligned() as u32;
                let fp = ((vt.wrapping_add(0x20)) as *const usize).read_unaligned();
                let f: unsafe extern "stdcall" fn(*mut i32, i32) -> i32 = core::mem::transmute(fp);
                i_var1 = f(DAT_100aa208, DAT_100aa20c);
                DAT_100aa20c = 0;
            }
            if !DAT_100aa208.is_null() {
                i_var1 = DAT_100aa1b8 as i32;
                if i_var1 == 0 {
                    let vt = DAT_100aa208.read_unaligned() as u32;
                    let fp = ((vt.wrapping_add(0x10)) as *const usize).read_unaligned();
                    let f: unsafe extern "stdcall" fn(*mut i32) -> i32 = core::mem::transmute(fp);
                    i_var1 = f(DAT_100aa208);
                    DAT_100aa208 = core::ptr::null_mut();
                }
            }
        } else if !DAT_100aa208.is_null() && DAT_100aa20c != 0 && DAT_100aa1f4 != 0 {
            i_var1 = FUN_10006e00(local_8.as_mut_ptr().cast(), 2);
        }

        if DAT_100aa1f0 == 1 {
            DAT_100aa1f0 = 2;
            DAT_100aa1f4 = 0;
            DAT_100b3908 = 0;
        } else {
            i_var1 = 1;
        }
        i_var1
    }
}
