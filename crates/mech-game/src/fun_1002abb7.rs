// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1002abb7(param_1: i32, param_2: i16, param_3: u16) {
    // SAFETY: param_1 is the address of a game object; the fields at offsets
    // 0x170 and 0x172 are read and written where the original does.
    unsafe {
        let p = param_1 as usize as *mut u8;
        if p.byte_add(0x170).cast::<i16>().read_unaligned() == 0 {
            p.byte_add(0x170).cast::<i16>().write_unaligned(param_2);
            p.byte_add(0x172).cast::<u16>().write_unaligned(param_3);
        }
    }
}
