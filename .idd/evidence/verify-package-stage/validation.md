Change: add-verify-package-stage

Commands run for this artifact slice:

- `rtk cargo run --bin rusty-idd -- knowledge plan-context --workspace . --out .idd/knowledge/plan-context.md --change add-verify-package-stage --goal-file .idd/goals/add-verify-package-stage.md`
  - Result: passed; wrote graph planning context to .idd/knowledge/plan-context.md.
- `rtk cargo run --bin rusty-idd -- spec status openspec/changes/add-verify-package-stage`
  - Result: passed; proposal, specs, design, ADR, and tasks complete; archivable yes.
- `rtk cargo run --bin rusty-idd -- knowledge plan-context --workspace . --out .idd/knowledge/plan-context.json --change add-verify-package-stage --goal-file .idd/goals/add-verify-package-stage.md`
  - Result: passed; wrote JSON graph planning context to .idd/knowledge/plan-context.json.
- `rtk cargo run --bin rusty-idd -- knowledge refresh --workspace .`
  - Result: passed; refreshed index, report, architecture JSON, and architecture markdown.
- `rtk cargo run --bin rusty-idd -- validate --workspace .`
  - Result: passed twice; 0 critical, 0 warning.
- `rtk cargo run --bin rusty-idd -- manifest --workspace . --out .idd/MANIFEST.tsv`
  - Result: passed twice; wrote 3474 manifest entries.
- `rtk git diff --check`
  - Result: passed.

Final status: planning artifacts are ready for implementation of the verify
package stage.
