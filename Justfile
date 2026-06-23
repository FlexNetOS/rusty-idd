set dotenv-load := false

cargo := env_var_or_default("CARGO", "cargo")
rusty_idd := cargo + " run --bin rusty-idd --"
change := env_var_or_default("RUSTY_IDD_CHANGE", "upgrade-codex-harness-rusty-idd-flow")
goal := env_var_or_default("RUSTY_IDD_GOAL", "update the .codex agent harness and related files to reflect the true flow of rusty-idd. Ai_merge is a tool for rusty_idd and must be treated as one and removed as the main intent")

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

render:
    {{rusty_idd}} render --all

render-check:
    {{rusty_idd}} render --all --check

knowledge:
    {{rusty_idd}} knowledge refresh --workspace .

system-architecture:
    {{rusty_idd}} knowledge system-architecture --workspace . --system-root .. --out .idd/knowledge/system-architecture.json
    {{rusty_idd}} knowledge system-architecture --workspace . --system-root .. --out .idd/knowledge/system-architecture.md

operating-model:
    {{rusty_idd}} knowledge operating-model --workspace . --out .idd/knowledge/operating-model.json
    {{rusty_idd}} knowledge operating-model --workspace . --out .idd/knowledge/operating-model.md

operating-model-check:
    tmpdir=$(mktemp -d) && {{rusty_idd}} knowledge operating-model --workspace . --out "$tmpdir/operating-model.json" && {{rusty_idd}} knowledge operating-model --workspace . --out "$tmpdir/operating-model.md" && cmp -s .idd/knowledge/operating-model.json "$tmpdir/operating-model.json" && cmp -s .idd/knowledge/operating-model.md "$tmpdir/operating-model.md" || { echo ".idd/knowledge operating-model artifacts are stale; run just operating-model" >&2; rm -rf "$tmpdir"; exit 1; }; rm -rf "$tmpdir"

integration-plan:
    {{rusty_idd}} knowledge integration-plan --workspace . --out .idd/knowledge/integration-plan.json
    {{rusty_idd}} knowledge integration-plan --workspace . --out .idd/knowledge/integration-plan.md

integration-plan-check:
    tmpdir=$(mktemp -d) && {{rusty_idd}} knowledge integration-plan --workspace . --out "$tmpdir/integration-plan.json" && {{rusty_idd}} knowledge integration-plan --workspace . --out "$tmpdir/integration-plan.md" && cmp -s .idd/knowledge/integration-plan.json "$tmpdir/integration-plan.json" && cmp -s .idd/knowledge/integration-plan.md "$tmpdir/integration-plan.md" || { echo ".idd/knowledge integration-plan artifacts are stale; run just integration-plan" >&2; rm -rf "$tmpdir"; exit 1; }; rm -rf "$tmpdir"

integration-status:
    {{rusty_idd}} knowledge integration-status --workspace . --out .idd/knowledge/integration-status.json
    {{rusty_idd}} knowledge integration-status --workspace . --out .idd/knowledge/integration-status.md

integration-status-check:
    tmpdir=$(mktemp -d) && {{rusty_idd}} knowledge integration-status --workspace . --out "$tmpdir/integration-status.json" && {{rusty_idd}} knowledge integration-status --workspace . --out "$tmpdir/integration-status.md" && cmp -s .idd/knowledge/integration-status.json "$tmpdir/integration-status.json" && cmp -s .idd/knowledge/integration-status.md "$tmpdir/integration-status.md" || { echo ".idd/knowledge integration-status artifacts are stale; run just integration-status" >&2; rm -rf "$tmpdir"; exit 1; }; rm -rf "$tmpdir"

integration-owners:
    {{rusty_idd}} knowledge integration-owners --workspace . --next --out .idd/knowledge/integration-owners.json
    {{rusty_idd}} knowledge integration-owners --workspace . --next --out .idd/knowledge/integration-owners.md

integration-owners-check:
    tmpdir=$(mktemp -d) && {{rusty_idd}} knowledge integration-owners --workspace . --next --out "$tmpdir/integration-owners.json" && {{rusty_idd}} knowledge integration-owners --workspace . --next --out "$tmpdir/integration-owners.md" && cmp -s .idd/knowledge/integration-owners.json "$tmpdir/integration-owners.json" && cmp -s .idd/knowledge/integration-owners.md "$tmpdir/integration-owners.md" || { echo ".idd/knowledge integration-owners artifacts are stale; run just integration-owners" >&2; rm -rf "$tmpdir"; exit 1; }; rm -rf "$tmpdir"

integration-readiness:
    {{rusty_idd}} knowledge integration-readiness --workspace . --next --out .idd/knowledge/integration-readiness.json
    {{rusty_idd}} knowledge integration-readiness --workspace . --next --out .idd/knowledge/integration-readiness.md

