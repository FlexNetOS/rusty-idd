# Contributing

Use Conventional Commit style for commit subjects and PR titles:

```text
feat: add workspace snapshot filtering
fix(hf): preserve child repo commit type
ci: enforce semantic PR titles
docs: clarify plugin installation
```

If you are working from an internal FlexNetOS task, include the task wikilink in
commit messages:

```text
fix: remove legacy agent workspace entry [[tasks/HFTASK-678]]
```

GitHub merge subjects are derived from PR titles, so the semantic PR title check
is the merge-blocking guard for release notes. Local hooks catch malformed commit
messages before push when installed with:

```sh
make install-hooks
```
## Branch Model

- `develop` is the integration branch for normal changes.
- `main` is the protected release trunk.
- Feature and migration work should land as narrow pull requests into `develop`.
- Promotion from `develop` to `main` must pass `promote-verify`.

## Local Verification

Run the same gates that CI enforces:

```bash
make ci
```

Equivalent individual commands:

```bash
cargo build --workspace --locked
cargo test --workspace --locked
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo audit --deny warnings
cargo run --bin rusty-idd -- validate --workspace .
cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv
git diff --exit-code -- .idd/MANIFEST.tsv
```

## Pull Request Evidence

Every PR must include:

- Build command result.
- Test command result.
- Lint/typecheck result.
- Secret scan or validation result.
- Migration note that explains old path to new path, or why unchanged.
- Rollback path.
- Manifest update, or a note explaining why the manifest is unchanged.

## Release

Release Please owns the version in `VERSION` and `Cargo.toml` workspace package metadata. The release workflow builds native `rusty-idd` binaries for Linux, macOS, and Windows when a GitHub release is published.
