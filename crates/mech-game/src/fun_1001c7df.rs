// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1001c7df(param_1: i32, param_2: u16, param_3: i32) -> *mut u16 {
    mech_trace::enter!("1001c7df", param_1, param_2, param_3);
    // SAFETY: param_1 points at the caller's structure, and the structure at
    // its +0x20 field points at a table; all reads and writes happen where the
    // original does them.
    unsafe {
        let i_var1 = mech_trace::rd!(
            "1001c7df",
            ((param_1 as u32).wrapping_add(0x20) as usize as *const i32),
            read_unaligned
        );
        if i_var1 == 0 {
            return mech_trace::rt!("1001c7df", core::ptr::null_mut());
        }
        let base = i_var1 as u32 as usize as *mut u8;
        let off8 = mech_trace::rd!("1001c7df", base.byte_add(8).cast::<i32>(), read_unaligned);
        let count = mech_trace::rd!("1001c7df", base.byte_add(6).cast::<i16>(), read_unaligned);
        let addr = (off8 as u32)
            .wrapping_add(i_var1 as u32)
            .wrapping_add((count as i32 as u32).wrapping_mul(0x24));
        let p = addr as usize as *mut u8;
        mech_trace::wr!(
            "1001c7df",
            p.byte_add(4).cast::<u32>(),
            write_unaligned,
            (param_3 as u32).wrapping_sub(addr) & 0xffff
        );
        mech_trace::wr!(
            "1001c7df",
            p.byte_add(0x20).cast::<i32>(),
            write_unaligned,
            param_1
        );
        mech_trace::wr!("1001c7df", p.cast::<u16>(), write_unaligned, param_2);
        mech_trace::wr!("1001c7df", p.byte_add(2).cast::<u16>(), write_unaligned, 0);
        let cur = mech_trace::rd!("1001c7df", base.byte_add(6).cast::<i16>(), read_unaligned);
        mech_trace::wr!(
            "1001c7df",
            base.byte_add(6).cast::<i16>(),
            write_unaligned,
            (cur as i32 as u32).wrapping_add(1) as i16
        );
        mech_trace::rt!("1001c7df", p.cast::<u16>())
    }
}
