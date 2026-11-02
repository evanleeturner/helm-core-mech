// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_10083508: *mut u8;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_1000be10() {
    // SAFETY: DAT_10083508 is the game's own pointer to a structure; the
    // counter at offset 0x15e is read and written where the original does.
    unsafe {
        let p = DAT_10083508.byte_add(0x15e).cast::<i32>();
        let counter = p.read_unaligned().wrapping_add(1);
        p.write_unaligned(counter);
    }
}
