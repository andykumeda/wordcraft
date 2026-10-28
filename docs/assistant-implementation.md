# Portable assistant implementation ledger

Updated 2026-10-28 (America/Los_Angeles). **Implementation: Verified for the local preview; broader release gates remain open.**
The original shared and WordCraft automated gates passed, and a real local model completed MCP
read/edit/read-back/Undo acceptance. The Codex live follow-up found and repaired a continuation
defect and a long-chat-title layout defect. Corrected native Codex and mixed Codex/Ollama
edit/read-back/Undo acceptance now pass; current installed source is `1692eef9`.
Hosted-provider and other-platform live acceptance remain separate release gates.
The subsequent Qwen Instruct trial failed the original prompt without editing; a guided diagnostic
completed with Codex correction and exact Undo. The document-scope repair checkpoint under
Workstreams now tracks the requested fix and portable download; native revalidation is pending.
The execution host is **AK's Mini**. **Local preview installation: Deployed**, following the
user's request to place a testable build in `~/Applications`. `/Applications/WordCraft.app` is
unchanged. No public app release or WordCraft push has occurred. After compaction, read this ledger
and inspect Git state; do not restart completed implementation or checks.

## Accepted scope

- A Rust native assistant pane with one conversation and independently selected **General chat**
  and **App control** models. Chat delegates document work to control through WordCraft's existing
  MCP server; unrelated chat does not automatically read the document. Relevant document content
  may reach both selected providers.
- Codex account sign-in through an isolated Craft-managed home, plus OpenAI-compatible hosted/local
  connections. Discover/probe tool capabilities, permit explicit manual model IDs, leave bindings
  unset until configured, and never substitute a provider/model on failure.
- Apply requested edits directly through UI-thread commands, protection checks and normal Undo.
  Bind a turn to its originating document/window/selection; refresh stale document context and
  reject changed selections or unrelated targets. Stop cancels inference and queued edits;
  completed edits remain undoable and uncertain operations are never replayed automatically.
- Permit the current document and files explicitly named/chosen by the user. Enforce exact grants
  at the I/O boundary, including indirect command paths and symlink changes.
- Save per-app chats outside document files with New/Open/Delete and interrupted recovery. Journal
  tool outcomes before dispatch and after acknowledgment. Store credentials in the OS credential
  store; keep them out of settings, transcripts, logs and control responses.
- Reuse `craft-ai-core` connections/models/bindings; put app-independent orchestration, providers,
  persistence and reusable widgets in `craft-libs`. Share suite connections/defaults with per-app
  overrides. Preserve WASM through native-only execution services and existing theme/i18n patterns.
- The user authorized a separate local test build in `~/Applications`: the installed bundle is
  `WordCraft Assistant Preview.app`. Preserve the existing `/Applications/WordCraft.app` and MCP
  wrappers. This authorizes local installation, not a public release.
- Follow-up authorization: after the corrected Codex edit/Undo path works, try a local Ollama model
  for **App control**, keeping Codex for **General chat** to exercise independent mixed-provider
  routing. Discover and verify an already installed tool-capable model; do not silently substitute
  models/providers or download a model without establishing that it is needed.
- **Deferred:** GridCraft/PdfCraft/other app integrations, browser inference and public app release.
  Do not copy existing Codex authentication, chats, settings, skills or MCP registrations.

## Repositories and delivery

| Repository | Checkout / branch | Revision and delivery |
| --- | --- | --- |
| WordCraft | `/Users/andy/Dev/wordcraft`, `codex/portable-assistant` | Base `c4fafc215490a0186ee750454b1ce73a9823daed`; original implementation `38ca4478`; corrected installed binary source **`1692eef9cc87ced4d2a26e90f0b203e357abacce`**. Not pushed; separate local preview installed. |
| craft-libs | `/Users/andy/Dev/craft-libs`, `codex/portable-assistant` | Base `92d392cff132eaf53e13afff77f38b5c11b73aa0`; initial implementation `566be16052de3cbe04fa132a88c24db0b7c7f756`; home isolation `a8d4c0a178aff4c31d27e05bf87a4e93915bc831`; reviewed continuation/layout fix and current consumer pin **`d4641d3663819aeb20cab5ccc1623292c5f4e30e`** |

The user explicitly approved publishing to **andykumeda/craft-libs**. The shared implementation/fix commits and documentation-only acceptance commit
`6820cf7e9093c78d0f5ee4a52ee86d8e617075a2` were published to that fork's
`codex/portable-assistant` branch, and the remote SHA was verified. All
WordCraft craft-libs dependencies and Cargo.lock use the final revision from
`https://github.com/andykumeda/craft-libs`; no sibling-path overrides remain. Original storytold
origins were preserved. Existing non-Craft lockfile package versions remain present.

