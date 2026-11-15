// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10013360(this: *mut c_void, object_base: i32);
    fn FUN_10012f10(this: *mut c_void, object_base: i32, param_2: i32, param_3: i32, param_4: i32);
}

#[unsafe(export_name = "FUN_10040640")]
#[allow(non_snake_case)]
pub unsafe extern "fastcall" fn RemoveTextItemFromSurface(item: i32) {
    mech_trace::enter!("10040640", item);
    let base = item as usize as *mut u8;
    // SAFETY: item points at the caller's object; fields are read and
    // written at the same offsets as the original, and the callees are the
    // game's own functions.
    unsafe {
        mech_trace::wr!(
            "10040640",
            base.byte_add(0x36).cast::<u32>(),
            write_unaligned,
            0xffff_ffff
        );
        mech_trace::wr!(
            "10040640",
            base.byte_add(0x3a).cast::<u32>(),
            write_unaligned,
            0xffff_ffff
        );
        mech_trace::wr!("10040640", base.byte_add(0x35), write, 0);
        FUN_10013360(
            mech_trace::rd!(
                "10040640",
                base.byte_add(0xc).cast::<*mut c_void>(),
                read_unaligned
            ),
            item,
        );
        mech_trace::wr!("10040640", base.byte_add(0x14), write, 0);
        let surface = mech_trace::rd!(
            "10040640",
            base.byte_add(0xc).cast::<*mut c_void>(),
            read_unaligned
        );
        let x = mech_trace::rd!(
            "10040640",
            base.byte_add(0x25).cast::<i32>(),
            read_unaligned
        );
        let y = mech_trace::rd!(
            "10040640",
            base.byte_add(0x29).cast::<i32>(),
            read_unaligned
        );
        let width = mech_trace::rd!(
            "10040640",
            base.byte_add(0x21).cast::<i32>(),
            read_unaligned
        );
        let height = mech_trace::rd!(
            "10040640",
            base.byte_add(0x1d).cast::<i32>(),
            read_unaligned
        );
        FUN_10012f10(surface, x, y, width, height);
    }
}
