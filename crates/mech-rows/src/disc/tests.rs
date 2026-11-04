// SPDX-License-Identifier: MIT
//! Falsifier 2 (DS2), the player's image by its path: mercs's disc as the
//! player keeps it, `~/games/mech2merc`, read where it lies and never
//! copied. `Track 01.cue` with its 22 bins opens, and
//! `mech2merc-data.iso` alone. Every file on the image's data track,
//! walked through the `Folder` finds with its folders, reads equal byte
//! for byte to the iso's, and to `disc/`'s, the data track unpacked by
//! another tool (10-23), a witness the volume's reader did not make. No
//! hash crate is in the lock: byte equality says what a sha256 match
//! would. Its tracks are [`DISC`]'s 21, 2 to 22, 199,430 frames past
//! their pregaps (the bins' sizes, measured 11-03), 44.32 min; each
//! track's first, middle and last frame read equal to its bin's: a
//! track's first frames are silence, so a start one frame off reads the
//! same there. Skipped with a notice when the image is absent.
use std::collections::BTreeMap;
use std::ffi::CString;
use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};

use kernel_files::{Create, Error, Folder, Mode};
use logical_disc::{Audio, Disc, ReadAt, Source};

use crate::disc::DISC;

/// A raw sector's bytes (ECMA-130), as a buffer's length.
const FRAME: usize = 2352;
/// The same, as a byte's place.
const RAW: u64 = FRAME as u64;
/// Each audio bin's pregap, `INDEX 01 00:01:74` in the sheet: 1 s and 74
/// frames.
const PREGAP: u64 = 149;
/// A read's buffer: a file's bytes, a piece at a time.
const PIECE: usize = 1 << 16;

/// An open to read a file that is there.
const READ: Mode = Mode {
    read: true,
    write: false,
    create: Create::Existing,
};

/// A source file on the host, read by range (`pread`).
struct HostFile {
    file: File,
    size: u64,
}

impl ReadAt for HostFile {
    fn size(&self) -> u64 {
        self.size
    }

    fn read_at(&self, at: u64, into: &mut [u8]) -> Result<(), Error> {
        if at
            .checked_add(into.len() as u64)
            .is_none_or(|end| end > self.size)
        {
            return Err(Error::BadArgument);
        }
        self.file.read_exact_at(into, at).map_err(|error| {
            Error::Other(
                error
                    .raw_os_error()
                    .and_then(|code| u32::try_from(code).ok())
                    .unwrap_or(0),
            )
        })
    }
}

fn host(path: &Path) -> HostFile {
    let file = File::open(path).expect("the player's file opens");
    let size = file.metadata().expect("its size").len();
    HostFile { file, size }
}

/// The player's image folder, when this box holds it.
fn player_image() -> Option<PathBuf> {
    let folder = PathBuf::from(std::env::var_os("HOME")?).join("games/mech2merc");
    folder.join("Track 01.cue").is_file().then_some(folder)
}

/// Every file under `folder` (empty at the volume's top), by its name
/// from the top, folders recursed: a find's name that will not open as a
/// file is a folder.
fn walk(disc: &impl Folder, folder: &str, files: &mut Vec<String>) {
    let pattern = if folder.is_empty() {
        "*".to_string()
    } else {
        format!("{folder}\\*")
    };
    let pattern = CString::new(pattern).expect("a name");
    let mut name = [0u8; 256];
    let mut names = Vec::new();
    let find = match disc.find_first(&pattern, &mut name) {
        Ok(find) => find,
        Err(Error::NotFound) => return,
        Err(error) => panic!("{folder}'s find fails {error:?}"),
    };
    loop {
        let found = std::ffi::CStr::from_bytes_until_nul(&name).expect("a NUL");
        names.push(found.to_str().expect("an ISO 9660 name").to_string());
        match disc.find_next(find, &mut name) {
            Ok(()) => {}
            Err(Error::NoMore) => break,
            Err(error) => panic!("{folder}'s find fails {error:?}"),
        }
    }
    disc.find_close(find).expect("the find closes");
    for each in names {
        let path = if folder.is_empty() {
            each
        } else {
            format!("{folder}\\{each}")
        };
        let asked = CString::new(path.as_str()).expect("a name");
        match disc.open(&asked, READ) {
            Ok(file) => {
                disc.close(file).expect("it closes");
                files.push(path);
            }
            Err(Error::Denied) => walk(disc, &path, files),
            Err(error) => panic!("{path} opens {error:?}"),
        }
    }
}

