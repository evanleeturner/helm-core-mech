// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d230(id_likely: i32) -> i8 {
    mech_trace::enter!("1000d230", id_likely);
    if id_likely != -1 && id_likely != 0 {
        if id_likely < 7000 {
            return mech_trace::rt!("1000d230", 0);
        }
        return mech_trace::rt!("1000d230", ((id_likely < 10000) as i8).wrapping_add(1));
    }
    mech_trace::rt!("1000d230", 3)
}
