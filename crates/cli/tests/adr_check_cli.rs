//! Integration tests for `rusty-idd spec adr list --check` — the fail-closed
//! ADR-number collision gate (ADR-0016), via the compiled binary.

use std::path::Path;
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_rusty-idd")
}

/// Run `spec adr list --check <dir>` and return (code, stdout, stderr).
fn check(adr_dir: &Path) -> (i32, String, String) {
    let out = Command::new(bin())
        .args(["spec", "adr", "list", "--check"])
        .arg(adr_dir)
        .output()
        .expect("run rusty-idd spec adr list --check");
    (
        out.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
    )
}

/// Write a minimal, parseable ADR file (`NNNN-slug.md`).
fn write_adr(dir: &Path, number: u32, slug: &str) {
    let name = format!("{number:04}-{slug}.md");
    let body = format!("# {number}. {slug}\n\n- Status: accepted\n- Date: 2026-06-23\n");
    std::fs::write(dir.join(name), body).unwrap();
}

#[test]
fn no_duplicates_passes() {
    let d = tempfile::tempdir().unwrap();
    write_adr(d.path(), 1, "alpha");
    write_adr(d.path(), 3, "gamma");
    let (code, out, _e) = check(d.path());
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("no duplicate"), "{out}");
}

#[test]
fn baseline_collision_passes_but_is_reported() {
    let d = tempfile::tempdir().unwrap();
    write_adr(d.path(), 1, "alpha");
    // 0002 is a frozen baseline collision.
    write_adr(d.path(), 2, "beta");
    write_adr(d.path(), 2, "beta-two");
    let (code, out, _e) = check(d.path());
    assert_eq!(code, 0, "{out}");
    assert!(out.contains("ADR-0002"), "{out}");
    assert!(out.contains("accepted baseline"), "{out}");
}

#[test]
fn third_file_at_baseline_number_exceeds_baseline_and_fails() {
    let d = tempfile::tempdir().unwrap();
    // 0002 is a baseline collision pinned at count 2; a THIRD file pushes the
    // count past the frozen baseline and must fail closed.
    write_adr(d.path(), 2, "beta");
    write_adr(d.path(), 2, "beta-two");
    write_adr(d.path(), 2, "beta-three");
    let (code, _o, err) = check(d.path());
    assert_eq!(code, 1, "{err}");
    assert!(err.contains("ADR-0002"), "{err}");
}

#[test]
fn new_collision_fails_closed() {
    let d = tempfile::tempdir().unwrap();
    write_adr(d.path(), 1, "alpha");
    // 0003 is NOT in the frozen baseline -> must fail.
    write_adr(d.path(), 3, "gamma");
    write_adr(d.path(), 3, "gamma-two");
    let (code, _o, err) = check(d.path());
    assert_eq!(code, 1);
    assert!(
        err.contains("NEW COLLISION") || err.contains("ADR-0003"),
        "{err}"
    );
}

#[test]
fn new_collision_fails_even_alongside_baseline() {
    let d = tempfile::tempdir().unwrap();
    write_adr(d.path(), 2, "beta"); // baseline dup (ok)
    write_adr(d.path(), 2, "beta-two");
    write_adr(d.path(), 7, "eta"); // new dup (fail)
    write_adr(d.path(), 7, "eta-two");
    let (code, _o, err) = check(d.path());
    assert_eq!(code, 1, "{err}");
    assert!(err.contains("ADR-0007"), "{err}");
}
