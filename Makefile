RUST_BACKTRACE ?= full
RUST_LOG ?= trace

.DEFAULT_GOAL := build

build:
	cargo build --workspace

test:
	RUST_BACKTRACE=$(RUST_BACKTRACE) RUST_LOG=$(RUST_LOG) cargo test --workspace

fmt:
	cargo fmt -p hf -p ledger -p work-order

fmt-check:
	cargo fmt -p hf -p ledger -p work-order -- --check

clippy:
	cargo clippy --workspace --all-targets -- -D warnings

release:
	cargo build --release -p hf

clean:
	cargo clean

# Install git hooks (commit-msg validates release-safe subjects; pre-push runs CI checks locally)
# Uses git rev-parse to handle worktrees and submodules correctly
install-hooks:
	@echo "Installing git hooks..."
	@chmod +x .githooks/commit-msg
	@chmod +x .githooks/pre-commit
	@chmod +x .githooks/pre-push
	@mkdir -p "$$(git rev-parse --git-path hooks)"
	@hooks_dir="$$(git rev-parse --git-path hooks)" && root="$$(git rev-parse --show-toplevel)" && rel="$$(python3 -c 'import os, sys; print(os.path.relpath(sys.argv[1], sys.argv[2]))' "$$root/.githooks/commit-msg" "$$hooks_dir")" && ln -sf "$$rel" "$$hooks_dir/commit-msg"
	@hooks_dir="$$(git rev-parse --git-path hooks)" && root="$$(git rev-parse --show-toplevel)" && rel="$$(python3 -c 'import os, sys; print(os.path.relpath(sys.argv[1], sys.argv[2]))' "$$root/.githooks/pre-commit" "$$hooks_dir")" && ln -sf "$$rel" "$$hooks_dir/pre-commit"
	@hooks_dir="$$(git rev-parse --git-path hooks)" && root="$$(git rev-parse --show-toplevel)" && rel="$$(python3 -c 'import os, sys; print(os.path.relpath(sys.argv[1], sys.argv[2]))' "$$root/.githooks/pre-push" "$$hooks_dir")" && ln -sf "$$rel" "$$hooks_dir/pre-push"
	@echo "Commit-msg, pre-commit, and pre-push hooks installed."

.PHONY: build test fmt fmt-check clippy release clean install-hooks
CARGO ?= cargo
RUSTY_IDD ?= $(CARGO) run --bin rusty-idd --
RUSTY_IDD_CHANGE ?= upgrade-codex-harness-rusty-idd-flow
RUSTY_IDD_GOAL ?= update the .codex agent harness and related files to reflect the true flow of rusty-idd. Ai_merge is a tool for rusty_idd and must be treated as one and removed as the main intent

.PHONY: build test fmt fmt-check lint audit validate manifest manifest-check knowledge knowledge-check operating-model operating-model-check integration-plan integration-plan-check integration-status integration-status-check integration-owners integration-owners-check integration-readiness integration-readiness-check plan-context plan-context-check codex-env-check codex-runtime-audit codex-system-audit codex-model-loop ci install-hooks clean

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

operating-model:
	$(RUSTY_IDD) knowledge operating-model --workspace . --out .idd/knowledge/operating-model.json
	$(RUSTY_IDD) knowledge operating-model --workspace . --out .idd/knowledge/operating-model.md

operating-model-check:
	tmpdir=$$(mktemp -d) && $(RUSTY_IDD) knowledge operating-model --workspace . --out "$$tmpdir/operating-model.json" && $(RUSTY_IDD) knowledge operating-model --workspace . --out "$$tmpdir/operating-model.md" && cmp -s .idd/knowledge/operating-model.json "$$tmpdir/operating-model.json" && cmp -s .idd/knowledge/operating-model.md "$$tmpdir/operating-model.md" || (echo ".idd/knowledge operating-model artifacts are stale; run make operating-model" >&2; rm -rf "$$tmpdir"; exit 1); rm -rf "$$tmpdir"

integration-plan:
	$(RUSTY_IDD) knowledge integration-plan --workspace . --out .idd/knowledge/integration-plan.json
	$(RUSTY_IDD) knowledge integration-plan --workspace . --out .idd/knowledge/integration-plan.md

integration-plan-check:
	tmpdir=$$(mktemp -d) && $(RUSTY_IDD) knowledge integration-plan --workspace . --out "$$tmpdir/integration-plan.json" && $(RUSTY_IDD) knowledge integration-plan --workspace . --out "$$tmpdir/integration-plan.md" && cmp -s .idd/knowledge/integration-plan.json "$$tmpdir/integration-plan.json" && cmp -s .idd/knowledge/integration-plan.md "$$tmpdir/integration-plan.md" || (echo ".idd/knowledge integration-plan artifacts are stale; run make integration-plan" >&2; rm -rf "$$tmpdir"; exit 1); rm -rf "$$tmpdir"

