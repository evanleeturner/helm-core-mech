// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut c_void;
    static mut DAT_100ba7bc: u32;
    static mut DAT_100ba7c0: u32;
    static mut DAT_100ba824: [i8; 0];
    static mut DAT_100c9054: *mut u16;
    static mut DAT_100ba828: [i8; 0];
    static mut DAT_100ba850: [i8; 0];
    static mut DAT_100ba8a0: [i8; 0];
    static mut DAT_100ba878: [i8; 0];
    static mut DAT_100ba814: [i8; 0];
}

unsafe extern "C" {
    fn sprintf(dest: *mut i8, format: *const i8, ...) -> i32;
    fn fopen(filename: *const i8, mode: *const i8) -> *mut c_void;
    fn fprintf(file: *mut c_void, format: *const i8, ...) -> i32;
    fn fclose(file: *mut c_void) -> i32;
}

unsafe extern "stdcall" {
    fn HeapSize(h: *mut c_void, flags: u32, p: *const c_void) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "stdcall" fn FUN_100864d7() {
    mech_trace::enter!("100864d7");
    let mut name = [0i8; 100];
    let mut tag = [0u8; 8];
    // SAFETY: globals, the C runtime and HeapSize are the game's own; the
    // list nodes are the game's cache entries, read where the original does.
    unsafe {
        let i_var1 = mech_trace::gr!("100864d7", "100ba7c0", DAT_100ba7c0);
        mech_trace::gw!(
            "100864d7",
            "100ba7c0",
            DAT_100ba7c0,
            mech_trace::gr!("100864d7", "100ba7c0", DAT_100ba7c0).wrapping_add(1)
        );
        mech_trace::cr!(
            "100864d7",
            "sprintf",
            sprintf(
                name.as_mut_ptr(),
                (&raw const DAT_100ba814).cast::<i8>(),
                i_var1 as i32,
            )
        );
        let file = mech_trace::cr!(
            "100864d7",
            "fopen",
            fopen(name.as_ptr(), (&raw const DAT_100ba824).cast::<i8>())
        );
        tag[4] = 0;
        mech_trace::cr!(
            "100864d7",
            "fprintf",
            fprintf(file, (&raw const DAT_100ba828).cast::<i8>())
        );
        let mut i: i32 = 0;
        while i < 0x3f1 {
            let mut node = mech_trace::rd!(
                "100864d7",
                ((mech_trace::gr!("100864d7", "100ba7bc", DAT_100ba7bc) as usize)
                    .wrapping_add((i as usize).wrapping_mul(4))
                    as *const *mut u16),
                read_unaligned
            );
            while !node.is_null() {
                let t = mech_trace::rd!("100864d7", node.byte_add(4).cast::<u32>(), read_unaligned);
                tag[0..4].copy_from_slice(&t.to_le_bytes());
                let sz = mech_trace::cr!(
                    "100864d7",
                    "HeapSize",
                    HeapSize(
                        mech_trace::gr!("100864d7", "100adf74", DAT_100adf74),
                        1,
                        node as *const c_void
                    )
                );
                mech_trace::cr!(
                    "100864d7",
                    "fprintf",
                    fprintf(
                        file,
                        (&raw const DAT_100ba850).cast::<i8>(),
                        mech_trace::rd!("100864d7", node, read_unaligned) as u32,
                        tag.as_ptr(),
                        mech_trace::rd!("100864d7", node.byte_add(2).cast::<i16>(), read_unaligned)
                            as i32,
                        sz,
                    )
                );
                node = mech_trace::rd!(
                    "100864d7",
                    node.byte_add(8).cast::<*mut u16>(),
                    read_unaligned
                );
            }
            i = i.wrapping_add(1);
        }
        mech_trace::cr!(
            "100864d7",
            "fprintf",
            fprintf(file, (&raw const DAT_100ba878).cast::<i8>())
        );
        let mut node = mech_trace::gr!("100864d7", "100c9054", DAT_100c9054);
        while !node.is_null() {
            let t = mech_trace::rd!("100864d7", node.byte_add(4).cast::<u32>(), read_unaligned);
            tag[0..4].copy_from_slice(&t.to_le_bytes());
            let sz = mech_trace::cr!(
                "100864d7",
                "HeapSize",
                HeapSize(
                    mech_trace::gr!("100864d7", "100adf74", DAT_100adf74),
                    1,
                    node as *const c_void
                )
            );
            mech_trace::cr!(
                "100864d7",
                "fprintf",
                fprintf(
                    file,
                    (&raw const DAT_100ba8a0).cast::<i8>(),
                    mech_trace::rd!("100864d7", node, read_unaligned) as u32,
                    tag.as_ptr(),
                    mech_trace::rd!("100864d7", node.byte_add(2).cast::<i16>(), read_unaligned)
                        as i32,
                    sz,
                )
            );
            node = mech_trace::rd!(
                "100864d7",
                node.byte_add(12).cast::<*mut u16>(),
                read_unaligned
            );
        }
        mech_trace::cr!("100864d7", "fclose", fclose(file));
    }
}
