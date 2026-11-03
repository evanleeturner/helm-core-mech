// SPDX-License-Identifier: MIT

#[allow(non_upper_case_globals)]
unsafe extern "C" {
    static mut DAT_100a6780: u8;
    static mut DAT_100a677c: u32;
    static mut s__ssmk__s_smk_100a7204: u8;
    static mut s_smk__s_smk_100a7214: u8;
}

#[allow(non_snake_case)]
unsafe extern "C" {
    fn _sprintf(dest: *mut i8, format: *const u8, ...) -> i32;
    fn SmackOpen(name: *const i8, flags: u32, param_3: u32) -> i32;
    fn SmackColorRemap(smk: i32, remap: u32, count: u32, flags: u32);
    fn SmackToBuffer(smk: i32, x: u32, y: u32, w: u32, h: u32, buf: u32, flags: u32) -> u32;
    fn FUN_1006fab0(size: u32) -> u32;
}

#[unsafe(no_mangle)]
#[allow(non_snake_case)]
pub unsafe extern "thiscall" fn FUN_1005f530(this: *mut u8, param_1: i32) -> u32 {
    mech_trace::enter!("1005f530", this, param_1);
    let mut local_20 = [0i8; 32];
    // SAFETY: this points at the game's object; the globals, callees and the
    // Smack handle are the game's own, used as the original does.
    unsafe {
        if param_1 == 1 && mech_trace::gr!("1005f530", "100a6780", DAT_100a6780) != 0 {
            mech_trace::cr!(
                "1005f530",
                "_sprintf",
                _sprintf(
                    local_20.as_mut_ptr(),
                    &raw const s__ssmk__s_smk_100a7204,
                    &raw mut DAT_100a6780,
                    this,
                )
            );
        } else {
            mech_trace::cr!(
                "1005f530",
                "_sprintf",
                _sprintf(
                    local_20.as_mut_ptr(),
                    &raw const s_smk__s_smk_100a7214,
                    this,
                )
            );
        }
        mech_trace::gu!("1005f530", "100a677c", DAT_100a677c, |=, 0xfe000);
        let f10c = mech_trace::rd!(
            "1005f530",
            this.byte_add(0x10c).cast::<u32>(),
            read_unaligned
        );
        let flags = ((f10c & 0x40) << 3) | mech_trace::gr!("1005f530", "100a677c", DAT_100a677c);
        let h = mech_trace::cr!(
            "1005f530",
            "SmackOpen",
            SmackOpen(local_20.as_ptr(), flags, 0)
        );
        mech_trace::wr!(
            "1005f530",
            this.byte_add(0x100).cast::<i32>(),
            write_unaligned,
            h
        );
        if h == 0 {
            return mech_trace::rt!("1005f530", 0);
        }
        let f10c = mech_trace::rd!(
            "1005f530",
            this.byte_add(0x10c).cast::<u32>(),
            read_unaligned
        );
        if f10c & 0x10000000 != 0 {
            let p = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x140).cast::<u32>(),
                read_unaligned
            );
            SmackColorRemap(h, p.wrapping_add(0x92), 0xff, 0);
        }
        let ecx = mech_trace::rd!(
            "1005f530",
            this.byte_add(0x100).cast::<i32>(),
            read_unaligned
        );
        let ebp = mech_trace::rd!(
            "1005f530",
            this.byte_add(0x10c).cast::<u32>(),
            read_unaligned
        );
        let hp = ecx as usize as *const u8;
        let ebx = mech_trace::rd!("1005f530", hp.byte_add(4).cast::<i32>(), read_unaligned);
        mech_trace::wr!(
            "1005f530",
            this.byte_add(0x118).cast::<i32>(),
            write_unaligned,
            ebx
        );
        let edi = mech_trace::rd!("1005f530", hp.byte_add(8).cast::<i32>(), read_unaligned);
        mech_trace::wr!(
            "1005f530",
            this.byte_add(0x11c).cast::<i32>(),
            write_unaligned,
            edi
        );
        if ebp & 0x80 != 0 {
            let half = ebx / 2;
            let x = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x110).cast::<i32>(),
                read_unaligned
            );
            let y = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x114).cast::<i32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "1005f530",
                this.byte_add(0x110).cast::<i32>(),
                write_unaligned,
                x.wrapping_sub(half)
            );
            mech_trace::wr!(
                "1005f530",
                this.byte_add(0x114).cast::<i32>(),
                write_unaligned,
                y.wrapping_sub(edi)
            );
        }
        let ebp = ebp | 0x2000;
        mech_trace::wr!(
            "1005f530",
            this.byte_add(0x10c).cast::<u32>(),
            write_unaligned,
            ebp
        );
        mech_trace::wr!(
            "1005f530",
            this.byte_add(0x12c).cast::<u32>(),
            write_unaligned,
            0
        );
        let v = mech_trace::rd!("1005f530", hp.byte_add(0xc).cast::<u32>(), read_unaligned);
        mech_trace::wr!(
            "1005f530",
            this.byte_add(0x130).cast::<u32>(),
            write_unaligned,
            v
        );
        mech_trace::wr!(
            "1005f530",
            this.byte_add(0x128).cast::<u32>(),
            write_unaligned,
            0
        );
        let r;
        if ebp & 2 != 0 {
            mech_trace::wr!(
                "1005f530",
                this.byte_add(0x108).cast::<u32>(),
                write_unaligned,
                0
            );
            let p = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x140).cast::<usize>(),
                read_unaligned
            ) as *const u8;
            let y = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x114).cast::<u32>(),
                read_unaligned
            );
            let buf = mech_trace::rd!("1005f530", p.byte_add(0x42).cast::<u32>(), read_unaligned);
            let x = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x110).cast::<u32>(),
                read_unaligned
            );
            r = mech_trace::cr!(
                "1005f530",
                "SmackToBuffer",
                SmackToBuffer(ecx, x, y, 0x280, 0x1e0, buf, 0)
            );
        } else {
            let buf = mech_trace::cr!(
                "1005f530",
                "1006fab0",
                FUN_1006fab0(ebx.wrapping_mul(edi) as u32)
            );
            let h2 = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x11c).cast::<u32>(),
                read_unaligned
            );
            let w2 = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x118).cast::<u32>(),
                read_unaligned
            );
            mech_trace::wr!(
                "1005f530",
                this.byte_add(0x108).cast::<u32>(),
                write_unaligned,
                buf
            );
            let hh = mech_trace::rd!(
                "1005f530",
                this.byte_add(0x100).cast::<i32>(),
                read_unaligned
            );
            r = mech_trace::cr!(
                "1005f530",
                "SmackToBuffer",
                SmackToBuffer(hh, 0, 0, w2, h2, buf, 0)
            );
        }
        mech_trace::rt!("1005f530", (r & 0xffff_ff00) | 1)
    }
}
