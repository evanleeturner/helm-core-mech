// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100ab2b4: [i8; 0];
    static mut DAT_100ab2c8: [i8; 0];
    static mut DAT_100ab2b8: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(filename: *mut i8, mode: *mut i8) -> *mut u8;
    fn _fprintf(file: *mut u8, format: *mut i8, ...) -> i32;
    fn _fclose(file: *mut u8) -> i32;
    fn FUN_1006a38c(param_1: *mut i8) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001de5c(param_1: *mut i8) {
    // SAFETY: the callees and globals are the game's own; the file handle is
    // passed to fclose exactly as the original does, and param_1 is the caller's string.
    unsafe {
        let file = _fopen(
            (&raw mut DAT_100ab2b8).cast::<i8>(),
            (&raw mut DAT_100ab2b4).cast::<i8>(),
        );
        if !file.is_null() {
            _fprintf(file, (&raw mut DAT_100ab2c8).cast::<i8>(), param_1);
        }
        _fclose(file);
        FUN_1006a38c(param_1);
    }
}