`plan/` and the local craftrules checkout are absent. `storytold/craftrules` returned 404 through
public and connected reads; no unavailable instructions were invented. Installed Rust/Cargo
1.97.1 and the WASM target were used via `/Users/andy/.cargo/bin`.

## Original local preview installation checkpoint (historical)

**State: Deployed — local test build, native smoke verified.** The installed native release was
built from `38ca44788a5b190db630059bbbe967c5b6a91f26`, with shared revision `a8d4c0a`.
This is the original installed preview; the corrected `d4641d3` build is in progress below.
This follow-up adds no application source changes and is not a public release.

| Item | Installed configuration / verification |
| --- | --- |
| Destination | `/Users/andy/Applications/WordCraft Assistant Preview.app`; original `/Applications/WordCraft.app` executable hash unchanged; MCP wrappers preserved |
| Build | Apple Silicon ARM64 release, `cargo build --release --locked -p wordcraft` passed in 6m53s; installed `--version` reports `wordcraft 0.11.0 (38ca4478)` |
| Bundle identity | `ai.storyteller.wordcraft.assistant-preview`; distinct preview display name |
| Signing | Local ad-hoc signing with existing empty entitlements; installed bundle passed `codesign --verify --strict --deep --verbose=2`; no notarization |
| Document profile | Generated bundle environment `WORDCRAFT_PROFILE_DIR=/Users/andy/Library/Application Support/WordCraft Assistant Preview`; persistent preferences/recovery enabled for user testing |
| Assistant settings | Normal launches use the default shared `~/Library/Application Support/Craft/ai`; no credentials, conversations or model bindings copied |
| Codex executable | Generated bundle environment `CRAFT_CODEX_BINARY=/opt/homebrew/bin/codex`; installed version 0.165.0; sign-in/model gates remain open |
| Optional build assets | `CRAFT_I18N_DIR` and `CRAFT_FONTS_DIR` omitted: English UI and existing bundled/system-font defaults; this is not the multilingual distribution build |
| Native smoke | Two LaunchServices launches of the installed bundle using synthetic sample text and disposable assistant settings; Assistant status open/available/idle; screenshot visually inspected; both normal quits succeeded |
| Persistent profile | Second launch used the bundle-defined profile without an override; control key and saved `ui.json` appeared in the preview profile, proving Finder launch environment and preferences persistence |
| Listener cleanup | Temporary control port 17981 had verified app ownership; both smoke instances stopped and released the listener; no permanent listener or startup entry |
| Receipt and screenshot | `/private/tmp/wordcraft-preview-install.json`; `/private/tmp/wordcraft-preview-installed-clean.png` |
| Binary SHA-256 | `a0a824b4c92f9ed308414ab8b27021df5f3c4f21235cc1b6922da29c732ccc6b` |

**Updater caveat:** choose **Later** on the standard WordCraft release update prompt. Updating
this custom preview bundle has not been verified. The smoke selected Later; no update was installed.
No private documents or live inference were used during package verification. The app is installed
and quit, ready for the user to open. The pre-commit `cargo xtask ci` rerun passed all eight stages,
including **4,625 ordinary tests passed / 6 ignored**, the localization audit and 16-crate WASM check.
Output is at `/private/tmp/wordcraft-preview-ci.log`; the release build log is
`/private/tmp/wordcraft-preview-build.log`. This delivery record is committed separately from the
installed source revision; obtain its commit with `git log -1 --format=%H -- docs/assistant-implementation.md`.

## Codex live acceptance follow-up

**State: Verified after repair; original failure and repair history below.** The user requested sign-in and an actual assistant edit test. They completed
the device-code flow in the browser; the preview saved the isolated Craft Codex account after its
initial containment checks and model discovery. Both roles were explicitly set to the discovered
`gpt-6.1-sol` model as **WordCraft-only overrides**; suite defaults remain unset. Credentials and
one-time codes are not copied into this ledger or test artifacts.

A separate installed-preview process (PID 91872, temporary owned control port 17981) opened
`/private/tmp/wordcraft-codex-acceptance-rdx0kihp/Assistant Test.txt` with its own disposable document
profile and the normal shared assistant configuration. The user's original preview process was
preserved. The request was to read the synthetic document, change only 9:00 AM to 10:00 AM, then
read back. Sign-in and model capability checks succeeded, but the control role made **11 successful
get_text calls and no edits**, then stopped at the configured round limit. Independent document
read confirmed the original text was unchanged. No command was replayed after the failure.
Synthetic-only failure evidence is `attempt-1.json` in that test directory; the UI screenshot is
`assistant-running.png`. The test process received normal Quit after evidence capture.

