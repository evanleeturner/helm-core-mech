// SPDX-License-Identifier: MIT
//! Copies the shell library's 549 ported functions out of `mech-shell` into
//! the build folder, each definition renamed from `FUN_<address>` to
//! `HC5_<address>`, and with it each call a port makes to itself (two
//! recurse: `10053590` and `10053780`). The game's original keeps the name
//! `FUN_<address>`, an absolute symbol at its own address in the swap's
//! link, so a port's call to another function reaches the game's entry,
//! and through it the swap where that entry is swapped. The ports are the modules
//! `mech-shell`'s `lib.rs` gates on `target_arch = "x86"`; RT1's 20 are
//! `ShellState` functions, not swappable, and stay out. Four of RT1's have a
//! swappable twin from HC6's route, gated as `swap_<address>` beside RT1's
//! `fun_<address>`; its copy is written as `fun_<address>` like the rest.
//!
//! Writes `ports.rs`, one gated `mod` line a port, for `lib.rs` to include.
//! Panics when a port does not define its own function exactly once.
use std::env;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest = env::var("CARGO_MANIFEST_DIR").expect("cargo sets CARGO_MANIFEST_DIR");
    let shell = PathBuf::from(manifest).join("../mech-shell/src");
    let out = PathBuf::from(env::var("OUT_DIR").expect("cargo sets OUT_DIR"));
    println!("cargo::rerun-if-changed={}", shell.display());
    let lib = fs::read_to_string(shell.join("lib.rs")).expect("mech-shell's lib.rs reads");
    let mut ports = String::new();
    let mut gated = false;
    for line in lib.lines().map(str::trim) {
        let module = line.strip_prefix("mod ").and_then(|m| m.strip_suffix(';'));
        if let (true, Some(module)) = (gated, module) {
            let address = module
                .strip_prefix("fun_")
                .or_else(|| module.strip_prefix("swap_"))
                .expect("a gated module is fun_<address> or swap_<address>");
            let source = fs::read_to_string(shell.join(format!("{module}.rs")))
                .expect("each gated module's file reads");
            let own = format!(" fn FUN_{address}(");
            assert_eq!(
                source.matches(&own).count(),
                1,
                "{module} defines FUN_{address} once"
            );
            let copy = out.join(format!("fun_{address}.rs"));
            fs::write(
                &copy,
                source.replace(&format!("FUN_{address}"), &format!("HC5_{address}")),
            )
            .expect("the build folder takes the copy");
            let _ = writeln!(
                ports,
                "#[cfg(target_arch = \"x86\")]\n#[path = \"{}\"]\nmod fun_{address};",
                copy.display()
            );
        }
        gated = line == "#[cfg(target_arch = \"x86\")]";
    }
    assert_eq!(
        ports.matches("\nmod ").count(),
        549,
        "mech-shell gates 549 ports"
    );
    fs::write(out.join("ports.rs"), ports).expect("the build folder takes ports.rs");
}
