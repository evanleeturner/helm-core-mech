// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100984cc: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10042370(param_1: i32, param_2: *mut i32) -> *mut i16 {
    if param_1 < 0 {
        return core::ptr::null_mut();
    }
    // SAFETY: the table pointer is the game's own global; param_2 points at the
    // caller's key bytes and chain nodes are read at the original's offsets.
    unsafe {
        let b = param_2 as *const i8;
        let b2 = b.byte_add(2).read() as i32;
        let b3 = b.byte_add(3).read() as i32;
        let b1 = b.byte_add(1).read() as i32;
        let b0 = (param_2.read_unaligned() as i8) as i32;
        let sum = b2
            .wrapping_add(b3)
            .wrapping_add(b1)
            .wrapping_add(b0)
            .wrapping_add(param_1);
        let idx = sum % 0x3f1;
        let addr = DAT_100984cc.wrapping_add((idx as u32).wrapping_mul(4));
        let mut node: *mut i16 = (addr as *const u32).read_unaligned() as usize as *mut i16;
        while !node.is_null() {
            if node.read_unaligned() as i32 == param_1
                && node.byte_add(4).cast::<i32>().read_unaligned() == param_2.read_unaligned()
            {
                break;
            }
            node = node.byte_add(8).cast::<u32>().read_unaligned() as usize as *mut i16;
        }
        node
    }
}
