// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut _DAT_10160f48: i32;
    static mut _DAT_10160f44: i32;
    static mut _DAT_10160f4c: i32;
    static mut DAT_10160ef4: i32;
    static mut DAT_10160ef8: i32;
    static mut DAT_10160efc: i32;
    static mut DAT_10160f00: i32;
    static mut DAT_10160f04: i32;
    static mut DAT_10160f08: i32;
    static mut DAT_10160f0c: i32;
    static mut DAT_10160f10: i32;
    static mut DAT_10160f14: i32;
    static mut DAT_10168398: i32;
    static mut DAT_10160eb4: i32;
    static mut DAT_10160eb8: i32;
    static mut DAT_10160ec0: i32;
    static mut DAT_10160ec4: i32;
    static mut DAT_10160ed0: i32;
    static mut DAT_10160edc: i32;
    static mut DAT_10160ee0: i32;
    static mut DAT_10160ee8: i32;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1001a080(
        param_1: i32,
        param_2: i32,
        param_3: i32,
        param_4: i32,
        param_5: i32,
        param_6: i32,
    ) -> i32;
    fn FUN_100359c0(param_1: u32, param_2: i32, param_3: u8, param_4: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100357e3(param_1: *mut i32) -> u32 {
    // SAFETY: globals and callees are the game's own; param_1 points at three
    // readable and writable i32 values supplied by the caller.
    unsafe {
        let mut local_8: u32 = 0;
        let mut i_var1 = param_1.read_unaligned().wrapping_sub(_DAT_10160f48);
        let mut i_var2 = param_1.add(1).read_unaligned().wrapping_sub(_DAT_10160f44);
        let mut i_var3 = param_1.add(2).read_unaligned().wrapping_sub(_DAT_10160f4c);
        let u_var4 = FUN_1001a080(
            i_var1,
            DAT_10160ef4,
            i_var2,
            DAT_10160ef8,
            i_var3,
            DAT_10160efc,
        ) as u32;
        let u_var5 = FUN_1001a080(
            i_var1,
            DAT_10160f00,
            i_var2,
            DAT_10160f04,
            i_var3,
            DAT_10160f08,
        ) as u32;
        i_var1 = FUN_1001a080(
            i_var1,
            DAT_10160f0c,
            i_var2,
            DAT_10160f10,
            i_var3,
            DAT_10160f14,
        );
        i_var2 = FUN_100359c0(u_var4, DAT_10168398, DAT_10160eb4 as u8, DAT_10160ec4);
        param_1.write_unaligned(i_var2);
        i_var3 = DAT_10160ed0.wrapping_sub(DAT_10160ee0);
        i_var2 = FUN_100359c0(u_var5, DAT_10168398, DAT_10160eb8 as u8, DAT_10160ee8);
        param_1.add(1).write_unaligned(i_var3.wrapping_sub(i_var2));
        param_1.add(2).write_unaligned(i_var1);
        if 0 < i_var1 {
            if param_1.read_unaligned() < DAT_10160ec0
                || DAT_10160edc < param_1.read_unaligned()
                || param_1.add(1).read_unaligned() < DAT_10160ee0
                || DAT_10160ed0 < param_1.add(1).read_unaligned()
            {
                local_8 = 0;
            } else {
                local_8 = 1;
            }
        }
        local_8
    }
}
