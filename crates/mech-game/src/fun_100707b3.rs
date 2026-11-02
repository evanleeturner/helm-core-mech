// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100aaf68: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100707b3(param_1: i32, param_2: *mut i32) -> *mut i32 {
    // SAFETY: the global is the game's own pointer value; param_1 is the address
    // of a game object and param_2 points at the caller's output buffer, read and
    // written where the original does.
    unsafe {
        let base = DAT_100aaf68 as *const i32;
        let obj = param_1 as u32 as *const u8;
        param_2.write_unaligned(
            base.read_unaligned()
                .wrapping_sub(obj.byte_add(0x50).cast::<i32>().read_unaligned()),
        );
        param_2.byte_add(4).write_unaligned(
            base.byte_add(4)
                .read_unaligned()
                .wrapping_sub(obj.byte_add(0x54).cast::<i32>().read_unaligned()),
        );
        param_2.byte_add(8).write_unaligned(
            base.byte_add(8)
                .read_unaligned()
                .wrapping_sub(obj.byte_add(0x58).cast::<i32>().read_unaligned()),
        );
        param_2
    }
}
