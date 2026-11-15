// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10012590(
    surface: *mut u8,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) -> u32 {
    // A hot port: its call is one `fn.reads` line at its return, its reads
    // counted, in place of its `fn.in`, a `mem.rd` a read, and its `fn.ret`.
    let mut reads: u32 = 0;
    let mut counted = |value: i32| {
        reads += 1;
        value
    };
    // SAFETY: the owner points at the game's own object; the four bounds
    // fields are read at their byte offsets, in the original's order.
    let inside = unsafe {
        left <= counted(surface.byte_add(0x8a).cast::<i32>().read_unaligned())
            && counted(surface.byte_add(0x82).cast::<i32>().read_unaligned())
                <= width.wrapping_sub(1).wrapping_add(left)
            && top <= counted(surface.byte_add(0x8e).cast::<i32>().read_unaligned())
            && counted(surface.byte_add(0x86).cast::<i32>().read_unaligned())
                <= height.wrapping_sub(1).wrapping_add(top)
    };
    let answer = u32::from(inside);
    mech_trace::log::debug!(
        f = "10012590",
        a:% = mech_trace::Args([
            mech_trace::__shown!(surface),
            mech_trace::__shown!(left),
            mech_trace::__shown!(top),
            mech_trace::__shown!(width),
            mech_trace::__shown!(height)
        ]),
        n:% = mech_trace::__shown!(reads), v:% = mech_trace::__shown!(answer); "fn.reads"
    );
    answer
}
