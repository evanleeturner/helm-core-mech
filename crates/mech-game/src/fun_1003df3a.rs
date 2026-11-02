// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af098: u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1003df3a(param_1: *mut u16) {
    mech_trace::enter!("1003df3a", param_1);
    if param_1.is_null() {
        return;
    }
    // SAFETY: param_1 points at the caller's record; the global is the game's
    // own. Reads and writes follow the original's order and offsets.
    unsafe {
        if (mech_trace::rd!("1003df3a", param_1, read_unaligned) & 0x800) != 0 {
            return;
        }
        mech_trace::wr!(
            "1003df3a",
            param_1,
            write_unaligned,
            mech_trace::rd!("1003df3a", param_1, read_unaligned) | 0x800
        );
        if mech_trace::gr!("1003df3a", "100af098", DAT_100af098) == 0 {
            return;
        }
        let p = param_1.cast::<u8>();
        let next = mech_trace::rd!("1003df3a", p.byte_add(0x10).cast::<i32>(), read_unaligned);
        if next != 0 {
            let v = mech_trace::rd!("1003df3a", p.byte_add(0xc).cast::<u32>(), read_unaligned);
            (next as u32).wrapping_add(0xc).cast_ptr_write(v);
        }
        let prev = mech_trace::rd!("1003df3a", p.byte_add(0xc).cast::<i32>(), read_unaligned);
        if prev != 0 {
            let v = mech_trace::rd!("1003df3a", p.byte_add(0x10).cast::<u32>(), read_unaligned);
            mech_trace::wr!(
                "1003df3a",
                ((prev as u32).wrapping_add(0x10) as *mut u32),
                write_unaligned,
                v
            );
        }
        mech_trace::wr!(
            "1003df3a",
            p.byte_add(0xc).cast::<u16>(),
            write_unaligned,
            0
        );
        mech_trace::wr!(
            "1003df3a",
            p.byte_add(0xe).cast::<u16>(),
            write_unaligned,
            0
        );
        let v = mech_trace::rd!("1003df3a", p.byte_add(0xc).cast::<u32>(), read_unaligned);
        mech_trace::wr!(
            "1003df3a",
            p.byte_add(0x10).cast::<u32>(),
            write_unaligned,
            v
        );
    }
}

trait CastPtrWrite {
    unsafe fn cast_ptr_write(self, v: u32);
}

impl CastPtrWrite for u32 {
    unsafe fn cast_ptr_write(self, v: u32) {
        // SAFETY: the caller guarantees self is a valid address for a u32 write.
        unsafe { mech_trace::wr!("1003df3a", (self as *mut u32), write_unaligned, v) }
    }
}
