// SPDX-License-Identifier: MIT
//! `imp.through` under Wine: each call made straight and through its pass-through entry, the
//! answers held equal, and the row's fields held against the straight call's answer and last
//! error, each rendered by the renderer the row uses. Wine runs it, as the target's runner:
//! `cargo test -p mech_layer --test through --target i686-pc-windows-gnu`.
//!
//! `MECH_LAYER_THROUGH_LOG` names a file each row is appended to as the swap's sink writes a
//! line, `HH:MM:SS.mmm D imp.through … tid=<hex>`: the readers' fixture is that file.
//!
//! One original is a stand-in: the row under `WAIL32!_AIL_end_sample@4` is a Rust function's,
//! the Miles library not being here.
//!
//! The test compiles the layer's `src/through.rs` and `src/numbers.rs` into itself, beside the
//! four names they take from the layer's root, and never links the layer's library: that
//! library names Miles, Smacker and game symbols only the swap's build supplies.
//!
//! A program of its own (`harness = false`), as `Cargo.toml` says why. On any other target it
//! runs nothing.

#[cfg(not(all(target_arch = "x86", target_os = "windows")))]
fn main() {
    println!("through: 32-bit x86 Windows only, nothing run");
}

#[cfg(all(target_arch = "x86", target_os = "windows"))]
fn main() {
    wine::run();
}

#[cfg(all(target_arch = "x86", target_os = "windows"))]
#[path = "../src/through.rs"]
mod through;

#[cfg(all(target_arch = "x86", target_os = "windows"))]
#[path = "../src/numbers.rs"]
mod numbers;

/// `through.rs`'s `crate::window::Numbers`, as the layer's `window` names it.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
mod window {
    pub(crate) use crate::numbers::Numbers;
}

#[cfg(all(target_arch = "x86", target_os = "windows"))]
unsafe extern "system" {
    fn GetLastError() -> u32;
    fn SetLastError(code: u32);
}

/// The thread's last error as the call left it, read before any line.
#[cfg(all(target_arch = "x86", target_os = "windows"))]
fn left() -> u32 {
    // SAFETY: reads the thread's own last error.
    unsafe { crate::GetLastError() }
}

#[cfg(all(target_arch = "x86", target_os = "windows"))]
mod wine {
    use core::ffi::c_void;
    use core::ptr::{null_mut, with_exposed_provenance};
    use core::sync::atomic::{AtomicUsize, Ordering};
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::fs::{File, OpenOptions};
    use std::io::Write as _;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    use log::{Level, LevelFilter, Log, Metadata, Record};

    const GENERIC_READ: u32 = 0x8000_0000;
    const GENERIC_WRITE: u32 = 0x4000_0000;
    const CREATE_ALWAYS: u32 = 2;
    const OPEN_EXISTING: u32 = 3;
    const FILE_BEGIN: u32 = 0;
    const INVALID_HANDLE_VALUE: usize = usize::MAX;
    const VK_SHIFT: i32 = 0x10;
    /// A name no file holds: its folder is never made.
    const MISSING: &[u8] = b"C:\\through-test\\missing.bin\0";
    /// The file [`made_file`] makes, writes, seeks, reads back and deletes.
    const MADE: &[u8] = b"C:\\through-test.bin\0";
    /// What [`made_file`] writes.
    const WRITTEN: &[u8] = b"imp.through";
    /// The bytes [`made_file`] reads back, from offset 2.
    const BACK: usize = 5;

    type CreateFile =
        unsafe extern "system" fn(*const u8, u32, u32, *mut c_void, u32, u32, *mut c_void) -> usize;
    type Transfer = unsafe extern "system" fn(usize, *mut u8, u32, *mut u32, *mut c_void) -> i32;
    type Seek = unsafe extern "system" fn(usize, i32, *mut i32, u32) -> u32;
    type KeyState = unsafe extern "system" fn(i32) -> i16;
    type Three = unsafe extern "system" fn(u32, u32, u32) -> u32;
    type Two = unsafe extern "C" fn(u32, u32) -> u32;
    type Wide = unsafe extern "system" fn(u32) -> u64;
    type Ended = unsafe extern "system" fn(u32);