integration-readiness-check:
    tmpdir=$(mktemp -d) && {{rusty_idd}} knowledge integration-readiness --workspace . --next --out "$tmpdir/integration-readiness.json" && {{rusty_idd}} knowledge integration-readiness --workspace . --next --out "$tmpdir/integration-readiness.md" && cmp -s .idd/knowledge/integration-readiness.json "$tmpdir/integration-readiness.json" && cmp -s .idd/knowledge/integration-readiness.md "$tmpdir/integration-readiness.md" || { echo ".idd/knowledge integration-readiness artifacts are stale; run just integration-readiness" >&2; rm -rf "$tmpdir"; exit 1; }; rm -rf "$tmpdir"

plan-context:
    if [ -n "$RUSTY_IDD_GOAL_FILE" ]; then {{rusty_idd}} knowledge plan-context --workspace . --out .idd/knowledge/plan-context.json --change "{{change}}" --goal-file "$RUSTY_IDD_GOAL_FILE"; else goal="${RUSTY_IDD_GOAL:-{{goal}}}"; {{rusty_idd}} knowledge plan-context --workspace . --out .idd/knowledge/plan-context.json --change "{{change}}" --goal "$goal"; fi
    if [ -n "$RUSTY_IDD_GOAL_FILE" ]; then {{rusty_idd}} knowledge plan-context --workspace . --out .idd/knowledge/plan-context.md --change "{{change}}" --goal-file "$RUSTY_IDD_GOAL_FILE"; else goal="${RUSTY_IDD_GOAL:-{{goal}}}"; {{rusty_idd}} knowledge plan-context --workspace . --out .idd/knowledge/plan-context.md --change "{{change}}" --goal "$goal"; fi

plan-context-check:
    tmpdir=$(mktemp -d) && if [ -n "$RUSTY_IDD_GOAL_FILE" ]; then {{rusty_idd}} knowledge plan-context --workspace . --out "$tmpdir/plan-context.json" --change "{{change}}" --goal-file "$RUSTY_IDD_GOAL_FILE" && {{rusty_idd}} knowledge plan-context --workspace . --out "$tmpdir/plan-context.md" --change "{{change}}" --goal-file "$RUSTY_IDD_GOAL_FILE"; else goal="${RUSTY_IDD_GOAL:-{{goal}}}"; {{rusty_idd}} knowledge plan-context --workspace . --out "$tmpdir/plan-context.json" --change "{{change}}" --goal "$goal" && {{rusty_idd}} knowledge plan-context --workspace . --out "$tmpdir/plan-context.md" --change "{{change}}" --goal "$goal"; fi && cmp -s .idd/knowledge/plan-context.json "$tmpdir/plan-context.json" && cmp -s .idd/knowledge/plan-context.md "$tmpdir/plan-context.md" || { echo ".idd/knowledge plan-context artifacts are stale; run just plan-context" >&2; rm -rf "$tmpdir"; exit 1; }; rm -rf "$tmpdir"

knowledge-check:
    tmpdir=$(mktemp -d) && {{rusty_idd}} knowledge index --workspace . --out "$tmpdir/index.json" && {{rusty_idd}} knowledge report --workspace . --out "$tmpdir/report.md" && {{rusty_idd}} knowledge architecture --workspace . --out "$tmpdir/architecture.json" && {{rusty_idd}} knowledge architecture --workspace . --out "$tmpdir/architecture.md" && cmp -s .idd/knowledge/index.json "$tmpdir/index.json" && cmp -s .idd/knowledge/report.md "$tmpdir/report.md" && cmp -s .idd/knowledge/architecture.json "$tmpdir/architecture.json" && cmp -s .idd/knowledge/architecture.md "$tmpdir/architecture.md" || { echo ".idd/knowledge artifacts are stale; run just knowledge" >&2; rm -rf "$tmpdir"; exit 1; }; rm -rf "$tmpdir"

diagrams:
    {{rusty_idd}} knowledge diagrams --workspace . --out docs/rusty-idd/architecture-diagrams.md

diagrams-check:
    tmp=$(mktemp) && {{rusty_idd}} knowledge diagrams --workspace . --out "$tmp" && cmp -s docs/rusty-idd/architecture-diagrams.md "$tmp" || { echo "docs/rusty-idd/architecture-diagrams.md is stale; run just diagrams" >&2; rm -f "$tmp"; exit 1; }; rm -f "$tmp"

codex-env-check:
    {{rusty_idd}} codex env-check

codex-model-loop:
    {{rusty_idd}} codex model-loop

codex-runtime-audit:
    {{rusty_idd}} codex runtime-audit

codex-system-audit:
    {{rusty_idd}} codex system-audit

ci: build test validate manifest-check knowledge-check diagrams-check operating-model-check integration-plan-check integration-status-check integration-owners-check integration-readiness-check plan-context-check codex-env-check codex-runtime-audit codex-model-loop fmt-check lint audit