Root cause: Codex's adapter closed its ephemeral worker at each tool call and supplied results to
the next worker as quoted JSON history, rather than replying to the actual dynamic-tool callback.
The original compatibility probe masked this by removing tools and adding a new user instruction
for the continuation. Services work now retains a bounded ephemeral exchange, returns the actual
identified tool result, and strengthens continuation/cancellation/mismatch tests. Craft remains the
canonical history owner. Separately, a long saved-chat title expands the History combo and its
trailing label wraps vertically; the widgets workstream is fixing the compact header layout.

Both shared fixes are now committed as `d4641d3663819aeb20cab5ccc1623292c5f4e30e`, published to the
already authorized fork, and verified with `git ls-remote`. Shared fmt, full all-target Clippy,
unfiltered tests (**1,841 passed / 29 ignored**) and core/UI WASM all passed; logs are
`/private/tmp/craft-assistant-continuation-shared-{fmt,clippy,tests,wasm}.log`. Native focused tests
passed **45 / 2 ignored**; long-title geometry passed all 12 width/height/idle/running cases and four
screenshots were inspected. The portable provider lifetime contract is documented explicitly.

WordCraft's manifests and lockfile now pin that revision; no other package versions changed.
Final post-pin `cargo xtask ci` passed all eight stages: fmt, Clippy, **4,625 ordinary tests
passed / 6 ignored / zero failures**, the separate localization audit (one test), doctests, assets,
layers and the 16-crate WASM check. Log: `/private/tmp/wordcraft-codex-fixed-ci.log`; retained
target: `/private/tmp/wordcraft-codex-acceptance-ci`. The ARM64 release cache build also passed
in 14m20s at `/private/tmp/wordcraft-codex-preview-build`, log
`/private/tmp/wordcraft-codex-preview-build.log`. Its temporary `uncommitted` identity must be
replaced by the verified WordCraft commit before installation. The old preview quit normally
through its app menu with no save prompt before installation.

Completed: the verified consumer integration was committed and installed, and the corrected
Codex and mixed Ollama tests passed as recorded below. The earlier pre-fix CI attempt at
`/private/tmp/wordcraft-codex-acceptance-ci.log` stopped at sandbox-denied loopback binds
(40 tests passed, 11 control-server tests failed with `PermissionDenied`); later stages did not
run. That environment failure was resolved by the full successful reruns with listener permission.
The ignored live helper now creates/drops a provider for each standalone capability probe before
creating the real turn provider. Production already used per-turn provider ownership.

Routing verification requested by the user must distinguish configured labels from actual work:
record effective per-app bindings, routed provider/model status, the delegation/MCP journal, and
Ollama request-log plus active-model evidence during the synthetic edit, followed by read-back
and normal Undo. Ollama is installed at `/opt/homebrew/bin/ollama`; `gemma4:e4b` is already present,
and port 11434 was free on inspection. It was subsequently started after the Codex retest, with actual ownership verified;
the completed mixed inference is recorded below. No model download occurred.

## Corrected installed acceptance checkpoint

**State: Deployed / Verified for Codex and mixed Codex/Ollama editing.**
The corrected preview is source `1692eef9cc87ced4d2a26e90f0b203e357abacce`, shared
`d4641d3663819aeb20cab5ccc1623292c5f4e30e`, ARM64 version `0.11.0 (1692eef9)`.
The commit-tagged release build passed in 5m28s. The installed ad-hoc signature passed strict
verification, and the original `/Applications/WordCraft.app` hash is unchanged. Installed binary
SHA-256: `b880329f585f336ead519d921ccd4365bf339bd75406e52cbd8f49cd95383d78`.
Receipt: `/private/tmp/wordcraft-preview-corrected-install.json`.

Corrected native Codex acceptance passed on the Mini in about 30 seconds: chat and control both
used `gpt-6.1-sol`, with one `control_app` delegation and six successful MCP calls (`get_text`,
`inspect_document`, `select_text`, `type_text`, `get_text`, `inspect_document`). The saved journal
contains exact before/after reads; independent document read confirmed that only 9:00 AM became
10:00 AM. One normal `edit.undo` restored the original complete text. The long-title pane,
composer and Stop control were visually inspected in the installed app. No uncertain turn was
replayed. New synthetic evidence is in `/private/tmp/wordcraft-routing-_u55a7zx/`:
`codex-status.json`, `codex-conversation.json`, `codex-verification.json`, `codex-complete.png`.
The test listener is owned by preview PID 67132 on loopback 17981 with a disposable document
profile; the shared Craft sign-in and WordCraft-only model choices are retained.

Ollama 0.40.11 is now running as the task-owned manual process PID 70544 on 127.0.0.1:11434,
with cloud and pruning disabled. Its actual version/model metadata confirm existing
`gemma4:e4b` supports tools. The preview saved an Ollama connection without a key, passed
the synthetic tool-capability probe and changed only App control to `gemma4:e4b`. No model was downloaded.
The configured route alone is not execution proof: saved chats have no permanent provider/model
stamp. Capture live route status, the effective binding snapshot, actual Ollama requests/model
residency, completed delegation/MCP results, exact document output and normal Undo together.

