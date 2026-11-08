// SPDX-License-Identifier: MIT
//! The swap's rows a start cannot go without when the pieces are off: the exports the bind
//! keeps at their fronts, and the one row whose answer stays ours while the original draws.

/// The exports kept at their fronts with the pieces off. Each calls the real `LoadLibraryA`,
/// then binds the file it loaded, so without them a loaded file's slots are never bound at
/// all. Which of the 3 loads which file is not read, so all 3 stay: a guess, named.
pub const KEPT: &[&str] = &["LAUNCHER_LoadLibraryA", "HC6_LoadLibraryA", "GAME_LoadLibraryA"];

/// The export whose answer stays ours with the pieces off. `MW2SHELL.DLL`'s set-mode (the
/// layer's `draw` reads its return at `0x1000_dfae`) takes `BitBlt`'s answer as the screen's
/// height and quits with 2 on the original's 1; so the original draws, and the height is ours.
pub const HEIGHT: &str = "HC6_BitBlt";

#[cfg(test)]
mod tests {
    use super::*;

    /// The bind puts a kept export at its front before it asks for the height row, so a
    /// height export on the kept list would never be routed as the height.
    #[test]
    fn the_height_export_is_not_a_kept_one() {
        assert!(!KEPT.contains(&HEIGHT));
        assert!(KEPT.iter().all(|export| export.ends_with("_LoadLibraryA")));
    }
}
