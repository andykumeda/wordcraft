# WordCraft v0.15 assistant integration

Status: **Deployed on both Macs** — WordCraft Assistant Preview0.15.2, source `fccdfe38`, 2026-11-04. See [the current update ledger](#official-v0152-reconciliation--2026-11-04). Earlier checkpoints are historical.

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
| MacBook standard and Assistant Preview | Deployed | After the user saved/closed Preview and restored access, both dbacf0c4 bundles were atomically installed and independently passed the same helper0.1.2/native/settings/MCP checks; prior apps retained. |

Both-Mac installation and native acceptance are complete. No user-owned preview was force-quit, and no provider inference was requested. Exact MacBook receipts and the resolved connection/test-harness checkpoint are in the candidate qualification below.

## View toolbar refinement — 2026-11-04

| Work | State | Acceptance and evidence |
|---|---|---|
| Move View Assistant | Verified | Pin the existing icon/label control at the ribbon's far right, outside the horizontally scrolling groups; retain `ui.assistant`, selected state, tooltip and accessible name. Home's existing Assistant stays in its group. |
| Verification | Verified | All 39 ribbon tests pass, including 600/800/1440-point View widths, light/dark palettes, far-right placement, pointer open/keyboard Space close and unchanged document text. The new regression fails with the old layout at 600 points. Scoped all-target Clippy with `-D warnings`, formatting, asset attribution and layering pass. |
| Visual review and wasm | Verified | Offscreen source renders at 600 and 1440 points in light/dark were inspected: the original comment icon and Assistant label remain visible at the far right while narrow groups scroll. The wasm constituent passes for all 15 supported crates, including the UI. |
| Install/package/release | Not started | Existing installed apps and other-Mac artifacts remain unchanged. |

No provider, model, permission, conversation or document API changes are included.

Verification used the existing `ribbon::tests` suite, scoped UI all-target Clippy and the existing
asset/layer/wasm checks. UI compilation used a temporary package `opt-level=0` override and
`CARGO_INCREMENTAL=0` after an interrupted optimized build left unusable incremental linker
artifacts; no build configuration or dependency changes were committed. The full workspace CI
sweep was not run during concurrent app verification. Native VoiceOver, live-provider inference,
installed-app and other-Mac acceptance were not repeated for this placement-only source change.


## Shared helper 0.1.4 delivery — 2026-11-04

| Work | State | Acceptance and evidence |
|---|---|---|
| Hide tool-call and result rows | Deployed | Shared helper 0.1.4 installed with pinned local signature on Mini and MacBook; prior helper versions retained. App binaries and configuration were unchanged. |
| Actual configured-model acceptance | Verified | Codex account `gpt-6.1-terra` edited a disposable document on both Macs; Undo, Redo, save and reopen passed. Mini 15.11 seconds, MacBook 13.18 seconds. The instruction not to save was followed; the test then saved to verify persistence. Conversation screenshots show no tool rows. |
| Remaining checks | Not started | Native VoiceOver, IME and clipboard. The separate View toolbar source change remains uninstalled. |

Cloud-provider routing can send document task content to the selected provider. Existing windows
need Assistant closed and reopened to adopt the new helper. No public release was made.
See the [shared rollout ledger](https://github.com/andykumeda/craft-libs/blob/codex/independent-assistant/docs/two-mac-rollout.md) for package hashes and receipts.

The required `cargo xtask ci` passed all nine steps with the pinned `CRAFT_I18N_DIR`
(`059cbf95`) and open-font input configured. An initial run without language data passed fmt
and Clippy but failed the German hyphenation test; the corrected run passed. Logs are under
`/private/tmp/craft-assistant-014-20261104/`.

## Official v0.15.2 reconciliation — 2026-11-04

| Work | State | Acceptance / evidence |
|---|---|---|
| Verify official source | Verified | Published GitHub release v0.15.2 points to `edfea68101220c9bdc4b2b0ccf4acd9612bedb6e`; fetched exact tag from storytold/wordcraft. |
| Reconcile custom host | Verified | Merged the official tag while preserving embedded assistant, model-menu flicker fix, far-right View control, isolated preview profile and disabled official updater. Upstream RTL, fitted ribbon, modal dialog, language/proofing and document fidelity changes retained. |
| Source verification | Verified | All nine required CI steps passed, including 5202 ordinary native tests (9 ignored), translation audit and all15 WASM crates. Release-pinned craft-i18n `9d6d54d6d9ff603cc41e12e5f6ba51af3554ab66` and existing open-font input `b076417b5880bd4154bd22f00695b1d28c6bea83`; log `/tmp/wordcraft-v0152-ci.log`. |
| Native model/package/two-Mac install | Deployed | Signed candidate model checks passed before activation on each Mac; mounted/copied DMG and installed-path native checks passed. Both exact `/Applications/WordCraft Assistant Preview.app` paths are0.15.2(fccdfe38). Mini rollback retained. |

Generated package instructions now describe the existing normal Codex profile rather than the retired isolated account flow. No new provider, permission, profile or conversation behavior is introduced. The installed shared
helper0.1.6 remains separate. Standard WordCraft installation is coordinated independently; this
work owns only `/Applications/WordCraft Assistant Preview.app`. No public release is implied.

### Two-Mac delivery receipt

Release source: **`fccdfe385a07e70f51ab71f9544e0be08f7a0484`**, following merge
`076ea94c`. Cargo normalized the merge's ambiguous AES/HMAC dependency edges; the normalized
working bytes were what the complete passing CI and build used. The lock-only follow-up retains
those exact tested resolutions. No new runtime feature was added beyond the upstream merge.
The final required-input release desktop/CLI build passed in21m16s. The MCP client used for QA
is byte-identical to that final CLI (`94187b148f62392eaa418fde1d115d59896347b29aece107419ac821bcb1f647`).

| Acceptance | Mini | MacBook |
|---|---|---|
| Actual configured model | Codex account / gpt-6.1-terra,29.21s | Codex account / gpt-6.1-terra,14.15s |
| Exact synthetic text; no premature save | Passed full decoded-text equality and unchanged saved-file bytes | Passed same independent oracles |
| One Undo, Redo, explicit DOCX save/reopen | Passed | Passed |
| Signed installed-path native smoke |17 MCP tools; edit/read/Undo/Redo/save/reopen/render; helper0.1.6 handshake | Same checks passed |
| Own test cleanup | Normal quit, listener released, owner-only key removed | Same checks passed |

The two actual chat screenshots were inspected: the requested four-word text is visible, the
original Assistant icon/toggle stays at the far right of View, and helper tool-call rows stay
hidden. Model latency is an observation during the rollout, not a performance comparison.
The installed-path smoke did not repeat inference after the identical signed candidate bytes
had already passed it. Native VoiceOver, IME and clipboard acceptance remain unperformed.

- Installed native SHA-256 on both Macs: `e956b241a39c106f9c3b8ce32bf229df0587134f0741d24c7814d4ccb1b969aa`.
- DMG: `dist/assistant-preview/fccdfe38/wordcraft-assistant-preview-0.15.2-fccdfe38-macos-aarch64.dmg`; SHA-256 `92f75302526e5c6d5b7e2a968d6f6e164f64003b4643c23e3865323f2f6d3f24`.
- ZIP: same prefix; SHA-256 `328286ba4cdc3c648f27f1f96474c761199e857065f4e18e2d6309bbbd3a7cd3`.
- Matching DMG/checksum and installation instructions are in both users' Downloads. Both mounted DMGs and copied apps passed signatures, version/source and native-byte checks before installation.
- Mini previous0.15.0(dbacf0c4) bundle, signature verified after retention: `/Users/andy/Applications/WordCraft Preview Backups/WordCraft Assistant Preview-replaced-fccdfe38-20261104-205921.app`. Book had no bundle at the target, so no preview was replaced there. Other legacy paths remain untouched.
- Both bundles keep ID `ai.storyteller.wordcraft.assistant-preview`, local signer `stt-dev`, separate Word profile and launcher `--disable-updates`. No notarization or public release. Installation touched app bundles and LaunchServices registration, without copying/removing profiles, credentials or conversations.
- The installed shared helper0.1.6 and production connection/model settings remain separate and were not changed. Each QA copied public AI configuration into its own private store, used a private Codex workspace and retained the normal local HOME/CODEX_HOME account. No credentials were transferred between Macs. Selected cloud inference receives the task's document context; no local-only privacy claim.

Private receipts: Mini `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/wordcraft-v0152-smoke-97ehhbox/report.json` (actual model), `/tmp/wordcraft-v0152-installed-mini.log` and `/tmp/wordcraft-v0152-install-mini.log`; MacBook `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-v0152-smoke-wq0njjf7/report.json` (actual model), `/private/tmp/wordcraft-v0152-fccdfe38/{installed-book,install-book}.log`. Package receipt `/private/tmp/wordcraft-v0152-fccdfe38/package-receipt.json`; source CI `/tmp/wordcraft-v0152-ci.log`, final release `/tmp/wordcraft-v0152-release-final.log`, packaging `/tmp/wordcraft-v0152-package.log`. Screenshots and synthetic documents remain private local QA outputs.

The standard WordCraft apps are a separately owned official-release rollout. This preview task
never force-quit a user instance or sent commands to an existing personal document window.