Mixed native acceptance passed in **256.04 seconds** (observed workload timing, not a controlled
hardware benchmark). The live status records **Codex/gpt-6.1-sol → Ollama/gemma4:e4b → Codex/gpt-6.1-sol**.
One `control_app` delegation contains four successful MCP calls: `get_text`, `select_text`,
`type_text`, `get_text`. Five post-boundary `POST /v1/chat/completions` requests returned HTTP 200
on the owned local Ollama listener; `/api/ps` captured `gemma4:e4b` and its digest. The journal
contains exact before/after document reads, independent read-back matched, and one normal Undo
restored the full original text. Evidence in the same directory: `mixed-status.json`,
`mixed-conversation.json`, `mixed-verification.json`, `mixed-ollama-active.json`,
`mixed-ollama-requests.log`, and visually inspected `mixed-complete.png`.

The disposable preview saved only its restored synthetic fixture and quit normally; loopback 17981
was verified released. The installed preview was then reopened normally, without a control listener,
with the synthetic baseline, Assistant open and a blank conversation. Saved Codex-chat/Ollama-control
choices survived this ordinary launch. Ollama remains running for the user trial; it is manual,
loopback-only, and has no login/reboot autostart. Its lifecycle is recorded in LOCAL_SERVICES.md.
The pre-delivery WordCraft CI rerun passed all eight stages again: **4,625 ordinary tests passed /
6 ignored**, localization audit passed and WASM passed across 16 crates; log
`/private/tmp/wordcraft-routing-delivery-ci.log`.

The user asked about a faster model. Hardware was verified as **Apple M1, 16GiB RAM, eight CPU cores**.
The Gemma run used Metal with 43/43 layers offloaded. Recommended next candidate is
`qwen3:4b-instruct-2507-q4_K_M` (official Ollama 2.5 GB listing and Qwen non-thinking model card),
which had not yet been downloaded or benchmarked at that checkpoint. The following section records
the subsequently authorized download and comparison. No automatic provider/model substitution was made.

## Qwen comparison follow-up — October 28, 2026

