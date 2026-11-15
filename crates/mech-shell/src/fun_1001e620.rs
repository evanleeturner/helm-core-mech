// SPDX-License-Identifier: MIT

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10017f30(param_1: i32, param_2: i32) -> u32;
    fn FUN_10017ec0(param_1: i32, param_2: i32, param_3: i8);
}

#[unsafe(export_name = "FUN_1001e620")]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn RemoveFromHeldCollection(
    collection_holder: *mut core::ffi::c_void,
    item: i32,
) {
    mech_trace::enter!("1001e620", collection_holder, item);
    // SAFETY: `collection_holder` points at the caller's object whose first field is an i32;
    // the callees are the game's own functions.
    unsafe {
        let found_index = mech_trace::cr!(
            "1001e620",
            "10017f30",
            FUN_10017f30(
                mech_trace::rd!("1001e620", collection_holder.cast::<i32>(), read_unaligned),
                item
            )
        );
        if (found_index as i32) >= 0 {
            FUN_10017ec0(
                mech_trace::rd!("1001e620", collection_holder.cast::<i32>(), read_unaligned),
                item,
                0,
            );
        }
    }
}
