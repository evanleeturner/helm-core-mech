// SPDX-License-Identifier: MIT
//! The shell library's functions, ported to Rust one module a function.
//!
//! Each port is taken from the decompile's Rust and keeps the original's
//! behavior: no fix. A port keeps its export's name, `FUN_<address>`, until
//! a name is approved for it. Two kinds sit side by side:
//!
//! - RT1's 20 (`rust/mw2shell/src/` at `02b7520`, judged against
//!   `MW2SHELL.DLL` under Wine), each global made a field of
//!   [`ShellState`], re-exported below as safe functions.
//! - The other 499: RT2 and RT3's 488 (`rust/` at `14b3e31`), copied
//!   unchanged, and HC6's 11, ported from mercs' rebuilt C by RT3's route;
//!   each exported as `FUN_<address>` with the original's calling
//!   convention, its globals and callees linked by name. Those conventions
//!   exist only on 32-bit x86, so each module's `mod` line carries
//!   `cfg(target_arch = "x86")`: on any other target the crate is RT1's 20
//!   alone. Build them with `--target i686-pc-windows-gnu`.
//!
//! Call: the 20 re-exported below; the game's globals any of them reads or
//! writes are the fields of [`ShellState`], passed by reference.
#![expect(
    non_snake_case,
    reason = "each port keeps its export's name, FUN_<address>, until a name is approved"
)]