/// The witness's files, by their names from its top, `\` between parts,
/// case folded as names compare.
fn witnessed(top: &Path, at: &Path, files: &mut BTreeMap<String, PathBuf>) {
    for entry in std::fs::read_dir(at).expect("the witness reads") {
        let path = entry.expect("an entry").path();
        if path.is_dir() {
            witnessed(top, &path, files);
        } else {
            let name = path.strip_prefix(top).expect("under the top");
            let name = name.to_str().expect("a name").replace('/', "\\");
            files.insert(name.to_ascii_lowercase(), path);
        }
    }
}

/// A file's bytes through a disc, a piece at a time, into `each`.
fn pieces(disc: &impl Folder, name: &str, mut each: impl FnMut(&[u8])) {
    let asked = CString::new(name).expect("a name");
    let file = disc.open(&asked, READ).expect("it opens");
    let mut piece = vec![0; PIECE];
    loop {
        let count = disc.read(file, &mut piece).expect("it reads");
        if count == 0 {
            break;
        }
        each(&piece[..count as usize]);
    }
    disc.close(file).expect("it closes");
}

/// A file's bytes through a disc, whole: the largest is 37M.
fn bytes(disc: &impl Folder, name: &str) -> Vec<u8> {
    let mut whole = Vec::new();
    pieces(disc, name, |piece| whole.extend_from_slice(piece));
    whole
}

#[test]
fn the_players_image_reads_as_its_iso_and_its_tracks() {
    let Some(folder) = player_image() else {
        eprintln!("NOTICE: no ~/games/mech2merc on this box; falsifier 2 skipped");
        return;
    };
    let files = (1..=22)
        .map(|number| {
            let name = format!("Track {number:02}.bin");
            let file = host(&folder.join(&name));
            (name, file)
        })
        .collect();
    let cue = host(&folder.join("Track 01.cue"));
    let image = Disc::open(Source::Image { cue, files }).expect("the image opens");
    let iso =
        Disc::open(Source::Iso(host(&folder.join("mech2merc-data.iso")))).expect("the iso opens");
    assert_eq!(image.label(), iso.label());

    let mut walked = Vec::new();
    walk(&image, "", &mut walked);
    let mut iso_walked = Vec::new();
    walk(&iso, "", &mut iso_walked);
    assert_eq!(walked, iso_walked);

    let mut witness = BTreeMap::new();
    let top = folder.join("disc");
    if top.is_dir() {
        witnessed(&top, &top, &mut witness);
        let mut keys: Vec<String> = walked
            .iter()
            .map(|name| name.to_ascii_lowercase())
            .collect();
        keys.sort();
        assert_eq!(keys, witness.keys().cloned().collect::<Vec<_>>());
    } else {
        eprintln!("NOTICE: no disc/ witness beside the image; the iso alone compared");
    }

    let mut total = 0u64;
    for name in &walked {
        let read = bytes(&image, name);
        assert_eq!(read, bytes(&iso, name), "{name}: the image's and the iso's");
        if let Some(path) = witness.get(&name.to_ascii_lowercase()) {
            let unpacked = std::fs::read(path).expect("the witness reads");
            assert_eq!(read, unpacked, "{name}: the image's and the witness's");
        }
        total += read.len() as u64;
    }
    eprintln!(
        "falsifier 2: {} files, {total} bytes, equal on the image, the iso{}",
        walked.len(),
        if witness.is_empty() { "" } else { " and disc/" }
    );

    assert!(iso.tracks().is_empty());
    let tracks = image.tracks();
    let numbers: Vec<u8> = tracks.iter().map(|track| track.number).collect();
    assert_eq!(numbers, (2..=22).collect::<Vec<u8>>());
    assert_eq!(tracks.len(), usize::from(DISC.tracks));
    let mut frames = 0u32;
    let mut frame = vec![0; FRAME];
    let mut bin = vec![0; FRAME];
    for track in tracks {
        let Audio::Pcm(pcm) = track.audio else {
            panic!("an image's track is PCM")
        };
        assert!(!pcm.big_endian, "a BINARY bin is little-endian");
        let file = host(&folder.join(format!("Track {:02}.bin", track.number)));
        assert_eq!(u64::from(pcm.frames), file.size / RAW - PREGAP);
        let middle = u64::from(pcm.frames / 2) * RAW;
        let last = u64::from(pcm.frames - 1) * RAW;
        let ends = [
            (0, PREGAP * RAW),
            (middle, PREGAP * RAW + middle),
            (last, file.size - RAW),
        ];
        for (at, from) in ends {
            let count = image
                .read_track(track.number, at, &mut frame)
                .expect("the track reads");
            assert_eq!(count, frame.len());
            file.read_at(from, &mut bin).expect("the bin reads");
            assert_eq!(frame, bin, "track {}'s frame at {at}", track.number);
        }
        frames += pcm.frames;
    }
    assert_eq!(frames, 199_430);
    assert_eq!(format!("{:.2}", f64::from(frames) / 4500.0), "44.32");
}
