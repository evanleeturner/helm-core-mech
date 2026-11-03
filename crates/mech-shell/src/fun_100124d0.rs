// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_100124d0(
    this: *mut u8,
    left: i32,
    top: i32,
    width: i32,
    height: i32,
) {
    mech_trace::enter!("100124d0", this, left, top, width, height);
    let right = left.wrapping_add(-1).wrapping_add(width);
    let bottom = top.wrapping_add(-1).wrapping_add(height);
    // SAFETY: `this` points at the caller's object; the fields at offsets
    // 0x82..0x8e are read and written unaligned where the original does.
    unsafe {
        let p_left = this.byte_add(0x82).cast::<i32>();
        let p_top = this.byte_add(0x86).cast::<i32>();
        let p_right = this.byte_add(0x8a).cast::<i32>();
        let p_bottom = this.byte_add(0x8e).cast::<i32>();
        if left < mech_trace::rd!("100124d0", p_left, read_unaligned) {
            mech_trace::wr!("100124d0", p_left, write_unaligned, left);
        }
        if top < mech_trace::rd!("100124d0", p_top, read_unaligned) {
            mech_trace::wr!("100124d0", p_top, write_unaligned, top);
        }
        if mech_trace::rd!("100124d0", p_right, read_unaligned) < right {
            mech_trace::wr!("100124d0", p_right, write_unaligned, right);
        }
        if mech_trace::rd!("100124d0", p_bottom, read_unaligned) < bottom {
            mech_trace::wr!("100124d0", p_bottom, write_unaligned, bottom);
        }
    }
}
