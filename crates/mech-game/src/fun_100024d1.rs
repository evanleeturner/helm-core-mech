// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100b38d8: i32;
    static mut DAT_100eef40: [i32; 0];
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100024d1() {
    // SAFETY: the globals are the game's own memory; the table entries are
    // addresses of game structures whose pointer at +0x20 leads to a block
    // holding the word at +0x3d0.
    unsafe {
        let mut local_8: i32 = 0;
        while local_8 < DAT_100b38d8 {
            let base = (&raw mut DAT_100eef40).cast::<i32>();
            let entry = base.offset(local_8 as isize).read_unaligned();
            let inner = ((entry as u32).wrapping_add(0x20) as *const i32).read_unaligned();
            let piVar1 = (inner as u32).wrapping_add(0x3d0) as *mut i32;
            let v = piVar1.read_unaligned();
            piVar1.write_unaligned(((v as u32).wrapping_shl(2)) as i32);
            local_8 = local_8.wrapping_add(1);
        }
    }
}
