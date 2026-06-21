set dotenv-load := false

cargo := env_var_or_default("CARGO", "cargo")
rusty_idd := cargo + " run --bin rusty-idd --"

build:
    {{cargo}} build --workspace --locked

test:
    {{cargo}} test --workspace --locked

fmt:
    {{cargo}} fmt --all

fmt-check:
    {{cargo}} fmt --all -- --check

lint:
    {{cargo}} clippy --workspace --all-targets --all-features -- -D warnings

audit:
    cargo audit --deny warnings

validate:
    {{rusty_idd}} validate --workspace .

manifest:
    {{rusty_idd}} manifest --workspace . --out .idd/MANIFEST.tsv

manifest-check:
    tmp=$(mktemp) && {{rusty_idd}} manifest --workspace . --out "$tmp" && cmp -s .idd/MANIFEST.tsv "$tmp" || { echo ".idd/MANIFEST.tsv is stale; run just manifest" >&2; rm -f "$tmp"; exit 1; }; rm -f "$tmp"

knowledge:
    {{rusty_idd}} knowledge refresh --workspace .

knowledge-check:
    tmpdir=$(mktemp -d) && {{rusty_idd}} knowledge index --workspace . --out "$tmpdir/index.json" && {{rusty_idd}} knowledge report --workspace . --out "$tmpdir/report.md" && {{rusty_idd}} knowledge architecture --workspace . --out "$tmpdir/architecture.json" && {{rusty_idd}} knowledge architecture --workspace . --out "$tmpdir/architecture.md" && cmp -s .idd/knowledge/index.json "$tmpdir/index.json" && cmp -s .idd/knowledge/report.md "$tmpdir/report.md" && cmp -s .idd/knowledge/architecture.json "$tmpdir/architecture.json" && cmp -s .idd/knowledge/architecture.md "$tmpdir/architecture.md" || { echo ".idd/knowledge artifacts are stale; run just knowledge" >&2; rm -rf "$tmpdir"; exit 1; }; rm -rf "$tmpdir"

codex-env-check:
    {{rusty_idd}} codex env-check

codex-model-loop:
    {{rusty_idd}} codex model-loop

codex-runtime-audit:
    {{rusty_idd}} codex runtime-audit

codex-system-audit:
    {{rusty_idd}} codex system-audit

ci: build test validate manifest-check knowledge-check codex-env-check codex-runtime-audit codex-model-loop fmt-check lint audit
