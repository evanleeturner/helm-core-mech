// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn HeapAlloc(h: *mut core::ffi::c_void, flags: u32, size: u32) -> *mut core::ffi::c_void;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001c299(param_1: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c303(
    param_1: i32,
    param_2: i32,
    param_3: i32,
    param_4: i32,
    param_5: i32,
    param_6: *mut u32,
) -> *mut i32 {
    if param_1 == 0 {
        return core::ptr::null_mut();
    }
    // SAFETY: the allocator, callee and globals are the game's own; param_1 and
    // param_6 point at caller memory, and the allocation is sized for all writes.
    unsafe {
        let mut u_var2: u32 = (param_3 as u32)
            .wrapping_mul(0x44)
            .wrapping_add((param_4 as u32).wrapping_mul(0x24))
            .wrapping_add(param_5 as u32)
            .wrapping_add(0x18);
        let pi_var1 = HeapAlloc(DAT_100adf74, 1, u_var2) as *mut i32;
        if pi_var1.is_null() {
            return core::ptr::null_mut();
        }
        let mut pi_var4 = pi_var1;
        let mut u_var3 = u_var2 >> 2;
        while u_var3 != 0 {
            pi_var4.write_unaligned(0);
            pi_var4 = pi_var4.wrapping_add(1);
            u_var3 = u_var3.wrapping_sub(1);
        }
        u_var2 &= 3;
        while u_var2 != 0 {
            (pi_var4 as *mut u8).write(0);
            pi_var4 = (pi_var4 as *mut u8).wrapping_add(1) as *mut i32;
            u_var2 = u_var2.wrapping_sub(1);
        }
        pi_var1.write_unaligned(param_2);
        pi_var1.byte_add(4).cast::<u16>().write_unaligned(0);
        pi_var1.byte_add(6).cast::<u16>().write_unaligned(0);
        pi_var1.byte_add(16).write_unaligned(0);
        pi_var1.byte_add(20).cast::<u16>().write_unaligned(0);
        pi_var1.byte_add(8).write_unaligned(
            ((param_3 as u32).wrapping_mul(0x44).wrapping_add(0x18) & 0xffff) as i32,
        );
        let off2 = pi_var1.byte_add(8).read_unaligned();
        param_6.write_unaligned(
            (pi_var1 as i32)
                .wrapping_add((param_4 as u32).wrapping_mul(0x24) as i32)
                .wrapping_add(off2) as u32,
        );
        FUN_1001c299(param_1, pi_var1 as i32);
        let head = (param_1.wrapping_add(0x1c)) as *mut i32;
        if head.read_unaligned() == 0 {
            head.write_unaligned(pi_var1 as i32);
            pi_var1.byte_add(12).write_unaligned(0);
        } else {
            let mut local_10 = head.read_unaligned() as *mut i32;
            if param_2 < local_10.read_unaligned() {
                pi_var1.byte_add(12).write_unaligned(local_10 as i32);
                head.write_unaligned(pi_var1 as i32);
            } else {
                loop {
                    let next = local_10.byte_add(12).read_unaligned();
                    if next == 0 {
                        break;
                    }
                    if (next as *const i32).read_unaligned() > param_2 {
                        break;
                    }
                    local_10 = next as *mut i32;
                }
                pi_var1
                    .byte_add(12)
                    .write_unaligned(local_10.byte_add(12).read_unaligned());
                local_10.byte_add(12).write_unaligned(pi_var1 as i32);
            }
        }
        pi_var1
    }
}