**State: Verified comparison; Qwen requires correction on this task.** The user requested the same edit/read-back/Undo test with Codex retained
for General chat. Inspection found `qwen3:4b`, Q4_K_M, 4.0B parameters, with tool support and
thinking enabled. The user then explicitly requested downloading the recommended exact
non-thinking variant: `qwen3:4b-instruct-2507-q4_K_M` from
[the official Ollama listing](https://ollama.com/library/qwen3:4b-instruct-2507-q4_K_M).
No timed inference was run with the first tag, and no automatic fallback was made.

The existing installed preview remains source `1692eef9`; no application code or binary change
was needed. A new disposable fixture/profile under `/private/tmp/wordcraft-routing-e29k8dlb`
used the identical text and prompt. Test PID 1724 owned loopback 17981, verified before private-key
use. The idle duplicate ordinary preview was quit normally with its unchanged saved fixture.
The configured IPv4 Ollama endpoint is still the verified process 70544. A separate user-owned
IPv6 Ollama process 95060 was observed and left unchanged. Download log: `qwen-download.log`
in the new test directory. The user-authorized exact Instruct model is now saved for App control; General chat remains
Codex/gpt-6.1-sol. Existing downloaded models were preserved.

The exact Instruct download succeeded and its manifest digest is
`0edcdef34593eac1aa2be9c7d06c432dcf81945adca5eca2f27662c18f168ba0` (2,497,293,803 bytes).
WordCraft discovered it, passed the synthetic tool-capability check and saved it for App control;
General chat remained Codex/gpt-6.1-sol. Model residency was captured before timing. Only the
positively identified CI process tree was paused for the timed run and resumed afterward.

**Original identical-prompt result: failed correctness after 46.44 seconds.** The Qwen control
model called `get_text` with `doc:0`; the bound-document guard rejected that target. The assistant
reported the failure and stopped. Independent document text was unchanged, zero mutations
occurred, and Undo was not run because there was no edit. Two local inference requests returned
HTTP 200, which establishes transport success rather than edit success. Evidence: `qwen-status.json`,
`qwen-conversation.json`, `qwen-verification.json`, `qwen-ollama-requests.log`, and
`qwen-original-failed.png`. Do not present 46.44s as an editing speedup.

The separate diagnostic completed in **66.78 seconds** with the added instruction to omit optional
`doc` parameters and pass that instruction through delegation. It first read and selected correctly,
then again supplied `doc:0` to `type_text`, which the host refused before any mutation. Qwen's
control summary incorrectly claimed success despite that rejected write. Codex inspected the
canonical tool results, re-delegated with corrective guidance, and the second control sequence
read, selected, edited and read back successfully. Final document text was independently checked,
and one normal Undo restored the exact full baseline. Seven MCP calls were recorded: six successful
and one rejected; two delegations and nine HTTP 200 inference requests. The original failed run is
not replaced by this diagnostic. No host guard, application code or runtime limit changed; this was
recovery from an acknowledged refusal, not replay after an uncertain edit.

Diagnostic evidence: `qwen-diagnostic-status.json`, `qwen-diagnostic-conversation.json`,
`qwen-diagnostic-verification.json`, `qwen-diagnostic-requests.log` and visually inspected
`qwen-diagnostic-complete.png` under the same test directory. The receipt explicitly records the
extra guidance, recovered error, false intermediate control summary and successful final Undo.
CI was running during this diagnostic, so its time is not a controlled comparison with the original
isolated run. The guided Qwen observation was shorter than the earlier Gemma observation, but Qwen
needed correction on this task; this does not establish an overall speed or reliability result.

The test instance saved only its restored synthetic baseline and quit normally. Port 17981 was
verified released. The ordinary installed preview was reopened with the Qwen control choice and
Codex chat preserved, no control listener, and a fresh conversation for further user testing.
The existing IPv4 and IPv6 Ollama services were not restarted or reconfigured. No public release,
WordCraft push, dependency update, binary replacement or shared-library edit was needed.

Compare approximate observed task time with the previous 30.27s Codex and 256.04s Gemma runs,
including active route transitions, actual local requests/model identity, ordered MCP reads/edit,
independent exact document read and one normal Undo. Two-second polling gives approximate wall
time; one completed test establishes a smoke result, not a statistical reliability rate. A concise
comparison is saved as `qwen-comparison.md` in the Qwen evidence directory.

Required pre-commit `cargo xtask ci` completed with exit 0: all eight stages passed (fmt, strict
Clippy, tests, localization audit, doctest, assets, layers and WASM). Ordinary tests: **4,625 passed /
6 ignored / 0 failed**; the separate localization audit passed; doctest had zero tests; WASM checked
16 crates. Log: `/private/tmp/wordcraft-qwen-ci.log`. Only this ledger and the assistant help changed;
the installed preview remains `1692eef9`, and the shared dependency remains pinned to `d4641d3`.
After the CI process tree exited, its owned `/private/tmp/wordcraft-qwen-ci` build cache was removed
(8.5 GiB). Verification logs, synthetic evidence, downloaded models and the installed preview remain.
This documentation increment is on `codex/portable-assistant`; identify its commit with
`git log -1 -- docs/assistant-implementation.md`. No WordCraft push or public release was made.

## Workstreams

### Document scope repair — in progress

Additional user request: after repair and native acceptance, make the preview downloadable for other
Macs. Prepare a versioned archive, checksum and install notes. The user confirmed **Apple silicon
only**; private ZIP versus GitHub release destination remains pending (default private ZIP).
Do not claim notarization or Intel support
without verifying them. Packaging/distribution is pending and part of this follow-up's scope.

The user requested repairing Qwen's guessed document ID and rerunning the original prompt with
Codex chat unchanged. The assistant currently exposes the external MCP `doc` parameter and refers
to `list_documents`, although its restricted catalog has no such tool. The repair will specialize
the assistant's tool contract for its bound document while preserving the external MCP catalog and
the UI-thread document/window/selection guards. Regression checks, full CI, a refreshed preview,
and a fresh original-prompt edit/read-back/Undo run are pending. Prior Qwen failures remain recorded.

Current source changes: `assistant/host.rs` specializes the copied catalog; `assistant/host/tests.rs`
adds catalog isolation and embedded-MCP foreign-ID regressions. Dispatch is unchanged. The services
agent owns verification using `/private/tmp/wordcraft-document-scope-ci` (two build jobs); focused
log `/private/tmp/wordcraft-document-scope-focused.log`: **24 passed, 0 failed, 1 manual capture
ignored**, including both new regressions. Full CI completed with exit 0: all eight gates passed, **4,627 ordinary tests passed / 6 ignored /
0 failed**, the separate localization audit passed, and WASM checked 16 crates. Log:
`/private/tmp/wordcraft-document-scope-ci.log`. Keep that cache for
the release build. Root reviewed the code and owns documentation, commit, packaging and live testing.
The packaging agent added `packaging/macos/assistant-preview.sh` and its guide. Old-source
`1692eef9` smoke passed archive/checksum/extracted signature/version/architecture/profile override
checks and rejected mismatched provenance. This is packaging evidence only, not repaired-app
acceptance. No Developer ID Application certificate is available; the archive is ad-hoc signed and
not notarized. The package uses a portable launcher and no machine-specific plist environment.
Repair and packaging source were committed as `d075a068be401677b4e4d510d2ca2d585ca5df71`.
The release build passed in session 78960 (11m16s) with the same target, two jobs, deployment target
macOS 11.0, explicit build SHA, and optional font/translation inputs unset. Log:
`/private/tmp/wordcraft-document-scope-release.log`. Root prepared
`/private/tmp/install-wordcraft-portable-preview.py` to validate/extract a ZIP, preserve the original
app, back up and replace only the preview, verify signatures, and record the install receipt.
The ordinary preview currently has the saved synthetic baseline and Codex/Qwen bindings. Do not
restart completed sign-in or change models. After passing CI, commit the repair, build release with
that commit embedded, package it with the new script, and install the freshly extracted portable
bundle only after normal preview quit (retain a backup). Do not use the older installer that copies
the machine-specific plist. Create a disposable fixture/profile with
`/private/tmp/launch-wordcraft-document-scope-test.py` and run the original prompt without guidance,
so native acceptance covers the artifact recipients will receive.
Independently inspect tool results,
verify exact text and one Undo, restore/save the fixture, stop the test listener, and reopen the
ordinary preview. Update evidence, rerun incremental CI for the final documentation commit, and
remove owned build caches only after all verification/install work completes.

During the release build the Mini locked; CUA could not inspect the current preview, and a manual
unlock was requested asynchronously. Do not quit or replace that instance without checking for
unsaved work. Packaging can continue, and a separate extracted app may be tested with its own
disposable profile/control channel while leaving the existing instance untouched. The last observed
ordinary preview was saved with the synthetic 9:00 baseline and Codex/Qwen bindings.

The d075a068 ARM64 candidate ZIP is under `dist/assistant-preview/d075a068/`; checksum and extracted
signature/version checks passed. A separate extracted app at
`/private/tmp/wordcraft-scope-extracted-ww_nff85/.../WordCraft Assistant Preview.app` ran as PID 43817,
with verified loopback 17981 and fixture/profile `/private/tmp/wordcraft-document-scope-zwlb16b0`.
The original prompt ran unchanged with Codex chat and the exact Qwen Instruct model/digest.
**Document-ID errors were resolved, but the turn did not complete:** Qwen first chose an unknown
generic command, then misused `edit.replace` parameters. Codex redelegated; a correct `edit.replaceAll`
and `get_text` produced the exact requested text, but the global tool-round limit ended the turn at
70.56s. Independent exact text and one normal Undo passed. No extra user guidance was supplied.
Evidence: `repaired-{status,conversation,verification,ollama-active}.json` in that fixture directory.
The test saved only its restored baseline and quit normally; the original preview is untouched.
This ZIP is an intermediate candidate, not the final completed-task download.

The follow-up correction is generic tool-contract prose: direct text replacement to `select_text` followed
by `type_text`; reserve `execute`/`batch` for discovered command IDs/parameters. No model changes,
guard changes or round-limit increase. The services agent owns the minimal host description update
and strengthened the existing selection/replacement/Undo behavioral test. Focused tests passed
24/0 failures/1 manual capture ignored; full CI passed all eight gates, 4,627 ordinary tests with
6 ignored, separate localization audit, and 16-crate WASM. Logs:
`/private/tmp/wordcraft-replacement-contract-{focused,ci}.log`. Root must commit, rebuild/repackage that new source, and run a fresh
unchanged-prompt turn before final delivery. Preserve the d075 partial result rather than overwriting it.

The user clarified there is no lock-screen password; fresh CUA inspection succeeded. The ordinary
preview still showed its saved baseline and was quit normally through its app menu. Process exit
was independently verified. Installation is now unblocked; replace only the separate preview from
the newly verified archive, retaining its backup and the original `/Applications/WordCraft.app`.

| Workstream / owner | State | Acceptance / release status |
| --- | --- | --- |
| Shared contracts, runtime, HTTP, storage, widgets / shared agents | Verified | Full shared gates; two independent synthetic app hosts; published source |
| Codex home isolation / services agent | Verified | HOME/USERPROFILE/CODEX_HOME isolated; actual macOS read/write/network sandbox probe and selected-model continuation passed; broader adversarial acceptance remains gated |
| WordCraft MCP host, file grants, pane/settings/history / primary | Verified | Existing MCP and UI-thread command path; protection, Undo, cancellation, selection/window boundaries and persistence regressions |
| WordCraft final CI / primary | Verified | Post-fix eight-stage gate passed: 4,625 ordinary tests / 6 ignored; localization audit and 16-crate WASM passed |
| Local model acceptance / primary + services | Verified | Ollama 0.40.11, existing gemma4:e4b, both roles, actual MCP read/edit/read-back and one normal Undo passed |
| Qwen comparison / primary | Verified | Exact Instruct model downloaded and selected for control; original prompt failed without editing; separate guided recovery completed exact read/edit/read-back/Undo after Codex correction |
| Codex live continuation / services agent | Verified | Shared actual dynamic-tool result continuation repaired; full shared gates and 45 native focused tests passed; corrected installed Codex and mixed-provider live edit/read-back/Undo passed |
| Long-title assistant layout / widgets agent | Verified | Compact bounded history row; all 12 geometry cases and four inspected captures passed; corrected installed pane inspected with long titles, composer and Stop visible |
| Hosted live acceptance | Blocked | Requires a configured hosted connection; existing private credentials were not copied or read |
| Other-platform credential/native acceptance | Not started | Windows/Linux/BSD runtime and credential checks remain manual release gates |
| Documentation and local commits / primary | Verified | Consumer repair committed `1692eef9`; acceptance/help/progress docs updated after passing full delivery CI; shared evidence published as `6820cf7` |
| Local preview build/installation / primary | Deployed | Corrected `1692eef9` ARM64 preview installed, signature/original preservation and Codex/mixed native acceptance verified |
| Public release/other app integrations/browser inference | Not started | Deferred scope; existing installed app and MCP wrappers preserved |

## Implementation and review decisions

Shared contracts extend existing ai-core types without changing job behavior. `run_turn` uses
`ChatProvider` and `ToolHost`, with no WordCraft document dependency. Its synchronous
`TurnEvent::Checkpoint` journaling precedes dispatch and follows results; persistence failure
cancels queued work. Interrupted history closes unresolved calls explicitly and never replays
edits. Tool batches and saved message pairing are validated; schemas count toward context limits;
credential redaction covers streaming fragments, catalogs, tool arguments and error paths.

The Codex adapter uses an isolated home and ephemeral threads with bounded Craft history. A
synthetic external `.agents` skill proved that CODEX_HOME alone did not isolate discovery;
HOME and USERPROFILE now point at the Craft home too. Effective-policy, OS denial and selected-model
callback/continuation checks must pass before caller context. No persisted bypass exists.

WordCraft's `assistant/host.rs` bridges the existing MCP server to UI dispatch, checks document,
window, revision and full selection, enforces exact-file grants including indirect reads, and
refuses app/global settings and implicit dialogs. Authorized Open deliberately rebinds and
requires reading the new target. Missing acknowledgments are uncertain and never retried.
`assistant/native.rs` owns background jobs, provider dispatch, checkpoints, local history and
credential transactions. Failed key replacement preserves the old key; removal deletes only
unreferenced secrets. Ordinary headless WordApp instances never load the user's assistant profile.

Review fixes cover settings reload/Stop, saved connection identity, unsent drafts, stale route
labels, nested tool IDs, named-path tokenization, symlink retargeting, same-document other-window
edits, clipboard expansions and keyboard command search. The Thai font regression now tests real
tone-mark stacking instead of requiring OpenType tables on macOS's AAT-based Thonburi; no product
font behavior changed and no test was skipped to pass CI.

## Verification evidence

| Check | Actual result | Log / artifact |
| --- | --- | --- |
| Shared final source | fmt, all-target workspace Clippy, unfiltered tests **1,831 passed / 28 ignored**, core/UI WASM passed | `/private/tmp/craft-assistant-home-{fmt,shared-clippy,shared-tests,wasm}.log` |
| Shared native boundaries | Two synthetic app catalogs, routing/no-fallback, fragmented streams, malformed calls, duplicate prevention, limits/redaction, concurrent stores/recovery passed | Shared ledger and native test suites |
| WordCraft final CI | All 8 stages passed: fmt, Clippy, tests, i18n-report, doctest, assets, layers, WASM; **4,625 ordinary tests passed / 6 ignored**, no failures | `/private/tmp/wordcraft-assistant-ci-verified.log` |
| Focused assistant / file access | Initial assistant **31 passed / 1 ignored**, engine file access **11 passed**; later regressions included in final CI | `/private/tmp/wordcraft-assistant-final-tests.log`, `/private/tmp/wordcraft-assistant-files-tests.log` |
| Native pane screenshot | Passed and visually inspected; synthetic transcript/roles/tool activity/composer; no profile or Keychain access | `/private/tmp/wordcraft-assistant-screenshot.log`; `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/wordcraft-assistant-pane.png` |
| macOS Codex sandbox | Installed Codex 0.165 read/write/network denial and isolated home discovery passed with synthetic files; no account inference | `/private/tmp/craft-assistant-home-live.log`; shared native services doc |
| macOS Keychain | New synthetic item write/read/delete passed; no existing credential inspected | Shared `docs/assistant-native-services.md` |
| Local live model | **Passed in 359.61s**: both roles, original read, exact requested edit/read-back, persisted checkpoints and one normal Undo; 8 calls started/finished | `/private/tmp/wordcraft-assistant-live-ollama-final.log` |
| Preview delivery | ARM64 release build, installed version/hash/signature, native Assistant/profile smoke and original app preservation passed; full CI rerun passed all 8 stages | `/private/tmp/wordcraft-preview-{build,ci}.log`; install receipt and screenshot above |

Live acceptance used the explicit isolated `CRAFT_AI_CONFIG_DIR=/private/tmp/wordcraft-assistant-live-ollama`,
Ollama at `http://127.0.0.1:11434/v1`, and existing `gemma4:e4b` for both roles. No models were
downloaded, no credentials supplied, and application bindings remain unset. The model first
attempted edits before reading; the host refused them. It then read, delegated again, edited and
verified successfully. Completed turns are not proof that every future model response will comply.

The first local attempt failed with Provider after 219.96s (log `wordcraft-assistant-live-ollama.log`).
The second completed but exposed a test observer that recognized only top-level get_text, not
valid execute/batch reads (log `wordcraft-assistant-live-ollama-diagnostic.log`). The corrected
observer verifies exact successful MCP read results, rejects summary/mutation echoes, and has a
passing regression. The final fresh turn passed; no conversation or uncertain edit was replayed.

To repeat with explicitly prepared provider settings, run the ignored test
`assistant::native::tests::live_configured_models_edit_read_back_and_undo_through_mcp` with an
absolute CRAFT_AI_CONFIG_DIR containing connections.json and runtime.json. It probes each distinct
selected model, uses synthetic text and disposable history, and never signs in automatically.
For consistent workspace feature unification, use `cargo test --workspace --lib --bins --tests
assistant::native::tests::live_configured_models_edit_read_back_and_undo_through_mcp -- --ignored --exact --nocapture`.

## Remaining release gates and cleanup

Earlier cleanup entries below describe the original installation. Current corrected-test listeners
are stopped; the manual Ollama server remains running for the requested trial. Build-cache cleanup
is tracked in the delivery closeout below.

- Codex sign-in, selected-model callback/continuation and native WordCraft edit/Undo now pass.
  Complete the remaining adversarial containment acceptance before public release. The adapter remains fail-closed on any gate
  failure. The actual local sandbox probe is not account/model acceptance.
- Configure a hosted compatible endpoint and verify its live route. Mixed Codex/local-compatible
  routing already passed; hosted acceptance remains separate.
  No existing account chats, API keys or private document content were accessed for development.
- Verify Windows/Linux/BSD credential stores and native keyboard/accessibility/window behavior;
  complete native-speaker translation review. Missing translations use existing catalog fallback.
- User testing can proceed in the installed separate preview. Choose Later on standard-release
  update prompts. Public app release remains deferred; do not replace `/Applications/WordCraft.app`
  or change its MCP wrappers as part of this preview.
- Task-owned test/release/CI caches and the temporary package staging directory were removed after
  checks. The installed preview is retained for user testing. Useful logs, screenshots, installation
  receipts and credential-free synthetic live settings remain under Temp. The task-owned Ollama process exited successfully;
  a fresh socket check confirmed port 11434 released. Its stopped state is recorded in
  `~/.codex/LOCAL_SERVICES.md` (outside Git, so not committable here). Both preview smoke listeners
  on port 17981 also stopped; their launch/cleanup evidence is recorded in that inventory. No proxy
  or persistent startup entry was added.

## Delivery closeout before the Qwen follow-up

At that checkpoint, the installed ordinary preview was running as PID 79467 with Assistant open, the synthetic baseline
and the saved mixed provider choices; it has no control listener. Ollama PID 70544 remains on
127.0.0.1:11434 for the user trial. The restored fixture is saved. The original installed app hash
is still unchanged. The install receipt was updated with these checks.

All task-owned release/CI/shared-test caches and corrected package staging were removed after
verification. Installed preview, synthetic evidence, logs and receipts remain. No global startup
entry, proxy, WordCraft push or public app release was made. Both repositories have focused
commits; shared documentation publication is `6820cf7e9093c78d0f5ee4a52ee86d8e617075a2`, while
WordCraft continues to pin reviewed runtime `d4641d3`. The WordCraft documentation-only delivery
commit follows installed source `1692eef9`; use `git log -1 -- docs/assistant-implementation.md`
for its identity.

The later Qwen comparison is complete with the limitations recorded above. Remaining work is
limited to the release gates above and later integrations already deferred. Do not repeat completed
sign-in, build, Codex/Gemma tests or Qwen comparison after context compaction.

No global memory update was requested or performed. This is the durable repository ledger;
the shared repository ledger owns shared implementation details and verification.
