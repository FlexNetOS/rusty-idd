//! `rusty-idd spec adr` — the FS edge over the ADR supersession graph
//! (`rusty_idd_spec::adr`). Reads the repo-level `adr/` directory and reports
//! the in-force decisions or the next sequence number.

use std::collections::BTreeMap;
use std::path::Path;

use clap::Subcommand;
use rusty_idd_spec::adr::{parse_adr, Adr, AdrSet, AdrStatus};

/// Frozen baseline of ADR sequence numbers that are *knowingly* duplicated by
/// immutable, already-accepted ADRs, mapped to the EXACT number of ADRs that
/// legitimately share each. Parallel changes each allocated the same `spec adr
/// next` value before either committed, producing these four collisions (two
/// ADRs each). ADRs are immutable once accepted (supersede, don't edit), so the
/// files are frozen historical artifacts and reconciled by ADR-0016 + a
/// slug-canonical referencing rule — not by renumbering. The `--check` gate
/// accepts a baseline number only at its frozen count; a *third* file at a
/// baseline number (count exceeds the baseline) is a NEW collision and fails
/// closed, just like a collision at a fresh number. Mirrors the
/// `.cargo/audit.toml` baseline philosophy: known exceptions are pinned exactly,
/// anything beyond them fails.
const ACCEPTED_DUPLICATE_ADRS: &[(u32, usize)] = &[(2, 2), (4, 2), (5, 2), (6, 2)];

#[derive(Subcommand)]
pub enum AdrCommand {
    /// List ADRs. By default shows only the in-force set (accepted, not
    /// superseded); `--all` shows every ADR with its status.
    List {
        /// The ADR directory (defaults to `adr`).
        #[arg(default_value = "adr")]
        adr_dir: std::path::PathBuf,
        /// Show every ADR (including superseded/proposed) with its status.
        #[arg(long)]
        all: bool,
        /// Fail closed on any duplicate ADR number outside the frozen baseline
        /// of known historical collisions. Reports all duplicates; exits
        /// non-zero only on new ones.
        #[arg(long)]
        check: bool,
    },
    /// Print the next ADR sequence number (zero-padded NNNN).
    Next {
        /// The ADR directory (defaults to `adr`).
        #[arg(default_value = "adr")]
        adr_dir: std::path::PathBuf,
    },
}

/// Dispatch a `spec adr` subcommand.
pub fn run(cmd: AdrCommand) -> i32 {
    match cmd {
        AdrCommand::List {
            adr_dir,
            all,
            check,
        } => {
            if check {
                run_check(&adr_dir)
            } else {
                run_list(&adr_dir, all)
            }
        }
        AdrCommand::Next { adr_dir } => run_next(&adr_dir),
    }
}

/// Read and parse every `NNNN-*.md` ADR file in `adr_dir`.
fn load_adrs(adr_dir: &Path) -> Result<AdrSet, String> {
    if !adr_dir.is_dir() {
        return Err(format!("adr directory not found: {}", adr_dir.display()));
    }
    let entries = std::fs::read_dir(adr_dir)
        .map_err(|e| format!("failed to read {}: {e}", adr_dir.display()))?;
    let mut adrs: Vec<Adr> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        // Only ADR files (start with a digit); skip README.md, templates, etc.
        if !name.chars().next().is_some_and(|c| c.is_ascii_digit()) {
            continue;
        }
        let src = std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read {}: {e}", path.display()))?;
        if let Some(a) = parse_adr(&name, &src) {
            adrs.push(a);
        }
    }
    Ok(AdrSet::new(adrs))
}

fn run_list(adr_dir: &Path, all: bool) -> i32 {
    let set = match load_adrs(adr_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("rusty-idd: {e}");
            return 1;
        }
    };

    if all {
        let mut adrs: Vec<&Adr> = set.adrs.iter().collect();
        adrs.sort_by_key(|a| a.number);
        if adrs.is_empty() {
            println!("No ADRs in {}.", adr_dir.display());
            return 0;
        }
        for a in adrs {
            let status = status_label(&set, a);
            println!("ADR-{:04}  {:<11} {}", a.number, status, a.title);
        }
    } else {
        let in_force = set.in_force();
        if in_force.is_empty() {
            println!("No in-force ADRs in {}.", adr_dir.display());
            return 0;
        }
        println!("In-force ADRs ({}):", in_force.len());
        for a in in_force {
            println!("  ADR-{:04}  {}", a.number, a.title);
        }
    }
    0
}