    unsafe extern "system" {
        fn GetCurrentThreadId() -> u32;
        fn GetLastError() -> u32;
        fn SetLastError(code: u32);
        fn LoadLibraryA(name: *const u8) -> *mut c_void;
        fn GetProcAddress(module: *mut c_void, name: *const u8) -> *mut c_void;
        fn CloseHandle(handle: usize) -> i32;
        fn DeleteFileA(name: *const u8) -> i32;
    }

    thread_local! {
        /// The rows this thread logged since [`rows`] last cleared them, as the sink renders
        /// each after its time and level.
        static LINES: RefCell<Vec<String>> = const { RefCell::new(Vec::new()) };
    }

    /// The file `MECH_LAYER_THROUGH_LOG` names, when it names one.
    static FIXTURE: OnceLock<Option<Mutex<File>>> = OnceLock::new();
    /// Every row logged, the fixture's count of lines when every write took.
    static LOGGED: AtomicUsize = AtomicUsize::new(0);

    /// Keeps each row for its thread's [`rows`], and appends it to the fixture as the swap's
    /// sink writes a line. A row is logged inside the layer's frame, so nothing here panics.
    struct Sink;

    static SINK: Sink = Sink;

    impl Log for Sink {
        fn enabled(&self, metadata: &Metadata<'_>) -> bool {
            metadata.level() <= Level::Debug
        }

        fn log(&self, record: &Record<'_>) {
            if !self.enabled(record.metadata()) {
                return;
            }
            let body = mech_logline::body(record);
            if let Some(Some(file)) = FIXTURE.get() {
                let since = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap_or_default();
                let seconds = since.as_secs() % 86_400;
                // SAFETY: reads the calling thread's own id.
                let thread = unsafe { GetCurrentThreadId() };
                let line = format!(
                    "{:02}:{:02}:{:02}.{:03} {} {body} tid={thread:x}\n",
                    seconds / 3_600,
                    seconds / 60 % 60,
                    seconds % 60,
                    since.subsec_millis(),
                    &record.level().as_str()[..1],
                );
                if let Ok(mut file) = file.lock() {
                    let _ = file.write_all(line.as_bytes());
                }
            }
            LOGGED.fetch_add(1, Ordering::Relaxed);
            LINES.with(|lines| lines.borrow_mut().push(body));
        }

        fn flush(&self) {}
    }

    /// A row's fields by key.
    type Row = BTreeMap<String, String>;

    /// `call`'s answer, and the rows it logged on this thread.
    fn rows<T>(call: impl FnOnce() -> T) -> (T, Vec<Row>) {
        LINES.with(|lines| lines.borrow_mut().clear());
        let answer = call();
        let lines: Vec<String> = LINES.with(|lines| lines.borrow_mut().drain(..).collect());
        (answer, lines.iter().map(|line| fields(line)).collect())
    }

    /// A row's fields: its event `imp.through`, then `key=value` words, none quoted.
    fn fields(line: &str) -> Row {
        assert!(
            !line.contains('"'),
            "a quoted value, which this split cannot read: {line}"
        );
        let mut words = line.split(' ');
        assert_eq!(words.next(), Some("imp.through"), "{line}");
        words
            .map(|word| {
                let (key, value) = word
                    .split_once('=')
                    .unwrap_or_else(|| panic!("not key=value: {word}"));
                (key.to_owned(), value.to_owned())
            })
            .collect()
    }

    /// `row` holds `want`'s fields and `from`, and no other.
    fn holds(row: &Row, want: &[(&str, String)]) {
        for (key, value) in want {
            assert_eq!(row.get(*key), Some(value), "{key} in {row:?}");
        }
        let keys: Vec<&str> = row
            .keys()
            .map(String::as_str)
            .filter(|&key| key != "from")
            .collect();
        let mut wanted: Vec<&str> = want.iter().map(|&(key, _)| key).collect();
        wanted.sort_unstable();
        assert_eq!(keys, wanted, "the keys of {row:?}");
        assert!(row.contains_key("from"), "from in {row:?}");
    }

    /// One number as a row shows it.
    fn shown(value: u32) -> String {
        mech_trace::__shown!(value).to_string()
    }

