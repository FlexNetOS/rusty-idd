# Prompt Front Door Upstream Adoption

## Scope

This note records the adopt-first pass for `integrate-prompt-front-door`.

The Rusty IDD owner surface is `repo:prompt-hub`. The external adopt-first
inputs are:

- `github.com/f/prompts.chat`
- `github.com/f/ai-prompt`

## Owner Repo State

`prompt_hub` was inspected as a peer owner repo at:

- remote: `git@github.com:FlexNetOS/prompt_hub.git`
- head: `4b17d6527fc9c235e65ebaaf4174c5d397711c69`
- branch: `main`
- status: dirty before this slice (`D .output.txt`, untracked
  `worktrees/`)

Native diagnostics:

- `cd ../prompt_hub && cargo metadata --locked --format-version 1`: passed
- `cd ../prompt_hub && cargo test --workspace --all-features --locked`:
  passed, 1206 passed / 4 ignored

The dirty peer state was not modified by this Rusty IDD slice.

## Upstream Pins

Pinned upstream revisions:

- `github.com/f/prompts.chat`:
  `22a2badf58d6b76ee2d799298492f265aa8aa08d`
- `github.com/f/ai-prompt`:
  `59db2b139745be5cbf773eb630fec02940468c54`

Tracked source mirrors:

- `third_party/upstream/prompts.chat`
- `third_party/upstream/ai-prompt`

The mirrors preserve upstream source, scripts, docs, tests, examples, package
metadata, CI files, plugin assets, and lockfiles. Generated install/build
outputs were excluded (`.git`, `node_modules`, `.next`, `dist`, `build`,
`coverage`) because they are local diagnostic byproducts, not upstream source.

## Native Diagnostics

`github.com/f/ai-prompt`:

- `npm ci`: passed
- `npm run build`: passed
- `npm run lint:js`: passed
- `npm run lint:css`: passed
- observed toolchain: upstream CI uses Node 20; local environment was
  Node v22.22.3 / npm 10.9.8
- npm audit after install reported 62 vulnerabilities from upstream dependency
  graph: 2 low, 53 moderate, 6 high, 1 critical

`github.com/f/prompts.chat`:

- `npm ci`: failed without `DATABASE_URL` during `prisma generate`
- `DATABASE_URL="postgresql://test:test@localhost:5432/test" npm ci`: passed
- `DATABASE_URL="postgresql://test:test@localhost:5432/test" npm run lint`:
  passed with 206 upstream warnings
- `DATABASE_URL="postgresql://test:test@localhost:5432/test" npm test`:
  passed, 44 files / 709 tests
- observed toolchain: package metadata requires Node 24.x; local environment was
  Node v22.22.3 / npm 10.9.8 and emitted `EBADENGINE`
- npm audit after install reported 65 vulnerabilities from upstream dependency
  graph: 2 low, 40 moderate, 18 high, 5 critical

No upstream server, MCP transport, database service, or host daemon was started.

## Rusty IDD Boundary

The local boundary added in this slice is
`rusty-idd knowledge integration-readiness` support for external
`adopt_first_inputs`.

The readiness report now records:

- source and tracked mirror path
- required tools (`git`, `node`, `postgres`, `wordpress`)
- native diagnostic commands
- runtime assumptions such as Node 24 and `DATABASE_URL`
- feature gates for web/MCP/server/plugin surfaces

This preserves `crates/core` as std-only and keeps host services out of default
Rusty IDD workflows.

## Automation Environment Context

This slice treats prompt/front-door work as part of the wider Rusty IDD
automation flow rather than as a standalone merge checklist:

- `prompt_hub` remains the user-facing producer/front door for prompts that
  should become OpenSpec plans, ADRs, specs, tasks, implementation work, and
  validation evidence.
- Yazelix is the current terminal/parser/runtime surface and carries the
  tree-sitter direction, nushell, Lua, Ghostty, Zellij, and contributor-tool
  expectations. This slice does not downgrade those assumptions.
- RTK is the foundational command/tool wrapper, with rtk-ai surfaces including
  ICM, VOX, and GRIT. Repository commands in this work were run through
  `rtk proxy` so tool execution stays inside the parent-managed environment.
- Beads is mandatory for future code contributors through Yazelix. The current
  system operating model already records the Beads upstream anchors under
  `capability:github-agent-run-upgrades`; this prompt-front-door slice only
  records readiness and does not select or vendor a canonical Beads
  implementation.
- Missing tools required by upstreams remain parent/meta/envctl-managed
  requirements. The local diagnostics observed Node and Postgres assumptions,
  but no user-global tool install or host service start was performed.

## Valuable Upstream Surfaces

`prompts.chat` contributes prompt registry, prompt package, CLI, plugin,
Claude plugin, skill metadata, MCP/server code, Prisma data model, tests, and
prompt quality/parser utilities.

`ai-prompt` contributes a WordPress/Gutenberg prompt display block, prompt
rendering UI, blueprint/demo metadata, plugin packaging scripts, examples, and
lint/build surfaces.

For the next implementation pass, `prompt_hub` should stay the producer/front
door and Rusty IDD should consume structured prompt intent into OpenSpec
artifacts. Web/MCP/server/plugin runtime activation requires an explicit feature
boundary.

## Consolidation / Cuts

No upstream features were downgraded or replaced in this slice.

Cuts made:

- Excluded `.git` directories from mirrors after exact revisions were recorded.
- Excluded generated install/build outputs (`node_modules`, `.next`, `dist`,
  `build`, `coverage`).

No prompt registry, plugin, MCP/server, Prisma, WordPress/Gutenberg, lint, test,
or packaging feature was replaced with a local guess. Runtime activation remains
feature-gated until the owning repo/spec makes it explicit.

Rollback:

1. Revert `third_party/upstream/prompts.chat` and
   `third_party/upstream/ai-prompt`.
2. Revert the `IntegrationUpstreamInput` readiness model.
3. Re-run focused owner/upstream diagnostics plus Rusty IDD gates.
4. Regenerate `.idd/knowledge/*` and `.idd/MANIFEST.tsv`.
