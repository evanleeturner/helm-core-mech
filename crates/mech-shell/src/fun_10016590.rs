// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1008ef60: [i8; 1];
    static mut DAT_1008ef64: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _strtok(_Str: *mut i8, _Delim: *mut i8) -> *mut i8;
    fn _toupper(_C: i32) -> i32;
    fn FUN_1004f240(str: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10016590(str: *mut i8) {
    // SAFETY: the callees and globals are the game's own; str and the tokens
    // returned by strtok point at NUL-terminated bytes read where the original reads.
    unsafe {
        let mut pc_var1 = _strtok(str, (&raw mut DAT_1008ef60).cast::<i8>());
        loop {
            if pc_var1.is_null() {
                return;
            }
            if *pc_var1 == 0x2d
                && _toupper(*pc_var1.add(1) as i32) == 0x58
                && *pc_var1.add(2) == 0x3d
            {
                let i_var2 = _toupper(*pc_var1.add(3) as i32);
                let mut arg: i32 = 0;
                let mut call = true;
                if i_var2 == 0x46 {
                    arg = 4;
                } else if i_var2 == 0x4d {
                    arg = 1;
                } else if i_var2 == 0x53 {
                    arg = 2;
                } else {
                    call = false;
                }
                if call {
                    FUN_1004f240(arg);
                }
            }
            pc_var1 = _strtok(core::ptr::null_mut(), (&raw mut DAT_1008ef64).cast::<i8>());
        }
    }
}
