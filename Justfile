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

manifest-check: manifest
    git diff --exit-code -- .idd/MANIFEST.tsv

ci: build test validate manifest-check fmt-check lint audit
