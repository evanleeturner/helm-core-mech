// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10018060(param_1: i32, param_2: i32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_10054ef0(this: *mut u8, one_based_index: i32) -> u32 {
    mech_trace::enter!("10054ef0", this, one_based_index);
    // SAFETY: `this` points at the game's container object; the fields at
    // 0x80 and 0x88 are read as the original does, and the collection
    // pointer at 0x88 is only dereferenced when the field at 0x80 is nonzero.
    unsafe {
        if mech_trace::rd!(
            "10054ef0",
            this.byte_add(0x80).cast::<i32>(),
            read_unaligned
        ) != 0
        {
            let coll = mech_trace::rd!(
                "10054ef0",
                this.byte_add(0x88).cast::<i32>(),
                read_unaligned
            );
            let count = mech_trace::rd!(
                "10054ef0",
                (coll as usize as *const u8).byte_add(4).cast::<i32>(),
                read_unaligned
            );
            if one_based_index <= count && -1 < one_based_index {
                let item = mech_trace::cr!(
                    "10054ef0",
                    "10018060",
                    FUN_10018060(coll, one_based_index.wrapping_sub(1))
                );
                return mech_trace::rt!("10054ef0", item);
            }
        }
    }
    mech_trace::rt!("10054ef0", 0)
}
