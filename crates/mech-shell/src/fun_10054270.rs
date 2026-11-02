// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_1009c698: i32;
    static mut DAT_1009c690: u32;
    static mut DAT_1009c68c: i32;
    static mut DAT_1009fecc: i8;
    static mut DAT_1009feb4: i8;
    static mut DAT_100bd950: i8;
    static mut DAT_100bfed8: i8;
    static mut DAT_100bdf90: i8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dst: *mut i8, fmt: *const i8, ...) -> i32;
    fn FID_conflict___wfopen(
        filename: *const core::ffi::c_void,
        mode: *const core::ffi::c_void,
    ) -> *mut core::ffi::c_void;
    fn _fread(
        ptr: *mut core::ffi::c_void,
        size: u32,
        count: u32,
        stream: *mut core::ffi::c_void,
    ) -> u32;
    fn _fclose(stream: *mut core::ffi::c_void) -> i32;
    fn FUN_10053df0(param_1: *mut core::ffi::c_void);
    fn FUN_10053cf0();
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10054270(param_1: i32) {
    let mut file_name = [0i8; 32];
    let mut file_number: i32;
    let saved_config_number: i32;

    // SAFETY: globals and callees are the game's own; param_1, when nonzero,
    // points at a structure whose field at +0x28 is read.
    unsafe {
        if param_1 == 0 {
            file_number = 0;
            saved_config_number = DAT_1009c698;
        } else {
            file_number = ((param_1 as u32).wrapping_add(0x28) as *const i32).read_unaligned();
            let mut s = file_number;
            if file_number < 0 {
                file_number = DAT_1009c698;
                s = DAT_1009c698;
            }
            saved_config_number = s;
        }
        DAT_1009c698 = saved_config_number;
        _sprintf(file_name.as_mut_ptr(), &raw const DAT_1009feb4, file_number);
        let cpc_file = FID_conflict___wfopen(
            file_name.as_ptr().cast(),
            (&raw mut DAT_1009fecc).cast::<core::ffi::c_void>(),
        );
        if !cpc_file.is_null() {
            _fread((&raw mut DAT_100bd950).cast(), 0x40, 1, cpc_file);
            _fread((&raw mut DAT_100bfed8).cast(), 0x10, 0x10, cpc_file);
            _fread((&raw mut DAT_100bdf90).cast(), 0xde0, 1, cpc_file);
            _fclose(cpc_file);
            FUN_10053df0((&raw mut DAT_100bdf90).cast());
            FUN_10053cf0();
            DAT_1009c68c = 0;
            DAT_1009c690 = (&raw mut DAT_100bdf90) as u32;
        }
    }
}
