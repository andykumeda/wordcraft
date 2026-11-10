# WordCraft v0.15 assistant integration

Status: **Deployed for WordCraft on both Macs**, 2026-11-05 (America/Los_Angeles).
The active app is `/Applications/WordCraft.app` with the integrated Assistant; its separate
Assistant Preview app and old MCP shortcut have been retired. This closes this app's workstream only. The
[suite consolidation ledger](https://github.com/andykumeda/pdfcraft/blob/codex/independent-assistant/docs/single-app-assistant-rollout.md) owns overall progress; this per-app deployment does not claim all four apps are complete.

## Current single-app deployment — 2026-11-05

| Component | Installed / verified state |
| --- | --- |
| Normal app | WordCraft 0.15.2, native source `847526d8838553ad2c15e8d3ab7db9795d3e7f48`; one title-bar Assistant sparkle beside Discord |
| Underlying official release | `v0.15.2`, commit `edfea68101220c9bdc4b2b0ccf4acd9612bedb6e` |
| Signed native SHA-256 | `ebd7751d715f6f6fcba306ad900811fd94c03184ff33377a7b60b247d6597f85` |
| Assistant helper | 0.1.7 on Mini and MacBook; bridge protocol1 unchanged |
| Update owner | Managed ArtCraft Launcher 0.14.1/source19170137; actual installed-launcher replacements passed on both Macs |
| Local signing | Exact `stt-dev` certificate `D7B1B5524FFEB37AB01D3960017BDF6743E3EDFD`; not notarized or a public release |

Use the **managed ArtCraft Launcher** to check official release freshness and update this app.
Its trusted source builder retains the Assistant, reconciles the requested official tag,
runs the required checks and produces a signed normal bundle. The launcher verifies the
standard identity, exact certificate, assistant protocol/repository and signed
`CraftUnderlyingTag`/`CraftUnderlyingCommit` before replacement. Failed builds, unavailable
build hosts or source conflicts leave the installed app intact; there is no stock-package
fallback. The MacBook uses signed artifacts from the Mini, which must be reachable for a new
build. The app's own updater is disabled by its standard wrapper. Preserve the managed launcher,
including its self-update protection; ordinary upstream app/launcher packages are not this route.

The previous native `fccdfe38` passed configured Codex/Terra/helper 0.1.6 candidate acceptance
independently on both Macs (24.62s Mini / 13.19s MacBook): requested synthetic edit, no early save,
Undo/Redo, explicit save/reopen and rendered output. After the protected installations,
no-model checks on the normal paths passed with helper 0.1.7 on both hosts: panel/status,
control responses, light/dark composer outlines and update guidance. Screenshots were
inspected; the bottom composer border is complete without clipping. The helper update was
not qualified by repeating a paid model turn. Full source CI passed all nine gates;
this historical acceptance does not imply a new model turn on the toolbar-only native above.
The current toolbar source and installed checks are recorded in the [title-bar ledger](#title-bar-assistant-placement--2026-11-05).

Evidence: `/private/tmp/wordcraft-standard-ci.log`,
`/private/tmp/craft-single-app-evidence/word-{mini,book}/` (actual-model candidates),
and `/private/tmp/craft-single-app-evidence/word-{mini,book}-installed/` (installed reports/rendered panels).
Owned processes exited normally; listeners and task-owned control keys were cleaned up.
Standard and old Preview profiles, recovery data, shared AI settings and conversations were
preserved, without profile merging or credential copying. Active Preview bundles and obsolete
MCP shortcuts were retired, and Preview installers archived under the per-host
`~/Library/Application Support/Craft/retired-apps/2026-11-05-single-app.noindex/` record.
No owner process was force-quit. Physical IME, clipboard and VoiceOver checks were not repeated.

## Historical integration and Preview delivery (superseded)

Everything below records earlier checkpoints, not the active installation policy.

Historical checkpoint: WordCraft Assistant Preview0.15.2/source `fccdfe38` was deployed on2026-11-04. See [the current update ledger](#official-v0152-reconciliation--2026-11-04). Earlier checkpoints are historical.

The isolated task branch combines assistant commit `05d5fb21` with upstream `origin/main`
`ab0020d6` (WordCraft 0.15.0). This retains the embedded, independently installed assistant
and its complete live MCP client interface while taking upstream document, proofing,
interface and file-format fixes. The original assistant checkout remains separate.

## Historical integration task ledger

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


## Title-bar Assistant placement — 2026-11-05

| Work / owner | State | Acceptance / evidence |
|---|---|---|
| Word title bar / Word owner | Deployed | One icon-only Assistant beside Discord, across Home/View/Insert; existing `ui.assistant`, selected state, accessible name and tooltip retained. Home/View duplicates removed. |
| Source verification / Word owner | Verified | Placement/pointer/Space regression across 600/800/1440 points, Home/View/Insert and both palettes; collapsed ribbon/hidden Discord. All nine `cargo xtask ci` gates passed; four 600/1440-point light/dark renders inspected. |
| Standard package / Word owner | Verified | Source `847526d8`, optimized required-input build passed in 13m05s; standard signed ZIP extraction, DMG mount/copy, exact certificate/metadata and independent both-host candidate checks passed. |
| Two-Mac delivery / primary + Word owner | Deployed | Protected launcher installed normal app on both Macs; independently repeated actual icon open/close, helper 0.1.7 light/dark renders and data/cleanup checks on each installed path. No inference requested. |

This supersedes the historical View-only placement above. No provider/model/credential,
conversation or document behavior changes are requested. Verification and deployment evidence
will be recorded independently; no public release is implied.

Source verification used pinned fonts `b076417b5880bd4154bd22f00695b1d28c6bea83` and
craft-i18n `9d6d54d6d9ff603cc41e12e5f6ba51af3554ab66`. Complete CI passed formatting,
workspace/all-target strict Clippy, 5202 ordinary native tests (9 ignored), translation audit,
language embedding, doctests, attribution, layering and all 15 WASM crates. Existing WASM
warnings remain. The placement regression failed on the old ribbon layout at 600 points and
passes after the move. Pointer open and keyboard Space close preserve synthetic document text.
The title-bar icon uses original code-drawn sparkle art, matching the suite's Assistant identity;
no new asset/dependency is added. Source screenshots are private QA outputs under
`/private/tmp/wordcraft-title-toolbar-shots/`; CI log `/tmp/wordcraft-title-toolbar-ci.log`.
The normal desktop minimum width remains 760 points; the 600-point offscreen stress case retains
the existing crowded title/search controls, while the Assistant itself stays visible.
Native installed checks, VoiceOver, IME, clipboard and inference have not been repeated here.


### Title-bar package and installed acceptance

Source `847526d8838553ad2c15e8d3ab7db9795d3e7f48` is committed/pushed to the existing
`fork/codex/independent-assistant`; no upstream-main push, public release or notarization.
The standard package retains `ai.storyteller.wordcraft`, the normal profile, disabled built-in
updater, protocol 1 and exact underlying official `v0.15.2`/`edfea681` metadata. The exact local
certificate SHA-1 is `D7B1B5524FFEB37AB01D3960017BDF6743E3EDFD`. Artifacts:
`dist/assistant-standard/847526d8/wordcraft-0.15.2-macos-aarch64.{zip,dmg}`.
ZIP SHA-256 `7c901009dfef1234030f4e141a4b7c3ed21701164126905a931ca590146f8f29`;
DMG SHA-256 `1fda3d98fdb816b7e53c535257e3210ac6eee14c921eef0cf526074b5ca9b70d`.
The exact installed signed native on both hosts is
`ebd7751d715f6f6fcba306ad900811fd94c03184ff33377a7b60b247d6597f85`.

| Host / component | State | Independent observed evidence |
|---|---|---|
| Mini / signed candidate | Verified | Actual top icon opened/closed Assistant in light/dark, helper 0.1.7 child rendered; reviewed PNGs; source/native verified. |
| MacBook / signed candidate | Verified | Same independent checks and exact native bytes; temporary candidate unregistered after normal exit. |
| Mini / `/Applications/WordCraft.app` | Deployed | Protected launcher replacement; actual icon open/close, helper light/dark PNG review, synthetic document/readback and saved bytes unchanged. |
| MacBook / `/Applications/WordCraft.app` | Deployed | Same independent installed-path checks; exact signed native source/hash matched Mini. |

All four isolated native runs used disposable profiles and empty connection stores, copied only
public helper/source configuration, requested no inference and read no real provider credentials.
The unavailable-model label in those disposable screenshots is expected from that deliberately
empty store. Normal Word/retired Preview profiles, recovery files and the entire shared AI store
had identical before/after file hashes. Every owned app exited normally, released its listener
and removed its control key; no baseline user process was stopped. Candidate paths were
unregistered after testing. Existing model credentials, routing, conversations and CLI/MCP
registrations remain unchanged. Physical VoiceOver, IME and clipboard were not repeated.

Private evidence (reports and reviewed PNGs):

- Mini candidate: `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/wordcraft-top-toolbar-v6epzvtc/`.
- Book candidate: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-top-toolbar-otn0ceyu/`.
- Mini installed: `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/wordcraft-top-toolbar-vksjnm0f/`.
- Book installed: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-top-toolbar-l4gyt71u/`.
- Book report/PNG copies: `/private/tmp/wordcraft-title-toolbar-evidence/{book-candidate,book-installed}/` on Mini.
- CI/build/package logs: `/tmp/wordcraft-title-toolbar-{ci,release,package}.log`.
- Primary launcher receipts: `/private/tmp/word-toolbar-install-{mini,book}.log` on Mini.

Primary retained exact verified prior-app ZIPs per host under
`~/Library/Application Support/Craft/retired-apps/2026-11-05-top-toolbar.noindex/`
as `wordcraft-HOST-before-toolbar.zip`. The root suite ledger owns final inventory and
post-documentation cache-fingerprint refresh. No app/provider contract changes or remaining
Word installation work are implied by that documentation-only finalization.

## WordCraft 0.17 reconciliation — 2026-11-09

| Workstream | State | Acceptance / evidence |
| --- | --- | --- |
| 0.17 merge candidate | In progress | Isolated candidate only; 0.15.2 is not an ancestor of 0.17.0, so the provisional prior-release merge base and auto-merged changes require review. |
| Assistant document/window safety | In progress | Implicit operations fail if the active document or window changes; queued visual input fails closed pending window-bound delivery. Assistant suite passed locked/offline against exact b347 with the clean configured font checkout and official pinned i18n checkout (36 passed, 2 ignored), including delayed multi-window rejection. The switched-document regression failed with its guard disabled because the command succeeded; the passing guard source was restored. Existing screenshot document-switch cancellation, active-window capture and title-bar toggle tests passed against b347. Bound screenshots remain available; queued visual input remains disabled pending a window-bound queue. |
| Shared dependencies | In progress | Exact local b347 shared revision is pinned and Cargo metadata resolved locked/offline; publication is not assumed. Configured font checkout is clean; a separate official i18n checkout matches the pinned revision. |
| Build and delivery | In progress | Locked/offline GUI and CLI checks, scoped strict Clippy for UI/GUI/CLI, formatting, assets and layers passed against b347 and the pinned font and i18n inputs. Full source gates, actual-model document/Undo/Redo/save/reopen checks, signed packaging and protected launcher installation on both Macs remain required. Existing installations are unchanged. |

The first full offline `cargo xtask ci` run with b347 and the pinned font/i18n inputs passed
workspace formatting and Clippy, but its test stage failed when four desktop argument tests all
timed out after 30 seconds. The remaining CI stages were skipped. Running the same compiled
`--help` test alone then passed in 4.47 seconds; running all four argument tests together passed
in 0.67 seconds. The cause of the original timeouts is not established. The full gate therefore
has not passed. The original run is recorded in `/private/tmp/wordcraft-b347-full-ci.log`.
An offline rerun with `RUST_TEST_THREADS=1` again passed formatting and workspace Clippy,
but the invalid-port integration test timed out; the other three argument tests passed and
subsequent gates were skipped. That invalid-port test passed independently in 0.76 seconds
with the same external temporary directory. The serial run is recorded in
`/private/tmp/wordcraft-b347-full-ci-serial.log`; the timeout's cause remains unconfirmed.
Twenty direct runs of the invalid-port test passed; none was slow enough to capture a stack.
The separate Cargo-driven reproduction was paused at the user's request during compilation,
before its first test started. Its task-owned process tree (PIDs 4382, 4394, 13118, 13884,
and 13886) was stopped; sources and the external target under
`/Volumes/X10 Pro/codex-craft-build-20261109/wordcraft-full-ci` were retained. Its partial
log is `/private/tmp/wordcraft-argument-reproduction.log`. Resume only when requested, and
repeat full CI after concurrent builds have finished to assess the load hypothesis.
The shared dependency branch `codex/assistant-upstream-20261109` was independently checked
on the remote and points at the exact pinned b347 revision. A resumed full CI with one build
job was stopped during workspace Clippy to coordinate CPU use; formatting had passed and its
partial log is `/private/tmp/wordcraft-b347-full-ci-resumed.log`. The external target remains
available for a subsequent run.
The subsequent low-load full CI run passed all nine stages (formatting, workspace Clippy and
tests, translation audit, language check, doctests, assets, layers and wasm) with one build job
and one test thread, exact b347 and the pinned release inputs. The wasm check emitted two
warnings but passed. All four formerly timing-out argument tests passed. A sample of one slow
test child showed it still in macOS dynamic-loader startup before Word's argument handling;
this observation does not establish the cause of the earlier failures. Log:
`/private/tmp/wordcraft-b347-full-ci-lowload.log`.

### Matching CLI and existing MCP registrations — 2026-11-09

| Workstream | State | Acceptance and evidence |
| --- | --- | --- |
| WordCraft 0.17 CLI | Not started | Build `wordcraft-cli` from the exact reviewed integrated commit and b347 lock; sign and independently verify the artifact, version, architecture and digest before either host receives it. The integrated GUI packager currently includes only the GUI executable and does not update an external CLI. |
| Mini CLI and MCP | Not started | Existing PATH symlink selects 0.15.2; the enabled Codex MCP entry directly names a 0.15.0 CLI with connect-mode arguments. After independent artifact validation, preserve backups and update the versioned CLI, PATH symlink and only the MCP command path; keep the current connect arguments and every other setting. Verify actual MCP startup and connect behavior against a synthetic document. |
| MacBook CLI and MCP | Blocked | PATH selects 0.15.2 and the configured MCP wrapper selects the installed GUI version. Its 0.15.2 executable exists and its signature verifies, but the wrapper currently refuses it because that version's `approved-sha256.txt` is absent. Preserve the wrapper and connect arguments; once the exact 0.17 CLI is independently signed and verified, stage it without overwriting existing files, create its approval record only for that verified executable, and run `--diagnose-version 0.17.0` before GUI cutover. After cutover verify `--diagnose` and an actual synthetic MCP connection. Restart affected MCP client sessions as needed. |
| Data and delivery | Not started | Preserve both hosts' app settings, profiles, credentials, conversations, existing registrations, recovery copies and unrelated files. Keep verified rollback copies of any symlink or configuration actually changed. CLI preparation does not authorize installing the unqualified GUI. |

The wrapper has no approval-creation command; its supported check consumes an explicit, lowercase SHA-256 record next to the versioned executable. Do not manufacture an approval for an unverified artifact or replace the existing wrapper with an unguarded route. The current 0.15.2 missing approval and the planned 0.17 delivery are separate facts; neither has been modified.
