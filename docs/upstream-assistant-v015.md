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
| Native/model acceptance | In progress | No installed-app, real-provider, VoiceOver or other-Mac acceptance is implied by compilation and synthetic tests. |
| Candidate packaging | Verified | Build release app and CLI with the exact required font/translation inputs; sign both app variants locally, verify notices/metadata/signatures and report hashes; no installation. |
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

## App restart guard follow-up

The initial v0.15 integration's app updater guarded document/save/print state but could still
restart a clean app during an assistant turn. The follow-up pins shared
`986cb16234cfdac1ee1337fb4cdf00945cad4e5a` and also requires assistant restart safety.
Native turns, settings jobs, file choice, reentrant dispatch, drafts and unsaved settings block
restart without cancelling them. Shared mode additionally guards host turn/dispatch/chooser
state and uses `Controller::restart_safe()` for queued helper requests and unacknowledged input.
Missing or failed helpers do not permanently block updates with stale busy flags.

The controller is conservative about pending render-only frames: finish or stop the chat, save
assistant settings and close the panel before restarting if an update remains deferred.

Verification against the new exact Git pin: assistant filter **36 passed / 2 explicitly
ignored**, including all three new app-update guard regressions; all-target UI/MCP/app/CLI
Clippy and the web-target check passed. Shared accessor tests: **23 passed**, including its two
new regressions, with all-target Clippy. No wire contract changes or installed app changes.
The earlier 97f3e512 release build succeeded with required font/translation inputs; final
candidate binaries are rebuilt from the guard follow-up commit before signing.


## Local candidate and installed smoke evidence

Signed standard/preview bundles and the matching CLI are prepared from `8762d986`; both
installed variants and the versioned CLI matched the candidate signer/hashes on the Mini and
MacBook. Four isolated background installed-app runs passed MCP edit/read/Undo/Redo,
save/reopen, page rendering and helper 0.1.0 readiness, with all temporary listeners/keys
cleaned up. No inference or deployment edits were performed by these checks. Exact artifacts,
hashes, report references and qualification limits are in
[the local qualification record](wordcraft-015-local-qualification.md).

## Compact assistant rollout — 2026-11-03

| Workstream / owner | State | Acceptance / evidence | Delivery |
| --- | --- | --- | --- |
| Shared UI pin / Word owner | Verified | Pin host libraries 83e61084; exact workspace CI all nine stages passed; scoped assistant/control and packaging metadata checks passed | Source only |
| Signed desktop candidates / Word owner | Verified | Source dbacf0c4, pinned required inputs, actual compiled source/arm64, exact local certificate/IDs, ZIP/DMG mounted-copy signatures and notice-byte checks passed | Ready for primary installation |

The independently installed panel uses New chat/Chat history controls and a gear for settings,
keeps update checks inside settings, and has compact
file and model controls in the rounded composer. Existing model validation, profiles and
credentials are retained. The helper updates independently; moving the host's update controls
requires this one-time desktop rebuild. Older helpers retain their update toolbar until updated.
No installed app, CLI, user configuration or public release has changed in this workstream.

The first exact workspace CI passed formatting and workspace strict lint, then found a stale
language-test fixture: `review_language_validates_tags_and_marks_text_unchecked` assumed
French had no proofing tools. The unchanged pinned proofing implementation supports installed
French dictionaries. Its existing test binary reproduced the failure in 0.09s. Only the
synthetic unsupported-language fixture is changed to reserved `zxx` (no linguistic content);
production proofing, dictionaries, configuration and preceding validation assertions are unchanged.
The exact workspace filtered test passed, followed by the complete CI rerun: all nine stages passed.

The host pin remains 83e61084 while compatible helper UI revisions can advance independently; the subsequent 0.1.2 helper refinement requires no further WordCraft rebuild.

Final source verification: `CARGO_BUILD_JOBS=1 RUST_TEST_THREADS=1 cargo xtask ci` with
required pinned fonts/proofing passed formatting, workspace/all-target Clippy with warnings
denied, workspace tests, i18n audit/report, language embedding, doctests, assets (40),
layering (19 crates) and WASM (15 crates). Engine tests passed 1148; UI passed 1059 with five
explicitly ignored tests; integration suites passed. Earlier focused checks passed assistant 36
(two ignored), control 26 and packaging metadata/shell syntax. Full CI was executed; live
provider, manual visual captures and installed-app acceptance remain separate checks.

Optimized desktop build from `dbacf0c4b5c9571ad1c03df83efb926895ee7444` succeeded with
`WORDCRAFT_BUILD_SHA` set to that exact commit. Both variants report `wordcraft 0.15.0
(dbacf0c4)`, are arm64, and carry signed protocol 1 / repository `andykumeda/wordcraft`
metadata. Supported packagers verified standard/preview mounted DMG copies and preview ZIP
extraction. Independent certificate/identifier, source/version and all license/notice byte
checks passed. `APPLE_ID`, `APPLE_PASSWORD` and `APPLE_TEAM_ID` were explicitly unset; local
certificate signing only, no notarization or public release.

Artifacts and hashes are in [the candidate qualification](wordcraft-015-local-qualification.md#compact-layout-candidate--2026-11-03).
The installed CLI was not changed. An unused CLI archive produced by the standard packager
uses its pre-rollout cached executable and is excluded from this app delivery. Primary owns
both-Mac installation, preserved-user-instance checks and final native qualification.

## Compact-layout installation checkpoint — 2026-11-03

| Device | State | Actual evidence / remaining action |
| --- | --- | --- |
| Mini standard and Assistant Preview | Deployed | Source dbacf0c4 installed atomically; matching signed candidate hashes. Both isolated native runs passed helper 0.1.2, gear/settings-only updates, 17 MCP tools, synthetic edit/read/Undo/Redo, save/close/reopen and PNG rendering; owned listeners/keys cleaned up. |
| MacBook standard and Assistant Preview | Blocked | New Word host candidates are verified and ready but were not installed: device access became unavailable before installation. A user-owned Preview instance had last been observed running; save/close it and restore access before replacement. |

The MacBook's previously installed helper 0.1.2 is a separate component; its earlier success
does not establish that the new Word host was installed. No user-owned preview was force-quit.
No provider inference was requested. Current scope is not complete on both Macs until the
MacBook host installation and independent native checks pass.
