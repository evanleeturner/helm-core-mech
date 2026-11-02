// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008e954: [i8; 1];
    static mut DAT_1008e958: [i8; 1];
    static mut DAT_100b07f0: u32;
    static mut DAT_100b07f8: u8;
    static mut DAT_100b0a30: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FID_conflict___wfopen(_Filename: *mut i8, _Mode: *mut i8) -> *mut u8;
    fn _fread(_DstBuf: *mut u8, _ElementSize: u32, _Count: u32, _File: *mut u8) -> u32;
    fn _fclose(_File: *mut u8) -> i32;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn FUN_10035ef0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100110d0(param_1: *mut i32, param_2: *mut u8, param_3: *mut u32) {
    mech_trace::enter!("100110d0", param_1, param_2, param_3);
    // SAFETY: the globals and callees are the game's own; the parameters point
    // at the caller's storage, accessed where the original does.
    unsafe {
        let file = mech_trace::cr!(
            "100110d0",
            "FID_conflict___wfopen",
            FID_conflict___wfopen(
                (&raw mut DAT_1008e958).cast::<i8>(),
                (&raw mut DAT_1008e954).cast::<i8>(),
            )
        );
        if !file.is_null() {
            let n = mech_trace::cr!(
                "100110d0",
                "_fread",
                _fread((&raw mut DAT_100b07f0).cast::<u8>(), 0x344, 1, file)
            );
            if n != 1 {
                mech_trace::cr!("100110d0", "_fclose", _fclose(file));
                return;
            }
            mech_trace::cr!("100110d0", "_fclose", _fclose(file));
            if mech_trace::rd!("100110d0", param_1, read_unaligned) != 0 {
                mech_trace::wr!(
                    "100110d0",
                    param_1,
                    write_unaligned,
                    mech_trace::gr!("100110d0", "100b07f0", DAT_100b07f0) as i32
                );
                mech_trace::wr!(
                    "100110d0",
                    param_2,
                    write,
                    mech_trace::gr!("100110d0", "100b07f8", DAT_100b07f8)
                );
                mech_trace::wr!(
                    "100110d0",
                    param_3,
                    write_unaligned,
                    (&raw mut DAT_100b0a30) as u32
                );
            }
            let base = (&raw mut DAT_100b0a30).cast::<u8>();
            let mut i_var4: i32 = 0;
            let mut c_var1 = mech_trace::gr!("100110d0", "100b0a30", DAT_100b0a30) as i8;
            while 0x20 < c_var1 {
                let i_var2 = i_var4.wrapping_add(1);
                i_var4 = i_var4.wrapping_add(1);
                c_var1 = mech_trace::rd!("100110d0", base.byte_offset(i_var2 as isize), read) as i8;
            }
            mech_trace::wr!("100110d0", base.byte_offset(i_var4 as isize), write, 0);
            FUN_10035ef0();
        }
    }
}
