// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006e790(param_1: *mut core::ffi::c_void) -> u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003f8f0(param_1: i32) -> u32 {
    if param_1 != 0 {
        // SAFETY: param_1 is the address of the game's own structure; its
        // pointer fields are read and freed through the game's free routine
        // in the same order as the original.
        unsafe {
            let base = param_1 as *const u8;

            if base
                .byte_add(0x60)
                .cast::<*mut core::ffi::c_void>()
                .read_unaligned()
                .is_null()
                == false
            {
                FUN_1006e790(
                    base.byte_add(0x60)
                        .cast::<*mut core::ffi::c_void>()
                        .read_unaligned(),
                );
            }
            if base
                .byte_add(100)
                .cast::<*mut core::ffi::c_void>()
                .read_unaligned()
                .is_null()
                == false
            {
                FUN_1006e790(
                    base.byte_add(100)
                        .cast::<*mut core::ffi::c_void>()
                        .read_unaligned(),
                );
            }
            if base
                .byte_add(0x68)
                .cast::<*mut *mut core::ffi::c_void>()
                .read_unaligned()
                .is_null()
                == false
            {
                let p = base
                    .byte_add(0x68)
                    .cast::<*mut *mut core::ffi::c_void>()
                    .read_unaligned();
                let inner = p.read_unaligned();
                if inner.is_null() == false {
                    FUN_1006e790(inner);
                }
                FUN_1006e790(
                    base.byte_add(0x68)
                        .cast::<*mut core::ffi::c_void>()
                        .read_unaligned(),
                );
            }
            if base
                .byte_add(0x6c)
                .cast::<*mut *mut core::ffi::c_void>()
                .read_unaligned()
                .is_null()
                == false
            {
                let p = base
                    .byte_add(0x6c)
                    .cast::<*mut *mut core::ffi::c_void>()
                    .read_unaligned();
                let inner = p.read_unaligned();
                if inner.is_null() == false {
                    FUN_1006e790(inner);
                }
                FUN_1006e790(
                    base.byte_add(0x6c)
                        .cast::<*mut core::ffi::c_void>()
                        .read_unaligned(),
                );
            }
            if base
                .byte_add(0x70)
                .cast::<*mut core::ffi::c_void>()
                .read_unaligned()
                .is_null()
                == false
            {
                FUN_1006e790(
                    base.byte_add(0x70)
                        .cast::<*mut core::ffi::c_void>()
                        .read_unaligned(),
                );
            }
        }
    }
    0
}