    /// Numbers as `a` shows them: each as [`shown`], comma-joined.
    fn shown_all(values: &[u32]) -> String {
        values
            .iter()
            .map(|&value| shown(value))
            .collect::<Vec<_>>()
            .join(",")
    }

    /// A NUL-ended name as `p` shows it.
    fn text(name: &[u8]) -> String {
        kernel_process::Text(&name[..name.len() - 1]).to_string()
    }

    /// The words `c` and `by` take for `label`, the original answering.
    fn named(label: &str) -> [(&'static str, String); 2] {
        [("c", label.to_owned()), ("by", "orig".to_owned())]
    }

    fn word<T>(pointer: *const T) -> u32 {
        u32::try_from(pointer.expose_provenance()).expect("a 32-bit address")
    }

    fn error() -> u32 {
        // SAFETY: reads the thread's own last error.
        unsafe { GetLastError() }
    }

    /// `name`'s address in `module`, loaded if it is not.
    fn import(module: &[u8], name: &[u8]) -> usize {
        // SAFETY: a NUL-ended name; a module already loaded is counted once more.
        let handle = unsafe { LoadLibraryA(module.as_ptr()) };
        assert!(
            !handle.is_null(),
            "{} loads",
            String::from_utf8_lossy(module)
        );
        // SAFETY: a module just loaded and a NUL-ended name.
        let address = unsafe { GetProcAddress(handle, name.as_ptr()) };
        assert!(
            !address.is_null(),
            "{} found",
            String::from_utf8_lossy(name)
        );
        address.expose_provenance()
    }

    /// The pass-through entry for `original` under `label`, as the bind takes one.
    fn entry(original: usize, label: &'static str) -> usize {
        let original = u32::try_from(original).expect("a 32-bit address");
        crate::through::entry(original, label).expect("the pool has room") as usize
    }

    /// The function at `address`, as `F`.
    ///
    /// # Safety
    ///
    /// `F` is a function pointer type, and `address` holds a function of its signature and
    /// convention.
    unsafe fn function<F: Copy>(address: usize) -> F {
        assert_eq!(size_of::<F>(), size_of::<usize>(), "a function pointer");
        let pointer: *const () = with_exposed_provenance(address);
        // SAFETY: the caller's word: `F` is a function pointer, and `pointer` one of its kind.
        unsafe { core::mem::transmute_copy(&pointer) }
    }

    /// `original` straight, and by its entry under `label`, both as `F`.
    ///
    /// # Safety
    ///
    /// As [`function`]'s, for `original`.
    unsafe fn both<F: Copy>(original: usize, label: &'static str) -> (F, F) {
        let routed = entry(original, label);
        // SAFETY: the caller's word for `original`; the entry goes on to it.
        unsafe { (function(original), function(routed)) }
    }

    fn create_file() -> (CreateFile, CreateFile) {
        let original = import(b"kernel32.dll\0", b"CreateFileA\0");
        // SAFETY: `CreateFileA`'s own address.
        unsafe { both(original, "KERNEL32!CreateFileA") }
    }

    /// Opens [`MISSING`] for reading: the handle and the last error.
    fn open_missing(create: CreateFile) -> (usize, u32) {
        // SAFETY: a NUL-ended name, no security, no template.
        let handle = unsafe {
            create(
                MISSING.as_ptr(),
                GENERIC_READ,
                0,
                null_mut(),
                OPEN_EXISTING,
                0,
                null_mut(),
            )
        };
        (handle, error())
    }

    /// `CreateFileA` on a name no file holds: one row, `a` its 7 arguments, `p` the name, `v`
    /// and `e` the straight call's handle and last error.
    fn missing_file() {
        let (straight, routed) = create_file();
        let (want, none) = rows(|| open_missing(straight));
        assert!(none.is_empty(), "a straight call writes no row: {none:?}");
        let (got, logged) = rows(|| open_missing(routed));
        assert_eq!(got, want, "the handle and last error through the entry");
        let [row] = logged.as_slice() else {
            panic!("one row: {logged:?}")
        };
        let a = [
            word(MISSING.as_ptr()),
            GENERIC_READ,
            0,
            0,
            OPEN_EXISTING,
            0,
            0,
        ];
        let [c, by] = named("KERNEL32!CreateFileA");
        holds(
            row,
            &[
                c,
                ("a", shown_all(&a)),
                ("p", text(MISSING)),
                ("v", shown(u32::try_from(want.0).expect("a 32-bit handle"))),
                ("e", shown(want.1)),
                by,
            ],
        );
    }

    /// `CreateFileA`, `WriteFile`, `SetFilePointer` and `ReadFile`, straight or by entries.
    #[derive(Clone, Copy)]
    struct Files {
        create: CreateFile,
        write: Transfer,
        seek: Seek,
        read: Transfer,
    }

    /// Each call's answer and the last error it left, and what `WriteFile` and `ReadFile` filled
    /// in: equal from pass to pass.
    #[derive(Debug, PartialEq, Eq)]
    struct Answers {
        made: u32,
        wrote: (i32, u32, u32),
        sought: (u32, u32),
        read: (i32, u32, u32, [u8; BACK]),
    }

    /// One pass's answers, and the handle and the places its arguments named, a pass's own.
    struct Pass {
        answers: Answers,
        handle: u32,
        /// Where `WriteFile` put its count, `ReadFile` its bytes, and `ReadFile` its count.
        places: [u32; 3],
    }

    /// [`MADE`] made, [`WRITTEN`] written, sought to 2, [`BACK`] bytes read back; then closed
    /// and deleted, straight.
    fn pass(files: Files) -> Pass {
        let length = u32::try_from(WRITTEN.len()).expect("a short write");
        let back_length = u32::try_from(BACK).expect("a short read");
        // SAFETY: a NUL-ended name, no security, no template.
        let handle = unsafe {
            (files.create)(
                MADE.as_ptr(),
                GENERIC_READ | GENERIC_WRITE,
                0,
                null_mut(),
                CREATE_ALWAYS,
                0,
                null_mut(),
            )
        };
        let made = error();
        assert_ne!(handle, INVALID_HANDLE_VALUE, "the file made");
        let mut wrote = 0;
        // SAFETY: the handle just made; `WRITTEN`'s bytes, read only; `wrote` lives here.
        let put = unsafe {
            (files.write)(
                handle,
                WRITTEN.as_ptr().cast_mut(),
                length,
                &raw mut wrote,
                null_mut(),
            )
        };
        let write_error = error();
        // SAFETY: the handle just made; no high word.
        let at = unsafe { (files.seek)(handle, 2, null_mut(), FILE_BEGIN) };
        let seek_error = error();
        let mut back = [0; BACK];
        let mut read = 0;
        // SAFETY: the handle just made; `back` holds `BACK` bytes; `read` lives here.
        let got = unsafe {
            (files.read)(
                handle,
                back.as_mut_ptr(),
                back_length,
                &raw mut read,
                null_mut(),
            )
        };
        let read_error = error();
        let places = [
            word(&raw const wrote),
            word(back.as_ptr()),
            word(&raw const read),
        ];
        // SAFETY: the handle just made, closed once.
        assert_ne!(unsafe { CloseHandle(handle) }, 0, "the file closed");
        // SAFETY: a NUL-ended name.
        assert_ne!(unsafe { DeleteFileA(MADE.as_ptr()) }, 0, "the file deleted");
        Pass {
            answers: Answers {
                made,
                wrote: (put, write_error, wrote),
                sought: (at, seek_error),
                read: (got, read_error, read, back),
            },
            handle: u32::try_from(handle).expect("a 32-bit handle"),
            places,
        }
    }

    /// A file made, written, sought and read back: 4 rows in the order called, each `a` the
    /// arguments as passed and `v` and `e` as the call left them; the answers as straight.
    fn made_file() {
        let (create, create_routed) = create_file();
        let kernel = |name: &[u8]| import(b"kernel32.dll\0", name);
        // SAFETY: each the address of the function its type and label name.
        let (write, write_routed) = unsafe { both(kernel(b"WriteFile\0"), "KERNEL32!WriteFile") };
        // SAFETY: as above.
        let (seek, seek_routed) =
            unsafe { both(kernel(b"SetFilePointer\0"), "KERNEL32!SetFilePointer") };
        // SAFETY: as above.
        let (read, read_routed) = unsafe { both(kernel(b"ReadFile\0"), "KERNEL32!ReadFile") };
        // SAFETY: a NUL-ended name; a file left by an earlier run would change `made`.
        unsafe { DeleteFileA(MADE.as_ptr()) };
        let straight = Files {
            create,
            write,
            seek,
            read,
        };
        let routed = Files {
            create: create_routed,
            write: write_routed,
            seek: seek_routed,
            read: read_routed,
        };
        let (want, none) = rows(|| pass(straight));
        assert!(none.is_empty(), "straight calls write no row: {none:?}");
        let (got, logged) = rows(|| pass(routed));
        assert_eq!(got.answers, want.answers, "the answers through the entries");
        let [made_row, write_row, seek_row, read_row] = logged.as_slice() else {
            panic!("four rows: {logged:?}")
        };
        let (handle, places, answers) = (got.handle, got.places, &got.answers);
        let length = u32::try_from(WRITTEN.len()).expect("a short write");
        let back = u32::try_from(BACK).expect("a short read");
        let shown_i32 = |value: i32| shown(u32::from_ne_bytes(value.to_ne_bytes()));
        let [c, by] = named("KERNEL32!CreateFileA");
        let a = [
            word(MADE.as_ptr()),
            GENERIC_READ | GENERIC_WRITE,
            0,
            0,
            CREATE_ALWAYS,
            0,
            0,
        ];
        holds(
            made_row,
            &[
                c,
                ("a", shown_all(&a)),
                ("p", text(MADE)),
                ("v", shown(handle)),
                ("e", shown(answers.made)),
                by,
            ],
        );
        let [c, by] = named("KERNEL32!WriteFile");
        let a = [handle, word(WRITTEN.as_ptr()), length, places[0], 0];
        let (value, written_error, _) = answers.wrote;
        holds(
            write_row,
            &[
                c,
                ("a", shown_all(&a)),
                ("v", shown_i32(value)),
                ("e", shown(written_error)),
                by,
            ],
        );
        let [c, by] = named("KERNEL32!SetFilePointer");
        let a = [handle, 2, 0, FILE_BEGIN];
        holds(
            seek_row,
            &[
                c,
                ("a", shown_all(&a)),
                ("v", shown(answers.sought.0)),
                ("e", shown(answers.sought.1)),
                by,
            ],
        );
        let [c, by] = named("KERNEL32!ReadFile");
        let a = [handle, places[1], back, places[2], 0];
        let (value, read_error, _, _) = answers.read;
        holds(
            read_row,
            &[
                c,
                ("a", shown_all(&a)),
                ("v", shown_i32(value)),
                ("e", shown(read_error)),
                by,
            ],
        );
    }

    /// Asks Shift's state: the answer and the last error.
    fn ask(state: KeyState) -> (i16, u32) {
        // SAFETY: a key code; the call reads the keyboard's state alone.
        (unsafe { state(VK_SHIFT) }, error())
    }

    /// `GetAsyncKeyState` for Shift: `a` the key; `v` all of `eax`, its low half the straight
    /// answer.
    fn key_state() {
        let original = import(b"user32.dll\0", b"GetAsyncKeyState\0");
        // SAFETY: `GetAsyncKeyState`'s own address.
        let (straight, routed): (KeyState, KeyState) =
            unsafe { both(original, "USER32!GetAsyncKeyState") };
        let (want, none) = rows(|| ask(straight));
        assert!(none.is_empty(), "a straight call writes no row: {none:?}");
        let (got, logged) = rows(|| ask(routed));
        assert_eq!(got, want, "the answer and last error through the entry");
        let [row] = logged.as_slice() else {
            panic!("one row: {logged:?}")
        };
        let v = u32::from_str_radix(&row["v"], 16).expect("v in hex");
        assert_eq!(shown(v), row["v"], "v read back as the row shows it");
        assert_eq!(
            v & 0xffff,
            u32::from(u16::from_ne_bytes(want.0.to_ne_bytes())),
            "v's low half is the answer"
        );
        let [c, by] = named("USER32!GetAsyncKeyState");
        let key = u32::from_ne_bytes(VK_SHIFT.to_ne_bytes());
        holds(
            row,
            &[
                c,
                ("a", shown_all(&[key])),
                ("v", row["v"].clone()),
                ("e", shown(want.1)),
                by,
            ],
        );
    }

    /// [`outer`]'s way to `CreateFileA`: its entry, stored before `outer` is called.
    static INNER: AtomicUsize = AtomicUsize::new(0);

    /// A stdcall original of 3 that opens `name` by `CreateFileA`'s entry: that call's frame
    /// nests in this one's, and this one returns with its 12 bytes popped by itself.
    extern "system" fn outer(name: u32, disposition: u32, salt: u32) -> u32 {
        // SAFETY: `INNER` holds `CreateFileA`'s entry, stored before any call here.
        let create: CreateFile = unsafe { function(INNER.load(Ordering::Relaxed)) };
        let name = with_exposed_provenance::<u8>(name as usize);
        // SAFETY: a NUL-ended name, no security, no template.
        let handle = unsafe {
            create(
                name,
                GENERIC_READ,
                0,
                null_mut(),
                disposition,
                0,
                null_mut(),
            )
        };
        u32::try_from(handle).unwrap_or(0) ^ salt
    }

    fn call_outer(three: Three) -> (u32, u32) {
        // SAFETY: `outer`'s signature; a NUL-ended name.
        (
            unsafe { three(word(MISSING.as_ptr()), OPEN_EXISTING, 0x5a5a) },
            error(),
        )
    }

    /// The pair for [`outer`]: straight, and by its entry.
    fn outer_both() -> (Three, Three) {
        let (_, create) = create_file();
        INNER.store(create as usize, Ordering::Relaxed);
        // SAFETY: `outer`'s own address.
        unsafe { both(outer as Three as usize, "TEST!outer@12") }
    }

    /// A stdcall original of 3 calling a second entry, 3 times: the nested row first, as the
    /// straight call's own, then the original's, no `a` (none declared); the answers as
    /// straight, so the pops held.
    fn nesting() {
        let (straight, routed) = outer_both();
        let (want, inner) = rows(|| call_outer(straight));
        let [inner] = inner.as_slice() else {
            panic!("the straight call's nested row alone: {inner:?}")
        };
        for _ in 0..3 {
            let (got, logged) = rows(|| call_outer(routed));
            assert_eq!(got, want, "the answer and last error through the entry");
            let [first, second] = logged.as_slice() else {
                panic!("two rows: {logged:?}")
            };
            assert_eq!(first, inner, "the nested row, as the straight call's");
            let [c, by] = named("TEST!outer@12");
            holds(second, &[c, ("v", shown(want.0)), ("e", shown(want.1)), by]);
        }
    }

    /// A cdecl original of 2: its caller pops the arguments.
    extern "C" fn mixed(left: u32, right: u32) -> u32 {
        left.wrapping_mul(31) ^ right
    }

    fn call_mixed(two: Two) -> (u32, u32) {
        // SAFETY: sets this thread's own last error, which the call leaves as it is.
        unsafe { SetLastError(0x2a) };
        // SAFETY: `mixed`'s signature.
        (unsafe { two(7, 9) }, error())
    }

    /// A cdecl original: the answer and the last error as straight, `e` the error it left.
    fn cdecl() {
        // SAFETY: `mixed`'s own address.
        let (straight, routed): (Two, Two) = unsafe { both(mixed as Two as usize, "TEST!mixed") };
        let (want, none) = rows(|| call_mixed(straight));
        assert!(none.is_empty(), "a straight call writes no row: {none:?}");
        let (got, logged) = rows(|| call_mixed(routed));
        assert_eq!(got, want, "the answer and last error through the entry");
        let [row] = logged.as_slice() else {
            panic!("one row: {logged:?}")
        };
        let [c, by] = named("TEST!mixed");
        holds(row, &[c, ("v", shown(want.0)), ("e", shown(want.1)), by]);
    }

    /// A stdcall original answering 64 bits, the high half in `edx`.
    extern "system" fn wide(low: u32) -> u64 {
        (u64::from(!low) << 32) | u64::from(low)
    }

    fn call_wide(answer: Wide) -> (u64, u32) {
        // SAFETY: `wide`'s signature.
        (unsafe { answer(0x1234_5678) }, error())
    }

    /// A 64-bit answer: all of it as straight, `edx` kept; `v` its low half alone.
    fn high_half() {
        // SAFETY: `wide`'s own address.
        let (straight, routed): (Wide, Wide) =
            unsafe { both(wide as Wide as usize, "TEST!wide@4") };
        let (want, none) = rows(|| call_wide(straight));
        assert!(none.is_empty(), "a straight call writes no row: {none:?}");
        let (got, logged) = rows(|| call_wide(routed));
        assert_eq!(
            got, want,
            "the answer, both halves, and last error through the entry"
        );
        let [row] = logged.as_slice() else {
            panic!("one row: {logged:?}")
        };
        let low = u32::try_from(want.0 & u64::from(u32::MAX)).expect("the low half");
        let [c, by] = named("TEST!wide@4");
        holds(row, &[c, ("v", shown(low)), ("e", shown(want.1)), by]);
    }

    /// The stand-in for `_AIL_end_sample`: takes a sample handle, answers nothing, as the
    /// import does.
    extern "system" fn end_sample(_sample: u32) {}

    fn call_end(ended: Ended) -> u32 {
        // SAFETY: `end_sample`'s signature.
        unsafe { ended(0x0123_4560) };
        error()
    }

    /// A Miles row: `a` the sample handle; `v` whatever `eax` held, the import answering
    /// nothing.
    fn sound() {
        // SAFETY: `end_sample`'s own address.
        let (straight, routed): (Ended, Ended) =
            unsafe { both(end_sample as Ended as usize, "WAIL32!_AIL_end_sample@4") };
        let (want, none) = rows(|| call_end(straight));
        assert!(none.is_empty(), "a straight call writes no row: {none:?}");
        let (got, logged) = rows(|| call_end(routed));
        assert_eq!(got, want, "the last error through the entry");
        let [row] = logged.as_slice() else {
            panic!("one row: {logged:?}")
        };
        let [c, by] = named("WAIL32!_AIL_end_sample@4");
        holds(
            row,
            &[
                c,
                ("a", shown_all(&[0x0123_4560])),
                ("v", row["v"].clone()),
                ("e", shown(want)),
                by,
            ],
        );
    }

    /// `imp.through` off, the logger's level above DEBUG: no row, the answers as straight, the
    /// nested call too.
    fn off() {
        let (straight, routed) = create_file();
        let (outer_straight, outer_routed) = outer_both();
        log::set_max_level(LevelFilter::Info);
        let (want, none) = rows(|| (open_missing(straight), call_outer(outer_straight)));
        let (got, logged) = rows(|| (open_missing(routed), call_outer(outer_routed)));
        log::set_max_level(LevelFilter::Debug);
        assert!(none.is_empty(), "no row off: {none:?}");
        assert!(logged.is_empty(), "no row off: {logged:?}");
        assert_eq!(
            got, want,
            "the answers and last errors through the entries, off"
        );
    }

    pub(super) fn run() {
        let fixture = std::env::var_os("MECH_LAYER_THROUGH_LOG").map(|path| {
            let file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .expect("the fixture opens");
            Mutex::new(file)
        });
        FIXTURE.set(fixture).expect("the fixture set once");
        log::set_logger(&SINK).expect("the one logger");
        log::set_max_level(LevelFilter::Debug);
        let cases: [(&str, fn()); 8] = [
            ("missing_file", missing_file),
            ("made_file", made_file),
            ("key_state", key_state),
            ("nesting", nesting),
            ("cdecl", cdecl),
            ("high_half", high_half),
            ("sound", sound),
            ("off", off),
        ];
        for (name, case) in cases {
            case();
            println!("{name}: passed");
        }
        println!(
            "through: {} cases passed, {} rows logged",
            cases.len(),
            LOGGED.load(Ordering::Relaxed)
        );
    }
}
