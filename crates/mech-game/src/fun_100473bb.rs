// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100af0d8: *mut i32;
    static mut DAT_100af144: *mut u8;
    static mut DAT_100ad6c4: [u8; 1];
    static mut DAT_100ecbd0: [*mut i32; 0x780];
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_1007bbed(
        param_1: *mut i16,
        param_2: *mut i32,
        param_3: *mut u8,
        param_4: i32,
        param_5: *mut u32,
        param_6: *mut i32,
    ) -> *mut u16;
    fn FUN_10072879() -> i32;
    fn FUN_100814bb(param_1: i32, param_2: i32) -> i32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_100473bb(param_1: *mut i16) -> u32 {
    mech_trace::enter!("100473bb", param_1);
    let mut local_88: u32 = 0;
    let mut ai_stack_84: [i32; 32] = [0; 32];
    // SAFETY: the globals and callees are the game's own; puVar5 points at the
    // loaded data block, read at the offsets the original uses.
    unsafe {
        let base = &raw mut DAT_100ad6c4 as *mut u8;
        let table = &raw mut DAT_100ecbd0 as *mut *mut i32;
        let puv = mech_trace::cr!(
            "100473bb",
            "1007bbed",
            FUN_1007bbed(
                param_1,
                mech_trace::gr!("100473bb", "100af0d8", DAT_100af0d8),
                mech_trace::gr!("100473bb", "100af144", DAT_100af144),
                3,
                &raw mut local_88,
                base.byte_add(0xc) as *mut i32,
            )
        );
        if puv.is_null() {
            return mech_trace::rt!("100473bb", 0);
        }
        let i_var3 = mech_trace::rd!("100473bb", puv.cast::<i32>(), read_unaligned);
        let i_var4 = mech_trace::rd!("100473bb", puv.byte_add(4).cast::<i32>(), read_unaligned);
        let mut local_98: i32 = 8;
        let i_var7 = mech_trace::cr!("100473bb", "10072879", FUN_10072879());
        let mut local_a4: i32 = 0;
        while local_a4 < i_var3 {
            let u_var6 = mech_trace::rd!(
                "100473bb",
                puv.byte_add(local_98.wrapping_add(4) as u32 as usize)
                    .cast::<u32>(),
                read_unaligned
            );
            let i_var1 = local_98.wrapping_add(8);
            let first = mech_trace::rd!(
                "100473bb",
                puv.byte_add(local_98 as u32 as usize).cast::<i32>(),
                read_unaligned
            );
            if 0x1f < first {
                return mech_trace::rt!("100473bb", 0);
            }
            let i_var2 = first.wrapping_add(i_var7);
            if 0x77f < i_var2 {
                return mech_trace::rt!("100473bb", 0);
            }
            local_98 = i_var4.wrapping_mul(4).wrapping_add(i_var1);
            let i_var8 = mech_trace::cr!(
                "100473bb",
                "100814bb",
                FUN_100814bb(
                    0x14,
                    mech_trace::rd!("100473bb", base.byte_add(8).cast::<i32>(), read_unaligned)
                )
            );
            let slot = table.offset(i_var2 as isize);
            mech_trace::wr!(
                "100473bb",
                slot,
                write_unaligned,
                i_var8 as u32 as usize as *mut i32
            );
            if mech_trace::rd!("100473bb", slot, read_unaligned).is_null() {
                return mech_trace::rt!("100473bb", 0);
            }
            let ent = mech_trace::rd!("100473bb", slot, read_unaligned);
            mech_trace::wr!(
                "100473bb",
                ent.add(3),
                write_unaligned,
                (i_var1 as u32).wrapping_add(puv as u32) as i32
            );
            mech_trace::wr!("100473bb", ent.add(2), write_unaligned, u_var6 as i32);
            mech_trace::wr!("100473bb", ent.add(1), write_unaligned, i_var4);
            if mech_trace::rd!("100473bb", param_1, read_unaligned) == -1 {
                mech_trace::wr!("100473bb", ent, write_unaligned, 1);
            } else {
                mech_trace::wr!("100473bb", ent, write_unaligned, 0);
            }
            mech_trace::wr!(
                "100473bb",
                ai_stack_84.as_mut_ptr().offset(local_a4 as isize),
                write,
                i_var2
            );
            local_a4 = local_a4.wrapping_add(1);
        }
        local_a4 = 0;
        while local_a4 < i_var3 {
            let idx = mech_trace::rd!(
                "100473bb",
                ai_stack_84.as_ptr().offset(local_a4 as isize),
                read
            );
            let ent = mech_trace::rd!("100473bb", table.offset(idx as isize), read_unaligned);
            mech_trace::wr!(
                "100473bb",
                ent.add(4),
                write_unaligned,
                (local_98 as u32).wrapping_add(puv as u32) as i32
            );
            local_a4 = local_a4.wrapping_add(1);
        }
        mech_trace::rt!("100473bb", 1)
    }
}
