# WordCraft v0.15 assistant integration

Status: In progress. Source reconciliation only; installed apps and published releases are unchanged.

The isolated task branch combines assistant commit `05d5fb21` with upstream `origin/main`
`ab0020d6` (WordCraft 0.15.0). This retains the embedded, independently installed assistant
and its complete live MCP client interface while taking upstream document, proofing,
interface and file-format fixes. The original assistant checkout remains separate.

## Task ledger

| Work | State | Acceptance and evidence |
|---|---|---|
| Reconcile upstream source | Verified | Preserve assistant hooks, all MCP tools, proofing pane, inspection fields and updater opt-out; resolve source and documentation conflicts. |
| Update shared dependencies | Verified | Use the compatible committed shared fork revision; keep upstream proof/font-store dependencies and registry versions. Temporary verification path patches must never enter the committed lockfile. |
| Scoped verification | Verified | Check desktop, CLI and UI; run assistant, control and MCP regressions plus relevant Clippy. Record actual results below. |
| Native/model acceptance | Not started | No installed-app, real-provider, VoiceOver or other-Mac acceptance is implied by compilation and synthetic tests. |
| Candidate packaging | In progress | Build release app and CLI with the exact required font/translation inputs; sign both app variants locally, verify notices/metadata/signatures and report hashes; no installation. |
| Public release | Not started | No public release or main-branch push. |

## Resolution decisions

- Keep both the assistant and upstream Proofing pane cases and command routing.
- Preserve both interface theme/read-page-color diagnostics and upstream proofing inspection.
- Retain the explicit update opt-out around Windows updater completion while adopting the
  upstream app metadata. Customized assistant builds keep the configured compatible-fork
  updater and never fall back to upstream release assets.
- Retain upstream engine regression tests alongside the assistant background-color/Undo tests.
- Omit upstream trademark artwork and present this build as customized WordCraft, retaining
  plain upstream attribution, copyright notices and the original trademark terms.
- Copy both embedded Latin font licenses and pinned craft-fonts licenses into signed resources.
- Add only explicitly paired package source/protocol metadata; preserve the preview's distinct
  bundle/profile and update opt-out.
- Keep upstream progress statements and assistant history; historical preview/model claims
  remain historical. Current MCP parity and its limits are in [the parity record](mcp-assistant-parity.md).

## Verification

Initial UI/desktop/CLI checks passed against compatible shared source and actual Git `c74e48b`.
Final shared Git `3ff2754eeb1c079612247f4d288a22a20c169ac9` passed:

- UI assistant regressions: 33 passed, two explicitly ignored (live provider and manual capture).
- UI control regressions: 26 passed; updater regressions: two passed.
- MCP: 25 unit tests and six automation-root integration tests passed. Tests were built with
  the UI package's unified dependency features; bounded loopback fixtures needed local socket
  permission, with no configured service or installed-app changes.
- All-target Clippy for UI, MCP, desktop app and CLI passed with `-D warnings`.

The web-target check passed for `wasm32-unknown-unknown`; the final cached updater rerun passed
two tests. The two manually merged engine inspection/background-color/Undo regressions also
passed. The lock has one fork source for all
14 shared packages, no local path packages, and retains all upstream registry versions.

`cargo xtask assets` passed: all 40 remaining asset files attributed. Full formatting and diff
checks passed. `bash -n` and the synthetic macOS metadata
script passed for standard and preview plists, integer bridge protocol, omitted configuration,
seven rejected invalid pairs and byte-exact license/notice copying with pinned font and
translation/proofing inputs. Input versions match the release workflow: craft-fonts
`b076417b5880bd4154bd22f00695b1d28c6bea83` and craft-i18n
`059cbf95c2a3eb95d637ca8584faca1129d17f48`. Full workspace CI and native/model acceptance
were not run; these are scoped source checks, not an installed release claim.

The earlier v0.12 assistant parity results are recorded separately and are not evidence that
this merged source passes. No installed-app or native/model acceptance has been performed here.
