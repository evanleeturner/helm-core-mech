// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b0574: *mut u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1004718e(param_1: *mut u8) -> *mut u32 {
    // SAFETY: the global and the list nodes are the game's own memory; param_1
    // points at the caller's record. Reads happen in the original's order.
    unsafe {
        let mut local_c: *mut u32 = DAT_100b0574;
        let mut local_8: *mut u32 = core::ptr::null_mut();
        loop {
            if local_c.is_null() || !local_8.is_null() {
                return local_8;
            }
            let tag = local_c.add(1).cast::<i16>().read_unaligned();
            if tag == -2 || tag == -1 {
                let mut pbvar4: *const u8 = (local_c as *const u8).byte_add(6);
                let mut pbvar3: *const u8 = param_1;
                let mut equal = true;
                loop {
                    let b1 = pbvar4.read();
                    let b2 = pbvar3.byte_add(2).read();
                    if b1 != b2 {
                        equal = false;
                        break;
                    }
                    if b1 == 0 {
                        break;
                    }
                    let b1 = pbvar4.byte_add(1).read();
                    let b3 = pbvar3.byte_add(3).read();
                    if b1 != b3 {
                        equal = false;
                        break;
                    }
                    pbvar4 = pbvar4.byte_add(2);
                    pbvar3 = pbvar3.byte_add(2);
                    if b1 == 0 {
                        break;
                    }
                }
                if equal {
                    local_8 = local_c.add(1);
                }
            } else if tag == param_1.cast::<i16>().read_unaligned() {
                local_8 = local_c.add(1);
            }
            local_c = local_c.read_unaligned() as *mut u32;
        }
    }
}
