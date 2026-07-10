#!/usr/bin/env bash
set -euo pipefail
root="${1:-.handoff/loop/plan}"
required=(
  targets.md
  graph/target-dag.json
  graph/target-dag.md
  reports/lifeos-meta-integration-map.md
  reports/odysseus-front-door-evaluation.md
  reports/component-ownership-matrix.md
  reports/automation-roadmap.md
  risk-policy.md
  agent-backend-matrix.md
  agent-interop.md
)
for f in "${required[@]}"; do
  test -s "$root/$f" || { echo "missing-or-empty: $root/$f" >&2; exit 1; }
done
for pattern in \
  'findings/memory-vector-intelligence-*.md' \
  'findings/autoresearch-*.md' \
  'findings/rules-policy-org-*.md' \
  'findings/distributed-compute-*.md' \
  'findings/filesystem-layout-*.md' \
  'findings/prompt-architecture-*.md' \
  'reports/agent-run-ledger-*.md'
do
  shopt -s nullglob
  matches=("$root"/$pattern)
  shopt -u nullglob
  ((${#matches[@]} > 0)) || { echo "missing pattern: $root/$pattern" >&2; exit 1; }
  for f in "${matches[@]}"; do test -s "$f" || { echo "empty: $f" >&2; exit 1; }; done
done
python3 -m json.tool "$root/graph/target-dag.json" >/dev/null
# Require source-key blocks and at least one bracket citation in every markdown artifact.
while IFS= read -r -d '' f; do
  grep -q '## Source keys' "$f" || { echo "no source keys: $f" >&2; exit 1; }
  grep -Eq '\[[A-Z][0-9]\]' "$f" || { echo "no bracket citations: $f" >&2; exit 1; }
done < <(find "$root" -type f -name '*.md' -print0)
echo "plan-artifact-gate: ok ($root)"
