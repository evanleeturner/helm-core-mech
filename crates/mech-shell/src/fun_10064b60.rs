// SPDX-License-Identifier: MIT

use core::ffi::c_void;

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1006f200(param_1: u32) -> *mut c_void;
}

#[allow(non_snake_case)]
unsafe extern "thiscall" {
    fn FUN_10064d50(
        this: *mut c_void,
        param_1: u32,
        param_2: u32,
        param_3: u32,
        param_4: u32,
    ) -> *mut u32;
    fn FUN_10064c90(this: *mut c_void, param_1: *mut c_void);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10064b60(
    this: *mut c_void,
    param_1: u32,
    param_2: u32,
    param_3: u32,
    param_4: u32,
    param_5: u32,
    param_6: u8,
    param_7: *mut c_void,
    param_8: u32,
    param_9: u32,
    param_10: u32,
    param_11: u32,
    param_12: u32,
    param_13: u8,
    param_14: u32,
) -> *mut u32 {
    mech_trace::enter!(
        "10064b60", this, param_1, param_2, param_3, param_4, param_5, param_6, param_7, param_8,
        param_9, param_10, param_11, param_12, param_13, param_14
    );
    // SAFETY: `this` points at the caller's object; the callees are the
    // game's own, and fields are written at the original's offsets.
    unsafe {
        mech_trace::wr!(
            "10064b60",
            this.byte_add(0x24).cast::<u32>(),
            write_unaligned,
            param_5
        );
        mech_trace::wr!("10064b60", this.cast::<u32>(), write_unaligned, param_9);
        mech_trace::wr!("10064b60", this.byte_add(0x28).cast::<u8>(), write, param_6);
        mech_trace::wr!(
            "10064b60",
            this.byte_add(4).cast::<u32>(),
            write_unaligned,
            param_10
        );
        mech_trace::wr!(
            "10064b60",
            this.byte_add(8).cast::<u32>(),
            write_unaligned,
            param_11
        );
        mech_trace::wr!(
            "10064b60",
            this.byte_add(0x10).cast::<u32>(),
            write_unaligned,
            param_8
        );
        mech_trace::wr!(
            "10064b60",
            this.byte_add(0x14).cast::<u32>(),
            write_unaligned,
            param_12
        );
        mech_trace::wr!(
            "10064b60",
            this.byte_add(0x29).cast::<u8>(),
            write,
            param_13
        );
        mech_trace::wr!(
            "10064b60",
            this.byte_add(0x1c).cast::<u32>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "10064b60",
            this.byte_add(0xc).cast::<u32>(),
            write_unaligned,
            0xffff_ffff
        );
        mech_trace::wr!(
            "10064b60",
            this.byte_add(0x2a).cast::<u32>(),
            write_unaligned,
            param_14
        );
        let this_00 = mech_trace::cr!("10064b60", "1006f200", FUN_1006f200(0x10));
        let mut p_var1: *mut u32 = core::ptr::null_mut();
        if !this_00.is_null() {
            p_var1 = mech_trace::cr!(
                "10064b60",
                "10064d50",
                FUN_10064d50(this_00, param_1, param_2, param_3, param_4)
            );
        }
        mech_trace::wr!(
            "10064b60",
            this.byte_add(0x18).cast::<*mut u32>(),
            write_unaligned,
            p_var1
        );
        if mech_trace::rd!("10064b60", this.byte_add(0x28).cast::<u8>(), read) == 1 {
            FUN_10064c90(this, param_7);
        }
        mech_trace::rt!("10064b60", this.cast::<u32>())
    }
}