integration-status:
	$(RUSTY_IDD) knowledge integration-status --workspace . --out .idd/knowledge/integration-status.json
	$(RUSTY_IDD) knowledge integration-status --workspace . --out .idd/knowledge/integration-status.md

integration-status-check:
	tmpdir=$$(mktemp -d) && $(RUSTY_IDD) knowledge integration-status --workspace . --out "$$tmpdir/integration-status.json" && $(RUSTY_IDD) knowledge integration-status --workspace . --out "$$tmpdir/integration-status.md" && cmp -s .idd/knowledge/integration-status.json "$$tmpdir/integration-status.json" && cmp -s .idd/knowledge/integration-status.md "$$tmpdir/integration-status.md" || (echo ".idd/knowledge integration-status artifacts are stale; run make integration-status" >&2; rm -rf "$$tmpdir"; exit 1); rm -rf "$$tmpdir"

integration-owners:
	$(RUSTY_IDD) knowledge integration-owners --workspace . --next --out .idd/knowledge/integration-owners.json
	$(RUSTY_IDD) knowledge integration-owners --workspace . --next --out .idd/knowledge/integration-owners.md

integration-owners-check:
	tmpdir=$$(mktemp -d) && $(RUSTY_IDD) knowledge integration-owners --workspace . --next --out "$$tmpdir/integration-owners.json" && $(RUSTY_IDD) knowledge integration-owners --workspace . --next --out "$$tmpdir/integration-owners.md" && cmp -s .idd/knowledge/integration-owners.json "$$tmpdir/integration-owners.json" && cmp -s .idd/knowledge/integration-owners.md "$$tmpdir/integration-owners.md" || (echo ".idd/knowledge integration-owners artifacts are stale; run make integration-owners" >&2; rm -rf "$$tmpdir"; exit 1); rm -rf "$$tmpdir"

integration-readiness:
	$(RUSTY_IDD) knowledge integration-readiness --workspace . --next --out .idd/knowledge/integration-readiness.json
	$(RUSTY_IDD) knowledge integration-readiness --workspace . --next --out .idd/knowledge/integration-readiness.md

integration-readiness-check:
	tmpdir=$$(mktemp -d) && $(RUSTY_IDD) knowledge integration-readiness --workspace . --next --out "$$tmpdir/integration-readiness.json" && $(RUSTY_IDD) knowledge integration-readiness --workspace . --next --out "$$tmpdir/integration-readiness.md" && cmp -s .idd/knowledge/integration-readiness.json "$$tmpdir/integration-readiness.json" && cmp -s .idd/knowledge/integration-readiness.md "$$tmpdir/integration-readiness.md" || (echo ".idd/knowledge integration-readiness artifacts are stale; run make integration-readiness" >&2; rm -rf "$$tmpdir"; exit 1); rm -rf "$$tmpdir"

plan-context:
	$(RUSTY_IDD) knowledge plan-context --workspace . --out .idd/knowledge/plan-context.json --change "$(RUSTY_IDD_CHANGE)" --goal "$(RUSTY_IDD_GOAL)"
	$(RUSTY_IDD) knowledge plan-context --workspace . --out .idd/knowledge/plan-context.md --change "$(RUSTY_IDD_CHANGE)" --goal "$(RUSTY_IDD_GOAL)"

plan-context-check:
	tmpdir=$$(mktemp -d) && $(RUSTY_IDD) knowledge plan-context --workspace . --out "$$tmpdir/plan-context.json" --change "$(RUSTY_IDD_CHANGE)" --goal "$(RUSTY_IDD_GOAL)" && $(RUSTY_IDD) knowledge plan-context --workspace . --out "$$tmpdir/plan-context.md" --change "$(RUSTY_IDD_CHANGE)" --goal "$(RUSTY_IDD_GOAL)" && cmp -s .idd/knowledge/plan-context.json "$$tmpdir/plan-context.json" && cmp -s .idd/knowledge/plan-context.md "$$tmpdir/plan-context.md" || (echo ".idd/knowledge plan-context artifacts are stale; run make plan-context" >&2; rm -rf "$$tmpdir"; exit 1); rm -rf "$$tmpdir"

codex-env-check:
	$(RUSTY_IDD) codex env-check

codex-model-loop:
	$(RUSTY_IDD) codex model-loop

codex-runtime-audit:
	$(RUSTY_IDD) codex runtime-audit

codex-system-audit:
	$(RUSTY_IDD) codex system-audit

ci: build test validate manifest-check knowledge-check operating-model-check integration-plan-check integration-status-check integration-owners-check integration-readiness-check plan-context-check codex-env-check codex-runtime-audit codex-model-loop fmt-check lint audit

install-hooks:
	git config core.hooksPath .githooks

clean:
	$(CARGO) clean