#[cfg(target_arch = "x86")]
mod fun_100026c1;
#[cfg(target_arch = "x86")]
mod fun_100034c5;
#[cfg(target_arch = "x86")]
mod fun_100034e5;
#[cfg(target_arch = "x86")]
mod fun_100042c0;
#[cfg(target_arch = "x86")]
mod fun_10004330;
#[cfg(target_arch = "x86")]
mod fun_100043c0;
#[cfg(target_arch = "x86")]
mod fun_10004420;
#[cfg(target_arch = "x86")]
mod fun_10004550;
#[cfg(target_arch = "x86")]
mod fun_10004620;
#[cfg(target_arch = "x86")]
mod fun_10004660;
#[cfg(target_arch = "x86")]
mod fun_100046f0;
#[cfg(target_arch = "x86")]
mod fun_10004760;
#[cfg(target_arch = "x86")]
mod fun_100047f0;
#[cfg(target_arch = "x86")]
mod fun_10004800;
#[cfg(target_arch = "x86")]
mod fun_10004850;
#[cfg(target_arch = "x86")]
mod fun_10004960;
#[cfg(target_arch = "x86")]
mod fun_1000565c;
#[cfg(target_arch = "x86")]
mod fun_10005a60;
#[cfg(target_arch = "x86")]
mod fun_10006330;
#[cfg(target_arch = "x86")]
mod fun_100069e0;
#[cfg(target_arch = "x86")]
mod fun_10007690;
#[cfg(target_arch = "x86")]
mod fun_100076b0;
mod fun_10008680;
#[cfg(target_arch = "x86")]
mod fun_100092e0;
#[cfg(target_arch = "x86")]
mod fun_10009300;
#[cfg(target_arch = "x86")]
mod fun_10009370;
#[cfg(target_arch = "x86")]
mod fun_100094c0;
#[cfg(target_arch = "x86")]
mod fun_10009840;
#[cfg(target_arch = "x86")]
mod fun_100098d0;
#[cfg(target_arch = "x86")]
mod fun_10009e80;
#[cfg(target_arch = "x86")]
mod fun_10009f90;
#[cfg(target_arch = "x86")]
mod fun_10009fe0;
#[cfg(target_arch = "x86")]
mod fun_1000a030;
#[cfg(target_arch = "x86")]
mod fun_1000a060;
#[cfg(target_arch = "x86")]
mod fun_1000a0a0;
#[cfg(target_arch = "x86")]
mod fun_1000a120;
#[cfg(target_arch = "x86")]
mod fun_1000a140;
mod fun_1000a160;
#[cfg(target_arch = "x86")]
mod fun_1000a180;
#[cfg(target_arch = "x86")]
mod fun_1000a3a0;
#[cfg(target_arch = "x86")]
mod fun_1000a3f0;
#[cfg(target_arch = "x86")]
mod fun_1000a430;
#[cfg(target_arch = "x86")]
mod fun_1000a590;
#[cfg(target_arch = "x86")]
mod fun_1000a5d0;
#[cfg(target_arch = "x86")]
mod fun_1000a690;
mod fun_1000a6e0;
#[cfg(target_arch = "x86")]
mod fun_1000a6f0;
#[cfg(target_arch = "x86")]
mod fun_1000a790;
#[cfg(target_arch = "x86")]
mod fun_1000a800;
#[cfg(target_arch = "x86")]
mod fun_1000a890;
#[cfg(target_arch = "x86")]
mod fun_1000a8c0;
#[cfg(target_arch = "x86")]
mod fun_1000a910;
#[cfg(target_arch = "x86")]
mod fun_1000aa80;
#[cfg(target_arch = "x86")]
mod fun_1000ad30;
#[cfg(target_arch = "x86")]
mod fun_1000ad70;
#[cfg(target_arch = "x86")]
mod fun_1000adb0;
#[cfg(target_arch = "x86")]
mod fun_1000adf0;
#[cfg(target_arch = "x86")]
mod fun_1000ae30;
#[cfg(target_arch = "x86")]
mod fun_1000b7f0;
#[cfg(target_arch = "x86")]
mod fun_1000b8f0;
#[cfg(target_arch = "x86")]
mod fun_1000b940;
#[cfg(target_arch = "x86")]
mod fun_1000be10;
#[cfg(target_arch = "x86")]
mod fun_1000be30;
#[cfg(target_arch = "x86")]
mod fun_1000be60;
#[cfg(target_arch = "x86")]
mod fun_1000bf40;
#[cfg(target_arch = "x86")]
mod fun_1000bfd0;
#[cfg(target_arch = "x86")]
mod fun_1000bff0;
#[cfg(target_arch = "x86")]
mod fun_1000c220;
#[cfg(target_arch = "x86")]
mod fun_1000c250;
#[cfg(target_arch = "x86")]
mod fun_1000c2b0;
#[cfg(target_arch = "x86")]
mod fun_1000c410;
#[cfg(target_arch = "x86")]
mod fun_1000c4a0;
#[cfg(target_arch = "x86")]
mod fun_1000c4c0;
#[cfg(target_arch = "x86")]
mod fun_1000c4e0;
#[cfg(target_arch = "x86")]
mod fun_1000c5d0;
#[cfg(target_arch = "x86")]
mod fun_1000c6b0;
#[cfg(target_arch = "x86")]
mod fun_1000c990;
#[cfg(target_arch = "x86")]
mod fun_1000cb60;
#[cfg(target_arch = "x86")]
mod fun_1000cd20;
#[cfg(target_arch = "x86")]
mod fun_1000ce20;
#[cfg(target_arch = "x86")]
mod fun_1000ce60;
#[cfg(target_arch = "x86")]
mod fun_1000cec0;
#[cfg(target_arch = "x86")]
mod fun_1000cf60;
#[cfg(target_arch = "x86")]
mod fun_1000cfa0;
#[cfg(target_arch = "x86")]
mod fun_1000cfe0;
#[cfg(target_arch = "x86")]
mod fun_1000d000;
#[cfg(target_arch = "x86")]
mod fun_1000d230;
#[cfg(target_arch = "x86")]
mod fun_1000d260;
#[cfg(target_arch = "x86")]
mod fun_1000d3d0;
#[cfg(target_arch = "x86")]
mod fun_1000d440;
#[cfg(target_arch = "x86")]
mod fun_1000d4e0;
#[cfg(target_arch = "x86")]
mod fun_1000d5a0;
#[cfg(target_arch = "x86")]
mod fun_1000d5d0;
#[cfg(target_arch = "x86")]
mod fun_1000d620;
#[cfg(target_arch = "x86")]
mod fun_1000d650;
#[cfg(target_arch = "x86")]
mod fun_1000d6f0;
#[cfg(target_arch = "x86")]
mod fun_1000d7f0;
#[cfg(target_arch = "x86")]
mod fun_1000d880;
#[cfg(target_arch = "x86")]
mod fun_1000d9b0;
#[cfg(target_arch = "x86")]
mod fun_1000de00;
#[cfg(target_arch = "x86")]
mod fun_1000dee0;
#[cfg(target_arch = "x86")]
mod fun_1000df80;
#[cfg(target_arch = "x86")]
mod fun_1000e070;
#[cfg(target_arch = "x86")]
mod fun_1000e150;
#[cfg(target_arch = "x86")]
mod fun_1000e1a0;
#[cfg(target_arch = "x86")]
mod fun_1000e330;
#[cfg(target_arch = "x86")]
mod fun_1000e6d0;
#[cfg(target_arch = "x86")]
mod fun_1000e730;
#[cfg(target_arch = "x86")]
mod fun_1000e790;
#[cfg(target_arch = "x86")]
mod fun_1000e8f0;
#[cfg(target_arch = "x86")]
mod fun_1000e970;
#[cfg(target_arch = "x86")]
mod fun_1000ece0;
#[cfg(target_arch = "x86")]
mod fun_1000ed70;
#[cfg(target_arch = "x86")]
mod fun_1000ee30;
#[cfg(target_arch = "x86")]
mod fun_1000f110;
#[cfg(target_arch = "x86")]
mod fun_1000f170;
#[cfg(target_arch = "x86")]
mod fun_1000f1b0;
#[cfg(target_arch = "x86")]
mod fun_1000f360;
#[cfg(target_arch = "x86")]
mod fun_1000f640;
#[cfg(target_arch = "x86")]
mod fun_1000f6d0;
#[cfg(target_arch = "x86")]
mod fun_1000f8d0;
#[cfg(target_arch = "x86")]
mod fun_1000f940;
#[cfg(target_arch = "x86")]
mod fun_1000fce0;
#[cfg(target_arch = "x86")]
mod fun_1000fd70;
#[cfg(target_arch = "x86")]
mod fun_1000fdb0;
#[cfg(target_arch = "x86")]
mod fun_1000fe40;
mod fun_10010920;
mod fun_10010930;
#[cfg(target_arch = "x86")]
mod fun_100110d0;
#[cfg(target_arch = "x86")]
mod fun_10011170;
#[cfg(target_arch = "x86")]
mod fun_100115c0;
#[cfg(target_arch = "x86")]
mod fun_100115d0;
#[cfg(target_arch = "x86")]
mod fun_10011670;
#[cfg(target_arch = "x86")]
mod fun_100116b0;
#[cfg(target_arch = "x86")]
mod fun_10011d20;
#[cfg(target_arch = "x86")]
mod fun_10012220;
#[cfg(target_arch = "x86")]
mod fun_10012270;
#[cfg(target_arch = "x86")]
mod fun_10012590;
#[cfg(target_arch = "x86")]
mod fun_10012620;
#[cfg(target_arch = "x86")]
mod fun_10013100;
#[cfg(target_arch = "x86")]
mod fun_10013380;
#[cfg(target_arch = "x86")]
mod fun_10013420;
#[cfg(target_arch = "x86")]
mod fun_100164f0;
#[cfg(target_arch = "x86")]
mod fun_10016590;
#[cfg(target_arch = "x86")]
mod fun_10016620;
#[cfg(target_arch = "x86")]
mod fun_100166c0;
#[cfg(target_arch = "x86")]
mod fun_10016740;
mod fun_10016b50;
#[cfg(target_arch = "x86")]
mod fun_10016b60;
#[cfg(target_arch = "x86")]
mod fun_10016ba0;
#[cfg(target_arch = "x86")]
mod fun_10016bd0;
#[cfg(target_arch = "x86")]
mod fun_10016c20;
mod fun_10016c90;
#[cfg(target_arch = "x86")]
mod fun_10016ca0;
#[cfg(target_arch = "x86")]
mod fun_10016cd0;
#[cfg(target_arch = "x86")]
mod fun_10016cf0;
#[cfg(target_arch = "x86")]
mod fun_10016d40;
#[cfg(target_arch = "x86")]
mod fun_100175e0;
#[cfg(target_arch = "x86")]
mod fun_10017a40;
#[cfg(target_arch = "x86")]
mod fun_10017b70;
#[cfg(target_arch = "x86")]
mod fun_10017c90;
#[cfg(target_arch = "x86")]
mod fun_10017d70;
#[cfg(target_arch = "x86")]
mod fun_10017df0;
#[cfg(target_arch = "x86")]
mod fun_10017ec0;
#[cfg(target_arch = "x86")]
mod fun_10017f30;
#[cfg(target_arch = "x86")]
mod fun_10017ff0;
#[cfg(target_arch = "x86")]
mod fun_10018020;
#[cfg(target_arch = "x86")]
mod fun_10018060;
#[cfg(target_arch = "x86")]
mod fun_10018090;
#[cfg(target_arch = "x86")]
mod fun_100180f0;
#[cfg(target_arch = "x86")]
mod fun_100181d0;
#[cfg(target_arch = "x86")]
mod fun_10018b10;
#[cfg(target_arch = "x86")]
mod fun_10019aa0;
#[cfg(target_arch = "x86")]
mod fun_10019af0;
#[cfg(target_arch = "x86")]
mod fun_1001a480;
#[cfg(target_arch = "x86")]
mod fun_1001a730;
#[cfg(target_arch = "x86")]
mod fun_1001a910;
#[cfg(target_arch = "x86")]
mod fun_1001a980;
#[cfg(target_arch = "x86")]
mod fun_1001add0;
#[cfg(target_arch = "x86")]
mod fun_1001af40;
#[cfg(target_arch = "x86")]
mod fun_1001b680;
#[cfg(target_arch = "x86")]
mod fun_1001baa0;
#[cfg(target_arch = "x86")]
mod fun_1001c310;
#[cfg(target_arch = "x86")]
mod fun_1001d130;
#[cfg(target_arch = "x86")]
mod fun_1001d450;
#[cfg(target_arch = "x86")]
mod fun_1001d650;
#[cfg(target_arch = "x86")]
mod fun_1001d810;
#[cfg(target_arch = "x86")]
mod fun_1001d840;
#[cfg(target_arch = "x86")]
mod fun_1001da50;
#[cfg(target_arch = "x86")]
mod fun_1001db00;
#[cfg(target_arch = "x86")]
mod fun_1001e6b0;
mod fun_1002f31a;
#[cfg(target_arch = "x86")]
mod fun_1002f337;
#[cfg(target_arch = "x86")]
mod fun_1002f35f;
#[cfg(target_arch = "x86")]
mod fun_1002f408;
#[cfg(target_arch = "x86")]
mod fun_1002fea0;
#[cfg(target_arch = "x86")]
mod fun_1002ff60;
#[cfg(target_arch = "x86")]
mod fun_10030090;
#[cfg(target_arch = "x86")]
mod fun_100300f0;
#[cfg(target_arch = "x86")]
mod fun_100304b0;
#[cfg(target_arch = "x86")]
mod fun_10031000;
#[cfg(target_arch = "x86")]
mod fun_100315b0;
#[cfg(target_arch = "x86")]
mod fun_10031890;
#[cfg(target_arch = "x86")]
mod fun_100318e0;
#[cfg(target_arch = "x86")]
mod fun_10031910;
#[cfg(target_arch = "x86")]
mod fun_10031950;
#[cfg(target_arch = "x86")]
mod fun_10031980;
#[cfg(target_arch = "x86")]
mod fun_100319b0;
#[cfg(target_arch = "x86")]
mod fun_10031a00;
#[cfg(target_arch = "x86")]
mod fun_10031b20;
#[cfg(target_arch = "x86")]
mod fun_10031c40;
#[cfg(target_arch = "x86")]
mod fun_10031de0;
#[cfg(target_arch = "x86")]
mod fun_100323a0;
#[cfg(target_arch = "x86")]
mod fun_10032640;
#[cfg(target_arch = "x86")]
mod fun_100328f0;
#[cfg(target_arch = "x86")]
mod fun_100329e0;
#[cfg(target_arch = "x86")]
mod fun_10032a90;
#[cfg(target_arch = "x86")]
mod fun_10032b60;
#[cfg(target_arch = "x86")]
mod fun_10032c10;
#[cfg(target_arch = "x86")]
mod fun_10032ce0;
#[cfg(target_arch = "x86")]
mod fun_10032e00;
#[cfg(target_arch = "x86")]
mod fun_10032e50;
#[cfg(target_arch = "x86")]
mod fun_10032f60;
#[cfg(target_arch = "x86")]
mod fun_10032fc0;
#[cfg(target_arch = "x86")]
mod fun_10033e80;
#[cfg(target_arch = "x86")]
mod fun_10033f40;
#[cfg(target_arch = "x86")]
mod fun_10035cf0;
#[cfg(target_arch = "x86")]
mod fun_10035d10;
#[cfg(target_arch = "x86")]
mod fun_10035d50;
mod fun_10035d90;
#[cfg(target_arch = "x86")]
mod fun_10035da0;
#[cfg(target_arch = "x86")]
mod fun_10035e10;
#[cfg(target_arch = "x86")]
mod fun_10035e60;
#[cfg(target_arch = "x86")]
mod fun_10035e90;
#[cfg(target_arch = "x86")]
mod fun_10035eb0;
#[cfg(target_arch = "x86")]
mod fun_10035ef0;
#[cfg(target_arch = "x86")]
mod fun_10035f30;
#[cfg(target_arch = "x86")]
mod fun_10036100;
#[cfg(target_arch = "x86")]
mod fun_10036130;
#[cfg(target_arch = "x86")]
mod fun_10036180;
#[cfg(target_arch = "x86")]
mod fun_100361a0;
#[cfg(target_arch = "x86")]
mod fun_100361c0;
#[cfg(target_arch = "x86")]
mod fun_100361e0;
#[cfg(target_arch = "x86")]
mod fun_10036260;
#[cfg(target_arch = "x86")]
mod fun_10036280;
#[cfg(target_arch = "x86")]
mod fun_10036340;
#[cfg(target_arch = "x86")]
mod fun_100363c0;
#[cfg(target_arch = "x86")]
mod fun_100364f0;
#[cfg(target_arch = "x86")]
mod fun_10036540;
#[cfg(target_arch = "x86")]
mod fun_10036660;
#[cfg(target_arch = "x86")]
mod fun_100366c0;
#[cfg(target_arch = "x86")]
mod fun_10036780;
#[cfg(target_arch = "x86")]
mod fun_10037c90;
#[cfg(target_arch = "x86")]
mod fun_10037ce0;
#[cfg(target_arch = "x86")]
mod fun_10037d40;
#[cfg(target_arch = "x86")]
mod fun_10037e50;
#[cfg(target_arch = "x86")]
mod fun_10037ed0;
#[cfg(target_arch = "x86")]
mod fun_10037fc0;
#[cfg(target_arch = "x86")]
mod fun_10038160;
#[cfg(target_arch = "x86")]
mod fun_10039270;
#[cfg(target_arch = "x86")]
mod fun_100396a0;
#[cfg(target_arch = "x86")]
mod fun_1003a460;
#[cfg(target_arch = "x86")]
mod fun_1003a4d0;
#[cfg(target_arch = "x86")]
mod fun_1003a560;
#[cfg(target_arch = "x86")]
mod fun_1003a5c0;
#[cfg(target_arch = "x86")]
mod fun_1003a5e0;
#[cfg(target_arch = "x86")]
mod fun_1003a640;
#[cfg(target_arch = "x86")]
mod fun_1003b570;
#[cfg(target_arch = "x86")]
mod fun_1003bc40;
#[cfg(target_arch = "x86")]
mod fun_1003c1e0;
#[cfg(target_arch = "x86")]
mod fun_1003cc10;
#[cfg(target_arch = "x86")]
mod fun_1003cc40;
#[cfg(target_arch = "x86")]
mod fun_1003cc60;
#[cfg(target_arch = "x86")]
mod fun_1003ddc0;
#[cfg(target_arch = "x86")]
mod fun_1003f8f0;
#[cfg(target_arch = "x86")]
mod fun_1003fd30;
#[cfg(target_arch = "x86")]
mod fun_10040270;
mod fun_100406b0;
#[cfg(target_arch = "x86")]
mod fun_10040af0;
#[cfg(target_arch = "x86")]
mod fun_10040b50;
#[cfg(target_arch = "x86")]
mod fun_100416d0;
#[cfg(target_arch = "x86")]
mod fun_10041b10;
#[cfg(target_arch = "x86")]
mod fun_100421c0;
#[cfg(target_arch = "x86")]
mod fun_10042210;
#[cfg(target_arch = "x86")]
mod fun_100422e0;
#[cfg(target_arch = "x86")]
mod fun_10042370;
#[cfg(target_arch = "x86")]
mod fun_100423d0;
#[cfg(target_arch = "x86")]
mod fun_10042590;
#[cfg(target_arch = "x86")]
mod fun_10042730;
#[cfg(target_arch = "x86")]
mod fun_100428e0;
#[cfg(target_arch = "x86")]
mod fun_100430b0;
#[cfg(target_arch = "x86")]
mod fun_10043110;
#[cfg(target_arch = "x86")]
mod fun_10043150;
#[cfg(target_arch = "x86")]
mod fun_100431c0;
#[cfg(target_arch = "x86")]
mod fun_10043fb0;
#[cfg(target_arch = "x86")]
mod fun_10044060;
#[cfg(target_arch = "x86")]
mod fun_10044940;
#[cfg(target_arch = "x86")]
mod fun_10044970;
#[cfg(target_arch = "x86")]
mod fun_100449a0;
#[cfg(target_arch = "x86")]
mod fun_10044fd0;
#[cfg(target_arch = "x86")]
mod fun_10046720;
#[cfg(target_arch = "x86")]
mod fun_10046810;
#[cfg(target_arch = "x86")]
mod fun_10046900;
mod fun_1004b060;
#[cfg(target_arch = "x86")]
mod fun_1004b0b0;
#[cfg(target_arch = "x86")]
mod fun_1004b0f0;
#[cfg(target_arch = "x86")]
mod fun_1004b140;
#[cfg(target_arch = "x86")]
mod fun_1004b180;
#[cfg(target_arch = "x86")]
mod fun_1004b200;
#[cfg(target_arch = "x86")]
mod fun_1004c000;
#[cfg(target_arch = "x86")]
mod fun_1004c050;
#[cfg(target_arch = "x86")]
mod fun_1004c110;
#[cfg(target_arch = "x86")]
mod fun_1004c220;
#[cfg(target_arch = "x86")]
mod fun_1004c2d0;
#[cfg(target_arch = "x86")]
mod fun_1004c460;
#[cfg(target_arch = "x86")]
mod fun_1004c520;
#[cfg(target_arch = "x86")]
mod fun_1004c6d0;
#[cfg(target_arch = "x86")]
mod fun_1004c930;
#[cfg(target_arch = "x86")]
mod fun_1004c9b0;
#[cfg(target_arch = "x86")]
mod fun_1004cc80;
#[cfg(target_arch = "x86")]
mod fun_1004cec0;
#[cfg(target_arch = "x86")]
mod fun_1004d0a0;
#[cfg(target_arch = "x86")]
mod fun_1004db80;
#[cfg(target_arch = "x86")]
mod fun_1004dbe0;
#[cfg(target_arch = "x86")]
mod fun_1004e850;
#[cfg(target_arch = "x86")]
mod fun_1004e9e0;
#[cfg(target_arch = "x86")]
mod fun_1004eb30;
#[cfg(target_arch = "x86")]
mod fun_1004f0f0;
#[cfg(target_arch = "x86")]
mod fun_1004f110;
#[cfg(target_arch = "x86")]
mod fun_1004f130;
#[cfg(target_arch = "x86")]
mod fun_1004f180;
#[cfg(target_arch = "x86")]
mod fun_1004f1e0;
mod fun_1004f240;
#[cfg(target_arch = "x86")]
mod fun_1004f2c0;
#[cfg(target_arch = "x86")]
mod fun_1004f4f0;
#[cfg(target_arch = "x86")]
mod fun_10050390;
#[cfg(target_arch = "x86")]
mod fun_100503d0;
#[cfg(target_arch = "x86")]
mod fun_10050400;
#[cfg(target_arch = "x86")]
mod fun_10050490;
#[cfg(target_arch = "x86")]
mod fun_100504a0;
#[cfg(target_arch = "x86")]
mod fun_10050a10;
#[cfg(target_arch = "x86")]
mod fun_10050a30;
mod fun_10051650;
#[cfg(target_arch = "x86")]
mod fun_10051700;
#[cfg(target_arch = "x86")]
mod fun_10051950;
#[cfg(target_arch = "x86")]
mod fun_100530ae;
#[cfg(target_arch = "x86")]
mod fun_10053470;
#[cfg(target_arch = "x86")]
mod fun_10053500;
#[cfg(target_arch = "x86")]
mod fun_10053590;
#[cfg(target_arch = "x86")]
mod fun_10053630;
#[cfg(target_arch = "x86")]
mod fun_10053780;
#[cfg(target_arch = "x86")]
mod fun_10053a40;
#[cfg(target_arch = "x86")]
mod fun_10053cf0;
#[cfg(target_arch = "x86")]
mod fun_10053df0;
#[cfg(target_arch = "x86")]
mod fun_10053fc0;
#[cfg(target_arch = "x86")]
mod fun_10054000;
#[cfg(target_arch = "x86")]
mod fun_100540d0;
#[cfg(target_arch = "x86")]
mod fun_10054270;
#[cfg(target_arch = "x86")]
mod fun_10054460;
#[cfg(target_arch = "x86")]
mod fun_10054b30;
mod fun_10055340;
#[cfg(target_arch = "x86")]
mod fun_10055350;
mod fun_10055ae0;
#[cfg(target_arch = "x86")]
mod fun_10055bf0;
#[cfg(target_arch = "x86")]
mod fun_10056bb0;
#[cfg(target_arch = "x86")]
mod fun_10056be0;
#[cfg(target_arch = "x86")]
mod fun_10056c20;
#[cfg(target_arch = "x86")]
mod fun_10056c60;
#[cfg(target_arch = "x86")]
mod fun_10056dd0;
#[cfg(target_arch = "x86")]
mod fun_10057200;
#[cfg(target_arch = "x86")]
mod fun_10057440;
#[cfg(target_arch = "x86")]
mod fun_10057530;
#[cfg(target_arch = "x86")]
mod fun_100575d0;
#[cfg(target_arch = "x86")]
mod fun_100576a0;
#[cfg(target_arch = "x86")]
mod fun_100576d0;
#[cfg(target_arch = "x86")]
mod fun_100578d0;
#[cfg(target_arch = "x86")]
mod fun_10057a40;
#[cfg(target_arch = "x86")]
mod fun_100580a0;
#[cfg(target_arch = "x86")]
mod fun_10058400;
#[cfg(target_arch = "x86")]
mod fun_100596b0;
#[cfg(target_arch = "x86")]
mod fun_100596e0;
mod fun_10059870;
mod fun_10059e60;
#[cfg(target_arch = "x86")]
mod fun_10059e80;
#[cfg(target_arch = "x86")]
mod fun_10059ea0;
#[cfg(target_arch = "x86")]
mod fun_10059f10;
#[cfg(target_arch = "x86")]
mod fun_1005a180;
#[cfg(target_arch = "x86")]
mod fun_1005a230;
#[cfg(target_arch = "x86")]
mod fun_1005a2d0;
#[cfg(target_arch = "x86")]
mod fun_1005a3c0;
#[cfg(target_arch = "x86")]
mod fun_1005a570;
#[cfg(target_arch = "x86")]
mod fun_1005aaf0;
#[cfg(target_arch = "x86")]
mod fun_1005ab30;
#[cfg(target_arch = "x86")]
mod fun_1005ab70;
#[cfg(target_arch = "x86")]
mod fun_1005abb0;
#[cfg(target_arch = "x86")]
mod fun_1005af20;
#[cfg(target_arch = "x86")]
mod fun_1005b1b0;
#[cfg(target_arch = "x86")]
mod fun_1005b3f0;
mod fun_1005b5f0;
#[cfg(target_arch = "x86")]
mod fun_1005b600;
#[cfg(target_arch = "x86")]
mod fun_1005b6c0;
#[cfg(target_arch = "x86")]
mod fun_1005b710;
#[cfg(target_arch = "x86")]
mod fun_1005b830;
#[cfg(target_arch = "x86")]
mod fun_1005b8e0;
#[cfg(target_arch = "x86")]
mod fun_1005ba70;
#[cfg(target_arch = "x86")]
mod fun_1005bb20;
#[cfg(target_arch = "x86")]
mod fun_1005bbb0;
#[cfg(target_arch = "x86")]
mod fun_1005bc00;
#[cfg(target_arch = "x86")]
mod fun_1005e020;
#[cfg(target_arch = "x86")]
mod fun_1005e170;
#[cfg(target_arch = "x86")]
mod fun_1005e200;
#[cfg(target_arch = "x86")]
mod fun_1005e290;
#[cfg(target_arch = "x86")]
mod fun_1005e400;
#[cfg(target_arch = "x86")]
mod fun_1005e670;
#[cfg(target_arch = "x86")]
mod fun_1005e7f0;
#[cfg(target_arch = "x86")]
mod fun_1005e870;
#[cfg(target_arch = "x86")]
mod fun_1005e910;
#[cfg(target_arch = "x86")]
mod fun_1005efc0;
#[cfg(target_arch = "x86")]
mod fun_1005f120;
#[cfg(target_arch = "x86")]
mod fun_1005f8c0;
#[cfg(target_arch = "x86")]
mod fun_1005f8e0;
#[cfg(target_arch = "x86")]
mod fun_1005f900;
#[cfg(target_arch = "x86")]
mod fun_1005f920;
#[cfg(target_arch = "x86")]
mod fun_1005fb90;
#[cfg(target_arch = "x86")]
mod fun_10060010;
#[cfg(target_arch = "x86")]
mod fun_10060070;
#[cfg(target_arch = "x86")]
mod fun_100601fe;
#[cfg(target_arch = "x86")]
mod fun_100603be;
#[cfg(target_arch = "x86")]
mod fun_1006074e;
#[cfg(target_arch = "x86")]
mod fun_100609d0;
#[cfg(target_arch = "x86")]
mod fun_10060b90;
#[cfg(target_arch = "x86")]
mod fun_10060bc0;
#[cfg(target_arch = "x86")]
mod fun_10061350;
#[cfg(target_arch = "x86")]
mod fun_10062400;
#[cfg(target_arch = "x86")]
mod fun_10062430;
#[cfg(target_arch = "x86")]
mod fun_100624a0;
#[cfg(target_arch = "x86")]
mod fun_10062520;
#[cfg(target_arch = "x86")]
mod fun_10062bf0;
#[cfg(target_arch = "x86")]
mod fun_10063d30;
#[cfg(target_arch = "x86")]
mod fun_10063de0;
#[cfg(target_arch = "x86")]
mod fun_10063ea0;
#[cfg(target_arch = "x86")]
mod fun_10063ee0;
#[cfg(target_arch = "x86")]
mod fun_10063f00;
#[cfg(target_arch = "x86")]
mod fun_10063f40;
#[cfg(target_arch = "x86")]
mod fun_10063f70;
#[cfg(target_arch = "x86")]
mod fun_100640a0;
mod fun_100640f0;
#[cfg(target_arch = "x86")]
mod fun_100648b0;
#[cfg(target_arch = "x86")]
mod fun_10064e00;
#[cfg(target_arch = "x86")]
mod fun_10064e59;
#[cfg(target_arch = "x86")]
mod fun_10064e78;
#[cfg(target_arch = "x86")]
mod fun_10065a2b;
#[cfg(target_arch = "x86")]
mod fun_10065b64;
#[cfg(target_arch = "x86")]
mod fun_10065f68;
#[cfg(target_arch = "x86")]
mod fun_1006602f;
#[cfg(target_arch = "x86")]
mod fun_1006604e;
#[cfg(target_arch = "x86")]
mod fun_1006606a;
#[cfg(target_arch = "x86")]
mod fun_10066542;
#[cfg(target_arch = "x86")]
mod fun_10067221;
#[cfg(target_arch = "x86")]
mod fun_10067399;
#[cfg(target_arch = "x86")]
mod fun_1006761c;
#[cfg(target_arch = "x86")]
mod fun_100676ae;
#[cfg(target_arch = "x86")]
mod fun_10067837;
#[cfg(target_arch = "x86")]
mod fun_10067a14;
#[cfg(target_arch = "x86")]
mod fun_10067af3;
#[cfg(target_arch = "x86")]
mod fun_10067e8f;
#[cfg(target_arch = "x86")]
mod fun_1006808c;
#[cfg(target_arch = "x86")]
mod fun_100683cd;
#[cfg(target_arch = "x86")]
mod fun_100694df;
#[cfg(target_arch = "x86")]
mod fun_10069597;
#[cfg(target_arch = "x86")]
mod fun_100695bd;
#[cfg(target_arch = "x86")]
mod fun_10069684;
#[cfg(target_arch = "x86")]
mod fun_10069697;
#[cfg(target_arch = "x86")]
mod fun_100696b7;
#[cfg(target_arch = "x86")]
mod fun_1006984a;
#[cfg(target_arch = "x86")]
mod fun_10069881;
#[cfg(target_arch = "x86")]
mod fun_10069990;
#[cfg(target_arch = "x86")]
mod fun_100699d2;
#[cfg(target_arch = "x86")]
mod fun_10069b99;
#[cfg(target_arch = "x86")]
mod fun_10069bca;
#[cfg(target_arch = "x86")]
mod fun_10069bf7;
#[cfg(target_arch = "x86")]
mod fun_10069c79;
#[cfg(target_arch = "x86")]
mod fun_10069ca4;
#[cfg(target_arch = "x86")]
mod fun_10069d13;
#[cfg(target_arch = "x86")]
mod fun_10069db8;
#[cfg(target_arch = "x86")]
mod fun_10069e35;
#[cfg(target_arch = "x86")]
mod fun_1006a04e;
#[cfg(target_arch = "x86")]
mod fun_1006a0af;
#[cfg(target_arch = "x86")]
mod fun_1006a0e7;
#[cfg(target_arch = "x86")]
mod fun_1006a109;
#[cfg(target_arch = "x86")]
mod fun_1006a12c;
#[cfg(target_arch = "x86")]
mod fun_1006a160;
#[cfg(target_arch = "x86")]
mod fun_1006a18a;
#[cfg(target_arch = "x86")]
mod fun_1006a1d5;
#[cfg(target_arch = "x86")]
mod fun_1006a21d;
#[cfg(target_arch = "x86")]
mod fun_1006a267;
#[cfg(target_arch = "x86")]
mod fun_1006a27a;
#[cfg(target_arch = "x86")]
mod fun_1006a2dc;
#[cfg(target_arch = "x86")]
mod fun_1006a631;
#[cfg(target_arch = "x86")]
mod fun_1006a7b5;
#[cfg(target_arch = "x86")]
mod fun_1006b230;
#[cfg(target_arch = "x86")]
mod fun_1006b300;
#[cfg(target_arch = "x86")]
mod fun_1006b380;
#[cfg(target_arch = "x86")]
mod fun_1006b3f0;
#[cfg(target_arch = "x86")]
mod fun_1006b420;
#[cfg(target_arch = "x86")]
mod fun_1006b450;
#[cfg(target_arch = "x86")]
mod fun_1006b500;
#[cfg(target_arch = "x86")]
mod fun_1006b9c0;
#[cfg(target_arch = "x86")]
mod fun_1006c150;
#[cfg(target_arch = "x86")]
mod fun_1006cf40;
#[cfg(target_arch = "x86")]
mod fun_1006cf90;
#[cfg(target_arch = "x86")]
mod fun_1006cfd0;
mod fun_1006d130;
#[cfg(target_arch = "x86")]
mod fun_1006d140;
#[cfg(target_arch = "x86")]
mod fun_1006d9e0;
#[cfg(target_arch = "x86")]
mod fun_1006dbc0;
#[cfg(target_arch = "x86")]
mod fun_1006dc00;
#[cfg(target_arch = "x86")]
mod fun_1006dfd0;
#[cfg(target_arch = "x86")]
mod fun_1006e020;
mod state;
#[cfg(test)]
mod test_log;

pub use fun_1000a6e0::FUN_1000a6e0;
pub use fun_1000a160::FUN_1000a160;
pub use fun_1002f31a::FUN_1002f31a;
pub use fun_1004b060::FUN_1004b060;
pub use fun_1004f240::FUN_1004f240;
pub use fun_1005b5f0::FUN_1005b5f0;
pub use fun_1006d130::FUN_1006d130;
pub use fun_10016b50::FUN_10016b50;
pub use fun_10016c90::FUN_10016c90;
pub use fun_10035d90::FUN_10035d90;
pub use fun_10055ae0::FUN_10055ae0;
pub use fun_10059e60::FUN_10059e60;
pub use fun_100406b0::FUN_100406b0;
pub use fun_100640f0::FUN_100640f0;
pub use fun_10008680::FUN_10008680;
pub use fun_10010920::FUN_10010920;
pub use fun_10010930::FUN_10010930;
pub use fun_10051650::FUN_10051650;
pub use fun_10055340::FUN_10055340;
pub use fun_10059870::FUN_10059870;
pub use state::ShellState;