/// A human status label for `--all` listing.
fn status_label(set: &AdrSet, a: &Adr) -> String {
    match &a.status {
        AdrStatus::Proposed => "proposed".to_string(),
        AdrStatus::Other(s) if s.is_empty() => "unknown".to_string(),
        AdrStatus::Other(s) => s.clone(),
        AdrStatus::Accepted => {
            // Find who supersedes it, if anyone.
            if let Some(by) = set
                .adrs
                .iter()
                .find(|other| other.supersedes.contains(&a.number))
            {
                format!("superseded(by {:04})", by.number)
            } else {
                "in-force".to_string()
            }
        }
    }
}

fn run_next(adr_dir: &Path) -> i32 {
    // A missing adr/ dir is fine for `next` — it just means start at 0001.
    let set = load_adrs(adr_dir).unwrap_or_default();
    println!("{:04}", set.next_number());
    0
}

/// `spec adr list --check` — fail closed on ADR-number collisions outside the
/// frozen baseline. Duplicates are always reported (for visibility); only
/// *new* collisions cause a non-zero exit.
fn run_check(adr_dir: &Path) -> i32 {
    let set = match load_adrs(adr_dir) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("rusty-idd: {e}");
            return 1;
        }
    };
    let duplicates = collisions(&set);
    if duplicates.is_empty() {
        println!("ADR ledger: no duplicate numbers.");
        return 0;
    }

    let mut new_collisions: Vec<u32> = Vec::new();
    for (number, count) in &duplicates {
        // Accepted only at the EXACT frozen count: a baseline number with extra
        // files beyond its pinned count is a new collision (count exceeds
        // baseline), not a free pass.
        let baseline = ACCEPTED_DUPLICATE_ADRS
            .iter()
            .find(|(n, _)| n == number)
            .map(|(_, c)| *c);
        let accepted = baseline == Some(*count);
        let tag = match baseline {
            Some(expected) if *count > expected => "EXCEEDS BASELINE",
            Some(_) => "accepted baseline",
            None => "NEW COLLISION",
        };
        println!("ADR-{number:04}: {count} ADRs share this number ({tag})");
        if !accepted {
            new_collisions.push(*number);
        }
    }

    if new_collisions.is_empty() {
        println!(
            "ADR ledger OK: {} duplicate number(s), all in the frozen baseline.",
            duplicates.len()
        );
        0
    } else {
        eprintln!(
            "ADR ledger FAIL: {} new collision(s) outside the baseline {:?}:",
            new_collisions.len(),
            ACCEPTED_DUPLICATE_ADRS
        );
        for n in &new_collisions {
            eprintln!("  ADR-{n:04}");
        }
        eprintln!(
            "  fix: give the new ADR the next free number (`rusty-idd spec adr next`), \
             or, if it is an accepted immutable historical collision, add it to \
             ACCEPTED_DUPLICATE_ADRS with an ADR recording why."
        );
        1
    }
}

/// Numbers shared by more than one ADR, mapped to how many ADRs use them.
/// Sorted ascending by number (BTreeMap) for deterministic output.
fn collisions(set: &AdrSet) -> BTreeMap<u32, usize> {
    let mut counts: BTreeMap<u32, usize> = BTreeMap::new();
    for a in &set.adrs {
        *counts.entry(a.number).or_insert(0) += 1;
    }
    counts.into_iter().filter(|(_, c)| *c > 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn adr(number: u32, slug: &str) -> Adr {
        Adr {
            number,
            title: format!("ADR {number} {slug}"),
            status: AdrStatus::Accepted,
            supersedes: Vec::new(),
        }
    }

    #[test]
    fn collisions_finds_only_shared_numbers() {
        let set = AdrSet::new(vec![adr(1, "a"), adr(2, "b"), adr(2, "c"), adr(3, "d")]);
        let dups = collisions(&set);
        assert_eq!(dups.len(), 1);
        assert_eq!(dups.get(&2), Some(&2));
        assert!(!dups.contains_key(&1));
    }

    #[test]
    fn baseline_numbers_are_the_four_known_collisions() {
        // Guards against accidental edits to the frozen baseline: exactly the
        // four historical numbers, each pinned to a count of 2.
        assert_eq!(ACCEPTED_DUPLICATE_ADRS, &[(2, 2), (4, 2), (5, 2), (6, 2)]);
    }
}
