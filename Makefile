CARGO ?= cargo
RUSTY_IDD ?= $(CARGO) run --bin rusty-idd --

.PHONY: build test fmt fmt-check lint audit validate manifest manifest-check knowledge knowledge-check codex-env-check codex-runtime-audit codex-system-audit codex-model-loop ci install-hooks clean

build:
	$(CARGO) build --workspace --locked

test:
	$(CARGO) test --workspace --locked

fmt:
	$(CARGO) fmt --all

fmt-check:
	$(CARGO) fmt --all -- --check

lint:
	$(CARGO) clippy --workspace --all-targets --all-features -- -D warnings

audit:
	cargo audit --deny warnings

validate:
	$(RUSTY_IDD) validate --workspace .

manifest:
	$(RUSTY_IDD) manifest --workspace . --out .idd/MANIFEST.tsv

manifest-check:
	tmp=$$(mktemp) && $(RUSTY_IDD) manifest --workspace . --out "$$tmp" && cmp -s .idd/MANIFEST.tsv "$$tmp" || (echo ".idd/MANIFEST.tsv is stale; run make manifest" >&2; rm -f "$$tmp"; exit 1); rm -f "$$tmp"

knowledge:
	$(RUSTY_IDD) knowledge refresh --workspace .

knowledge-check:
	tmpdir=$$(mktemp -d) && $(RUSTY_IDD) knowledge index --workspace . --out "$$tmpdir/index.json" && $(RUSTY_IDD) knowledge report --workspace . --out "$$tmpdir/report.md" && cmp -s .idd/knowledge/index.json "$$tmpdir/index.json" && cmp -s .idd/knowledge/report.md "$$tmpdir/report.md" || (echo ".idd/knowledge artifacts are stale; run make knowledge" >&2; rm -rf "$$tmpdir"; exit 1); rm -rf "$$tmpdir"

codex-env-check:
	$(RUSTY_IDD) codex env-check

codex-model-loop:
	$(RUSTY_IDD) codex model-loop

codex-runtime-audit:
	$(RUSTY_IDD) codex runtime-audit

codex-system-audit:
	$(RUSTY_IDD) codex system-audit

ci: build test validate manifest-check knowledge-check codex-env-check codex-runtime-audit codex-model-loop fmt-check lint audit

install-hooks:
	git config core.hooksPath .githooks

clean:
	$(CARGO) clean
