// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100adf74: *mut u8;
    static mut DAT_100ae5a8: i8;
    static mut DAT_100ae5b4: i8;
    static mut DAT_100ae5c0: i8;
    static mut DAT_100ae438: i8;
    static mut DAT_100ae4c0: i8;
    static mut DAT_100ae450: i8;
    static mut DAT_100ae4c8: i8;
    static mut DAT_100ae500: i8;
    static mut DAT_100ae518: i8;
    static mut DAT_100ae468: i8;
    static mut DAT_100ae4d8: i8;
    static mut DAT_100ae480: i8;
    static mut DAT_100ae4e8: i8;
    static mut DAT_100ae490: i8;
    static mut DAT_100ae4f0: i8;
    static mut DAT_100ae4a8: i8;
    static mut DAT_100ae4f8: i8;
    static mut DAT_100ae5c8: i8;
    static mut DAT_100ae5d4: i8;
    static mut DAT_100ae528: i8;
    static mut DAT_100ae560: i8;
    static mut DAT_100ae530: i8;
    static mut DAT_100ae568: i8;
    static mut DAT_100ae540: i8;
    static mut DAT_100ae578: i8;
    static mut DAT_100ae550: i8;
    static mut DAT_100ae580: i8;
}

#[allow(non_snake_case)]
unsafe extern "stdcall" {
    fn joyGetDevCapsA(uJoyID: u32, pjc: *mut u8, cbjc: u32) -> u32;
    fn HeapAlloc(hHeap: *mut u8, dwFlags: u32, dwBytes: u32) -> *mut u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn FUN_10038a52(joy_id: i32);
    fn FUN_10038f3a(joy_id: u32, param_2: *mut i8, param_3: *mut i8, param_4: i32) -> i32;
    fn sprintf(buffer: *mut i8, format: *const i8, ...) -> i32;
    fn FUN_1003909d(joy_id: i32, param_2: i32);
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "C" fn FUN_10037fe6(joy_id: u32, param_2: *mut i8) -> u32 {
    let mut local_198: [u32; 0x65] = [0; 0x65];
    // SAFETY: globals and callees are the game's own; param_2 points at the
    // caller's device structure and the heap blocks are used within their
    // allocated sizes as the original does.
    unsafe {
        let m_var2 = joyGetDevCapsA(joy_id, local_198.as_mut_ptr() as *mut u8, 0x194);
        if m_var2 != 0 {
            return 1;
        }
        let puvar4 = HeapAlloc(DAT_100adf74, 9, 0x18) as *mut *const i8;
        param_2
            .byte_add(0x60)
            .cast::<*mut *const i8>()
            .write_unaligned(puvar4);
        let puvar5 = HeapAlloc(DAT_100adf74, 9, 0x18) as *mut *const i8;
        param_2
            .byte_add(100)
            .cast::<*mut *const i8>()
            .write_unaligned(puvar5);
        let pivar6 = HeapAlloc(DAT_100adf74, 9, 0x90) as *mut i32;
        param_2
            .byte_add(0x68)
            .cast::<*mut i32>()
            .write_unaligned(pivar6);
        let pivar7 = HeapAlloc(DAT_100adf74, 9, 0x90) as *mut i32;
        param_2
            .byte_add(0x6c)
            .cast::<*mut i32>()
            .write_unaligned(pivar7);
        if puvar4.is_null() || puvar5.is_null() || pivar6.is_null() || pivar7.is_null() {
            FUN_10038a52(param_2 as i32);
            return 1;
        }
        let pu_var8 = HeapAlloc(DAT_100adf74, 9, 0x88) as *mut u32;
        if pu_var8.is_null() {
            return 1;
        }
        pu_var8.write_unaligned(joy_id);
        param_2
            .byte_add(0x70)
            .cast::<*mut u32>()
            .write_unaligned(pu_var8);
        if FUN_10038f3a(
            joy_id,
            (&raw mut local_198[0x70 / 4]) as *mut i8,
            param_2.byte_add(0xc),
            0x40,
        ) == 0
        {
            sprintf(
                param_2.byte_add(0xc),
                &raw const DAT_100ae5a8,
                joy_id.wrapping_add(1),
            );
        }
        sprintf(param_2, &raw const DAT_100ae5b4, joy_id.wrapping_add(1));
        FUN_1003909d(param_2 as i32, local_198.as_mut_ptr() as i32);

        puvar4.write_unaligned(&raw const DAT_100ae438);
        puvar5.write_unaligned(&raw const DAT_100ae4c0);
        puvar4.add(1).write_unaligned(&raw const DAT_100ae450);
        puvar5.add(1).write_unaligned(&raw const DAT_100ae4c8);
        let mut local_1a4: u32 = 2;
        let caps = local_198[24] as u8;
        let mut b_var1 = (caps & 1) == 0;
        if !b_var1 {
            let mut i_var10: i32 = 8;
            let mut pc11 = param_2.byte_add(0x4c) as *const i8;
            let mut pc12 = &raw const DAT_100ae5c0 as *const i8;
            loop {
                if i_var10 == 0 {
                    break;
                }
                i_var10 = i_var10.wrapping_sub(1);
                b_var1 = *pc11 == *pc12;
                pc11 = pc11.add(1);
                pc12 = pc12.add(1);
                if !b_var1 {
                    break;
                }
            }
            if b_var1 {
                puvar4.add(2).write_unaligned(&raw const DAT_100ae500);
                puvar5.add(2).write_unaligned(&raw const DAT_100ae518);
                local_1a4 = 3;
            } else {
                puvar4.add(2).write_unaligned(&raw const DAT_100ae468);
                puvar5.add(2).write_unaligned(&raw const DAT_100ae4d8);
                local_1a4 = 3;
            }
        }
        if caps & 2 != 0 {
            puvar4
                .add(local_1a4 as usize)
                .write_unaligned(&raw const DAT_100ae480);
            puvar5
                .add(local_1a4 as usize)
                .write_unaligned(&raw const DAT_100ae4e8);
            local_1a4 = local_1a4.wrapping_add(1);
        }
        if caps & 4 != 0 {
            puvar4
                .add(local_1a4 as usize)
                .write_unaligned(&raw const DAT_100ae490);
            puvar5
                .add(local_1a4 as usize)
                .write_unaligned(&raw const DAT_100ae4f0);
            local_1a4 = local_1a4.wrapping_add(1);
        }
        if caps & 8 != 0 {
            puvar4
                .add(local_1a4 as usize)
                .write_unaligned(&raw const DAT_100ae4a8);
            puvar5
                .add(local_1a4 as usize)
                .write_unaligned(&raw const DAT_100ae4f8);
            local_1a4 = local_1a4.wrapping_add(1);
        }
        param_2
            .byte_add(0x58)
            .cast::<u32>()
            .write_unaligned(local_1a4);

        let pv = HeapAlloc(DAT_100adf74, 9, 0x180);
        pivar6.write_unaligned(pv as i32);
        let pv = HeapAlloc(DAT_100adf74, 9, 0x180);
        pivar7.write_unaligned(pv as i32);
        if pivar6.read_unaligned() == 0 || pivar7.read_unaligned() == 0 {
            FUN_10038a52(param_2 as i32);
            return 1;
        }
        local_1a4 = 0;
        while (local_1a4 as i32) < (local_198[15] as i32) {
            let i = local_1a4 as usize;
            let a = pivar6.add(i).read_unaligned();
            sprintf(
                a as usize as *mut i8,
                &raw const DAT_100ae5c8,
                local_1a4.wrapping_add(1),
            );
            let b = pivar7.add(i).read_unaligned();
            sprintf(
                b as usize as *mut i8,
                &raw const DAT_100ae5d4,
                local_1a4.wrapping_add(1),
            );
            let a = pivar6.add(i).read_unaligned();
            pivar6
                .add(i.wrapping_add(1))
                .write_unaligned(((a as u32).wrapping_add(0xc)) as i32);
            let b = pivar7.add(i).read_unaligned();
            pivar7
                .add(i.wrapping_add(1))
                .write_unaligned(((b as u32).wrapping_add(0xc)) as i32);
            local_1a4 = local_1a4.wrapping_add(1);
        }
        local_1a4 = local_198[15];
        if caps & 0x10 != 0 {
            let n = local_198[15] as usize;
            pivar6
                .add(n)
                .write_unaligned((&raw const DAT_100ae528) as i32);
            pivar7
                .add(n)
                .write_unaligned((&raw const DAT_100ae560) as i32);
            pivar6
                .add(n.wrapping_add(1))
                .write_unaligned((&raw const DAT_100ae530) as i32);
            pivar7
                .add(n.wrapping_add(1))
                .write_unaligned((&raw const DAT_100ae568) as i32);
            pivar6
                .add(n.wrapping_add(2))
                .write_unaligned((&raw const DAT_100ae540) as i32);
            pivar7
                .add(n.wrapping_add(2))
                .write_unaligned((&raw const DAT_100ae578) as i32);
            pivar6
                .add(n.wrapping_add(3))
                .write_unaligned((&raw const DAT_100ae550) as i32);
            pivar7
                .add(n.wrapping_add(3))
                .write_unaligned((&raw const DAT_100ae580) as i32);
            local_1a4 = local_198[15].wrapping_add(4);
        }
        param_2
            .byte_add(0x5c)
            .cast::<u32>()
            .write_unaligned(local_1a4);
        0
    }
}
