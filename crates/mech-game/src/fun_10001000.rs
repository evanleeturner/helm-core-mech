// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a4030: u32;
    static mut DAT_100a4034: [i8; 0];
    static mut DAT_100a4040: [i8; 0];
    static mut s_MW2_INI_100a4038: [i8; 0];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _fopen(filename: *mut i8, mode: *mut i8) -> *mut u8;
    fn FUN_100012e5(param_1: *mut i8) -> *mut i8;
    fn _fgets(buf: *mut i8, max_count: i32, file: *mut u8) -> *mut i8;
    fn _fclose(file: *mut u8) -> i32;
    fn _strspn(s: *mut i8, control: *mut i8) -> u32;
    fn _strchr(s: *mut i8, val: i32) -> *mut i8;
    fn __strcmpi(a: *mut i8, b: *mut i8) -> i32;
    fn _ftell(file: *mut u8) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10001000(param_1: *mut i8) -> u32 {
    let mut local_8c: [i8; 136] = [0; 136];
    // SAFETY: the callees and globals are the game's own; local_8c is a
    // local buffer of the size the original uses, and indices come from strspn on it.
    unsafe {
        DAT_100a4030 = 0xffff_ffff;
        let file = _fopen(
            (&raw mut s_MW2_INI_100a4038).cast::<i8>(),
            (&raw mut DAT_100a4034).cast::<i8>(),
        );
        if file.is_null() {
            return 0x41;
        }
        let pc_var2 = FUN_100012e5(param_1);
        let buf = local_8c.as_mut_ptr();
        loop {
            let pc_var3 = _fgets(buf, 0x85, file);
            if pc_var3.is_null() {
                _fclose(file);
                return 0x4a;
            }
            let s_var4 = _strspn(buf, (&raw mut DAT_100a4040).cast::<i8>()) as usize;
            if *buf.add(s_var4) != b'[' as i8 {
                continue;
            }
            let p = _strchr(buf.add(s_var4 + 1), 0x5d);
            if p.is_null() {
                continue;
            }
            *p = 0;
            let str1 = FUN_100012e5(buf.add(s_var4 + 1));
            if __strcmpi(str1, pc_var2) == 0 {
                break;
            }
        }
        DAT_100a4030 = _ftell(file) as u32;
        _fclose(file);
        0
    }
}
