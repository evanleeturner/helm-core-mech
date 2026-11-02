// SPDX-License-Identifier: MIT

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_1000d230(id_likely: i32) -> i8 {
    if id_likely != -1 && id_likely != 0 {
        if id_likely < 7000 {
            return 0;
        }
        return ((id_likely < 10000) as i8).wrapping_add(1);
    }
    3
}
