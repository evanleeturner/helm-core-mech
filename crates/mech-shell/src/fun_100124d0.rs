// SPDX-License-Identifier: MIT

#[unsafe(export_name = "FUN_100124d0")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn ExtendDirtyRect(
    surface: *mut u8,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) {
    // A hot port: its call is one `fn.reads` line at its return, its reads
    // counted, in place of its `fn.in` and a `mem.rd` a read; each write
    // keeps its own `mem.wr` line.
    let mut reads: u32 = 0;
    let mut counted = |value: i32| {
        reads += 1;
        value
    };
    let right = left.wrapping_add(-1).wrapping_add(width);
    let bottom = top.wrapping_add(-1).wrapping_add(height);
    // SAFETY: `surface` points at the caller's object; the fields at offsets
    // 0x82..0x8e are read and written unaligned where the original does.
    unsafe {
        let dirty_left_slot = surface.byte_add(0x82).cast::<i32>();
        let dirty_top_slot = surface.byte_add(0x86).cast::<i32>();
        let dirty_right_slot = surface.byte_add(0x8a).cast::<i32>();
        let dirty_bottom_slot = surface.byte_add(0x8e).cast::<i32>();
        if left < counted(dirty_left_slot.read_unaligned()) {
            mech_trace::wr!("100124d0", dirty_left_slot, write_unaligned, left);
        }
        if top < counted(dirty_top_slot.read_unaligned()) {
            mech_trace::wr!("100124d0", dirty_top_slot, write_unaligned, top);
        }
        if counted(dirty_right_slot.read_unaligned()) < right {
            mech_trace::wr!("100124d0", dirty_right_slot, write_unaligned, right);
        }
        if counted(dirty_bottom_slot.read_unaligned()) < bottom {
            mech_trace::wr!("100124d0", dirty_bottom_slot, write_unaligned, bottom);
        }
    }
    mech_trace::log::debug!(
        f = "100124d0",
        a:% = mech_trace::Args([
            mech_trace::__shown!(surface),
            mech_trace::__shown!(left),
            mech_trace::__shown!(top),
            mech_trace::__shown!(width),
            mech_trace::__shown!(height)
        ]),
        n:% = mech_trace::__shown!(reads); "fn.reads"
    );
}
