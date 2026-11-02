// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10093c70: [i8; 1];
    static mut DAT_10093c68: [i8; 1];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *mut i8, ...) -> i32;
    fn FID_conflict___wfopen(filename: *mut i8, mode: *mut i8) -> *mut u8;
    fn _fclose(file: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_10036340(param_1: u32) -> u32 {
    let mut local_20: [i8; 32] = [0; 32];
    // SAFETY: the callees and globals are the game's own; local_20 is a
    // local buffer passed as the original does.
    unsafe {
        _sprintf(
            local_20.as_mut_ptr(),
            (&raw mut DAT_10093c68).cast::<i8>(),
            param_1,
        );
        let file =
            FID_conflict___wfopen(local_20.as_mut_ptr(), (&raw mut DAT_10093c70).cast::<i8>());
        if !file.is_null() {
            _fclose(file);
            return 0xfffffffe;
        }
        0xffffffff
    }
}
