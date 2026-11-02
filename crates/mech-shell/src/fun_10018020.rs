// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _qsort(
        base: *mut core::ffi::c_void,
        num: usize,
        width: usize,
        compare: Option<
            unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void) -> i32,
        >,
    );
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10018020(param_1: i32) {
    if param_1 != 0 {
        // SAFETY: param_1 is the address of the game's entry list; fields are
        // read and written at the offsets the original uses.
        unsafe {
            let p = param_1 as usize as *mut u8;
            if (p.byte_add(0xc).read() & 2) != 0 {
                let pending = p.byte_add(0xd).cast::<i32>().read_unaligned();
                if pending == 1 {
                    let array = p
                        .byte_add(0x11)
                        .cast::<*mut core::ffi::c_void>()
                        .read_unaligned();
                    let compare = p
                        .byte_add(0x19)
                        .cast::<Option<
                            unsafe extern "C" fn(
                                *const core::ffi::c_void,
                                *const core::ffi::c_void,
                            ) -> i32,
                        >>()
                        .read_unaligned();
                    let num = p.byte_add(4).cast::<u32>().read_unaligned() as usize;
                    _qsort(array, num, 4, compare);
                    p.byte_add(0xd).cast::<i32>().write_unaligned(0);
                }
            }
        }
    }
}
