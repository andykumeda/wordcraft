# Portable assistant implementation ledger

Current source (2026-11-02): full side-panel and external MCP capability parity is verified by focused automated tests. The [full MCP parity change](mcp-assistant-parity.md) supersedes the assistant-only document, command and exact-file restrictions described in earlier historical entries below. This increment is not yet an installed release.

Updated 2026-10-29 (America/Los_Angeles). **Core single-model implementation: Verified and Deployed at `9c6c2cb0`; full MCP capability parity and reliable local-model acceptance remain open.**
The original shared and WordCraft automated gates passed, and a real local model completed MCP
read/edit/read-back/Undo acceptance. The Codex live follow-up found and repaired a continuation
defect and a long-chat-title layout defect. Corrected native Codex and mixed Codex/Ollama
edit/read-back/Undo acceptance now pass; that earlier 0.11 acceptance used source `ca279f8c`; current delivery is recorded below.
Hosted-provider and other-platform live acceptance remain separate release gates.
The subsequent Qwen Instruct trial failed the original prompt without editing; a guided diagnostic
completed with Codex correction and exact Undo. The document-scope repair checkpoint under
Workstreams records the completed fix, unchanged-prompt acceptance and portable download.
Latest follow-up: upstream 0.12.0 plus the October 29 settings, provider diagnostics, View discovery
and preview-updater safeguards are installed at `/Applications/WordCraft Assistant Preview.app`;
verified DMG/ZIP delivery is recorded at the end of this ledger. The earlier MacBook
launcher repair and provisional feedback remain recorded below.
The execution host is **AK's Mini**. **Local preview installation: Deployed**, following the
user's request to place the preview in `/Applications`. `/Applications/WordCraft.app` is
unchanged. No public app release or WordCraft push has occurred. After compaction, read this ledger
and inspect Git state; do not restart completed implementation or checks.

Current delivery: **`0.12.0 (9c6c2cb0)`**, one chat/model, WordCraft-only Terra selection; installed launcher and real Terra edit/save/reopen/Undo verified. See the final delivery checkpoint and [model comparison](assistant-benchmarks.md). Earlier revision and routing entries below are historical.

## Accepted scope

The latest 2026-10-29 owner decision supersedes the original two-role routing below: keep one
conversation and one model selector for Codex, Ollama or OpenRouter/API. The pending checkpoint
at the end records this candidate; the original scope and measured evidence remain historical.

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

### Document scope repair — verified and locally deployed

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

#### Final repair and download checkpoint

**Verified / Deployed locally:** installed source `ca279f8c4e24aabef56680846709a9bea63bd86e`,
shared runtime unchanged at `d4641d3`. Incremental release passed in 6m07s. Its portable ARM64 ZIP
was signed, checksum-verified, freshly extracted and installed in `~/Applications/WordCraft
Assistant Preview.app`. Original `/Applications/WordCraft.app` remains byte-identical. Install
receipt: `/private/tmp/wordcraft-document-scope-install.json`; previous preview backup is recorded
there. No model bindings or runtime limits changed.

The original prompt completed in **60.52s**, with Codex `gpt-6.1-sol` for General chat and
Ollama `qwen3:4b-instruct-2507-q4_K_M` for App control. There was no extra user guidance, no supplied
document ID and no tool error. Two automatic delegations covered editing and then read-back;
four successful MCP calls were `get_text`, `select_text`, `type_text`, `get_text`. All six local
inference requests returned HTTP 200; the actual loaded model digest matched the previously verified
`0edcdef34593eac1aa2be9c7d06c432dcf81945adca5eca2f27662c18f168ba0`.
Independent full text confirmed only 9:00 changed to 10:00, and **one normal Undo restored the exact
baseline**. The native screenshot was inspected. This is one successful smoke run, not a statistical
reliability benchmark; prior failed and partial runs remain recorded above.

Evidence directory: `/private/tmp/wordcraft-document-scope-sd0lsnne`:
`original-prompt.txt`, `final-status.json`, `final-conversation.json`, `final-verification.json`,
`final-ollama-active.json`, `final-ollama-requests.log`, `final-complete.png`.
Test PID 72615 saved only the restored baseline and quit normally; loopback 17981 was verified
released. The installed preview was reopened normally without a listener for user testing.

**Download:** Apple silicon only, macOS 11 declared minimum, ad-hoc signed and **not notarized**.
No Developer ID Application identity is available. Credentials, chats, settings, Codex and Ollama
models are not bundled. A portable launcher derives the receiving user's preview profile and
supports normal per-user Codex discovery. Each recipient configures providers/sign-in locally.
The user confirmed Apple silicon; public/GitHub publication was not selected, so no release or
WordCraft push was performed. Private sharing artifacts are retained in
`dist/assistant-preview/ca279f8c/`:

- `wordcraft-assistant-preview-0.11.0-ca279f8c-macos-aarch64.zip` (about 28 MiB)
- adjacent `.zip.sha256` and `-INSTALL.md`

ZIP SHA-256: `23d0742de69e10d8a40adfbc23a066e9578a72fe4a99f7303fd554c3748c7d66`.
Native binary SHA-256: `af3dc8d1164686862d0c230040c47005e0ce73c523f68d68ac39d0c46e9f6016`.
The reusable packaging command and signing limits are in `packaging/macos/assistant-preview.md`.
Other-Mac acceptance remains unperformed; this is a private preview, not a public production release.

Final documentation CI passed with exit 0: all eight stages, 4,627 ordinary tests passed / 6 ignored /
0 failed, separate localization audit and 16-crate WASM. Log:
`/private/tmp/wordcraft-document-scope-delivery-ci.log`. After verified exit, the owned build cache
`/private/tmp/wordcraft-document-scope-ci` was removed; logs, receipts, installed app and ZIP remain.
On the ordinary launch the Assistant pane was closed. At the user's request it was opened through
command search → Assistant; native accessibility confirmed the fresh composer, saved baseline and
Codex/Qwen routes. Home → Assistant also opens it. No new inference was started.
Do not rerun completed inference or rebuild the app after compaction.

| Workstream / owner | State | Acceptance / release status |
| --- | --- | --- |
| Shared contracts, runtime, HTTP, storage, widgets / shared agents | Verified | Full shared gates; two independent synthetic app hosts; published source |
| Codex home isolation / services agent | Verified | HOME/USERPROFILE/CODEX_HOME isolated; actual macOS read/write/network sandbox probe and selected-model continuation passed; broader adversarial acceptance remains gated |
| WordCraft MCP host, file grants, pane/settings/history / primary | Verified | Existing MCP and UI-thread command path; protection, Undo, cancellation, selection/window boundaries and persistence regressions |
| WordCraft final CI / primary | Verified | Repair eight-stage gates passed: 4,627 ordinary tests / 6 ignored; localization audit and 16-crate WASM passed; final documentation rerun tracked above |
| Local model acceptance / primary + services | Verified | Ollama 0.40.11, existing gemma4:e4b, both roles, actual MCP read/edit/read-back and one normal Undo passed |
| Qwen original-prompt repair / primary | Verified | Scoped tools and replacement contract; unchanged prompt completed in 60.52s with four successful MCP calls, no errors and exact read-back/Undo; earlier failures retained above |
| Codex live continuation / services agent | Verified | Shared actual dynamic-tool result continuation repaired; full shared gates and 45 native focused tests passed; corrected installed Codex and mixed-provider live edit/read-back/Undo passed |
| Long-title assistant layout / widgets agent | Verified | Compact bounded history row; all 12 geometry cases and four inspected captures passed; corrected installed pane inspected with long titles, composer and Stop visible |
| Hosted live acceptance | Blocked | Requires a configured hosted connection; existing private credentials were not copied or read |
| Other-platform credential/native acceptance | Not started | Windows/Linux/BSD runtime and credential checks remain manual release gates |
| Documentation and local commits / primary | Verified | Consumer repair committed `1692eef9`; acceptance/help/progress docs updated after passing full delivery CI; shared evidence published as `6820cf7` |
| Local preview build/installation / primary | Deployed | Portable `ca279f8c` ARM64 preview installed from the ZIP; signature/original preservation and Codex-chat/Qwen-control native acceptance verified |
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

## MacBook launcher PATH follow-up — verified packaging; user feedback provisional

The user reported Codex executable not found from Finder despite Terminal working. MacBook
`command -v codex` is `/Users/andy/.local/bin/codex`; the portable launcher only appended Homebrew
paths. Add the destination user's `~/.local/bin` after existing PATH/Homebrew entries, preserving
explicit CRAFT_CODEX_BINARY and prior executable precedence. No native code/model changes.
Repackage the existing verified ca279f8c native binary and provide a new DMG. The prior ZIP
MacBook Archive Utility Error94 remains destination-unverified; source ZIP passes integrity.
DMG alternative was verified mount/signature/version on Mini and copied to Mini Downloads.
Launcher/package smoke and mandatory pre-commit CI passed for this packaging repair.

The corrected archive is built from the unchanged verified native ca279f8c binary.
ZIP: `dist/assistant-preview/localbin-fix/wordcraft-assistant-preview-0.11.0-ca279f8c-macos-aarch64.zip`.
DMG: same directory, `wordcraft-assistant-preview-0.11.0-ca279f8c-macos-aarch64-localbin-fix.dmg`,
also copied with checksum to the Mini's Downloads. DMG SHA256:
`de4a14dbf04b6295e9809fe431a57d5034152381acdc5ce20abd5d29f1cdbb91`.
Smoke verified destination HOME containing spaces, discovery from its `.local/bin`, preserved
explicit CRAFT_CODEX_BINARY, native version, extracted/mounted signatures and DMG checksum.
Receipt: `/private/tmp/wordcraft-localbin-receipt.json`. The user reported “I think it is working”
after the Finder-launch test and moved on to Ollama configuration. This is provisional user feedback,
not an independently observed MacBook inference test. Supplied local Ollama configuration: endpoint
`http://127.0.0.1:11434/v1`, exact Qwen Instruct model for App control, Codex retained for General chat. No user settings, credentials, models or installed Mini preview changed.
Full precommit CI completed with exit 0: all eight gates passed; 4,627 ordinary tests passed,
0 failed, 6 ignored; separate localization audit and all 16 WASM crates passed. Log:
`/private/tmp/wordcraft-launcher-ci.log`. After verified exit, the owned cache
`/private/tmp/wordcraft-launcher-ci` was removed; logs, DMG and receipts remain. This increment
changes only the launcher and three help/ledger documents. No Rust rebuild, repeated inference,
Mini reinstall, public publication or provider/model substitution was performed. The native
binary remains source ca279f8c; this packaging revision is identified by the latest commit touching
`packaging/macos/assistant-preview.sh`. Do not repeat completed validation after compaction.

## 2026-10-29 — settings, provider errors and panel discovery

Current source improvements are verified by automated/synthetic checks; the installed preview remains unchanged.

| Workstream / owner | State | Acceptance and verification |
| --- | --- | --- |
| Shared settings / primary | Verified | Full-width model choices first; advanced IDs/scope and connection setup expandable; explicit save scope. Full UI tests 464 passed / 2 ignored; final focused assistant tests 9 passed / 2 ignored; UI Clippy, core/UI WASM and synthetic screenshot export passed, inspected visually. |
| HTTP diagnostics / provider agent | Verified | Selected connection/model and status-specific guidance; known Ollama context rejection reports validated counts. Six HTTP tests and all-target native Clippy pass; bounded bodies, cancellation and privacy covered. |
| View discovery / UI agent | Verified | Focused click/geometry regression passes: direct View button at 800/1440, Home toggle, unchanged document and custom-tab fitting. Four final screenshots inspected; valid existing speech-bubble icon and selected state. |
| Consumer integration / primary | Verified | Shared fork published and WordCraft pinned at be59354; only 11 shared Git source lines changed in Cargo.lock, no package versions changed. Final serial `cargo xtask ci` passed all eight gates: 4,628 ordinary tests / 6 ignored / zero failures plus the localization audit, doctests, assets, layers and WASM. Installed preview remains unchanged. |

The screenshot reports HTTP 400 from the OpenAI-compatible HTTP adapter. Its old error discards
provider bodies by design because they may echo private prompts or credentials. The exact rejected
field was initially unknown; the reproduction below identifies the context limit. Current local
settings use Codex for chat and Ollama/Qwen for document control. No private document content
was printed and no provider/model was changed; the later structural review of failing local
chats is recorded below. A network probe from the default sandbox did not reach the configured IPv4 endpoint;
that does not establish a live outage and does not explain an HTTP 400 response.

Provider workstream: focused native HTTP tests passed **4/4** (eight HTTP status cases,
secret redaction/body omission, existing streaming/timeout behavior); native all-target Clippy
and shared fmt passed. An authorized read-only/synthetic loopback check confirmed Ollama
**0.40.13**, current Qwen Instruct tool metadata, and a streaming/tool request **HTTP 200** with
one tool fragment and a completed response. No private content was sent and the failing turn
was not replayed. This verifies basic current compatibility, not the screenshot's rejection cause.
Log: `/private/tmp/craft-provider-error-clippy.log`.

The referenced `../../craftrules` checkout is absent. Both the public raw URLs and the connected
GitHub repository API return 404 for its instructions; the supplied WordCraft/craft-libs rules
and locally available docs are followed.

### Reproduced cause (Mini, new chat, remove dark background)

The user confirmed the Mini and a new chat. Structural metadata from the failed local chats
shows one successful `inspect_document` with an **11,552-byte** result, followed by HTTP 400
on the control-model continuation. No private document/result content was printed.
Replaying the same history shape against the app's synthetic sample reproduced Ollama's
context rejection: **4,432 input tokens exceed the configured 4,096-token context**.
The initial connection/tool request succeeds; this is not a credential failure. A new chat
can still fail because tool definitions and document inspection consume context too.

The HTTP adapter now recognizes only the exact known Ollama context-overflow form, extracts
validated numeric counts and shows owned guidance; arbitrary response text stays hidden.
Error-body reading is bounded by the existing configured response limit and cancellable.
Ollama's OpenAI-compatible API cannot change model context per request. Its documented
remedy is an explicitly configured model variant with `PARAMETER num_ctx` in a Modelfile,
or a server context setting. No model alias, binding change, download, service restart or
provider switch was made as part of this diagnosis.
Source: https://github.com/ollama/ollama/blob/main/docs/api/openai-compatibility.mdx#setting-the-local-context-size

Root-diagnostic evidence (synthetic content only):
`/private/tmp/wordcraft-synthetic-context-reproduction.json`,
`/private/tmp/wordcraft-synthetic-continuation.py` and its adjacent `.json`.
The **4,432** count belongs to the reproduced continuation; the discarded original response
body cannot establish the original request's exact count. Final native checks:
`/private/tmp/craft-provider-context-tests.log` (**6 passed**) and
`/private/tmp/craft-provider-context-clippy.log` (all-target Clippy passed).

Shared source commits: native diagnostics `4bf8fa4`, settings `be59354ae98b5c45c8c5d8062885fef697bc13d4`.
An initial push was rejected by automatic approval review because historical approval existed
only in repository documentation. The user then directly authorized these commits/destination
with “Proceed”; the retry succeeded and `git ls-remote` confirmed the exact final SHA at
`https://github.com/andykumeda/craft-libs`, branch `codex/portable-assistant`. No unresolved
publication rejection remains. WordCraft's three manifests and lockfile now use that revision;
no sibling-path overrides were introduced.

Shared evidence: `/private/tmp/craft-settings-ui-all.log` (464/2),
`/private/tmp/craft-settings-final-tests.log` (9/2),
`/private/tmp/craft-settings-ui-final-clippy.log`,
`/private/tmp/craft-settings-final-wasm.log`, and
`/private/tmp/craft-settings-final-screenshots.log` (one manual capture passed).
Inspected final screenshot: `$TMPDIR/craft-assistant-settings.png`; synthetic content only.

The initial narrow View click regression caught the generic ribbon fitter collapsing the
same-named group into a dropdown. The final implementation reserves a direct leading View
button before fitting the remaining groups, keeps fitting initialized for custom/hidden tabs,
and reuses the same toggle for Home. The 800/1440-width click test remains strict.

The shared target `/private/tmp/craft-settings-target` was removed after its checks completed
to recover disk space. Logs and inspected screenshots remain outside that cache.

### Final View capture and serial CI checkpoint

Four final synthetic captures passed and were inspected:
`/private/tmp/wordcraft-view-assistant-final-{800,1440}.png` and
`/private/tmp/wordcraft-view-assistant-final-open-{800,1440}.png`.
The final helper uses the existing original `comment` icon. Earlier captures of the collapsed
group and the fallback C icon are diagnostic evidence only, not final verification.
The first sandboxed capture could not obtain a graphics adapter; the permitted rerun succeeded.
No live window was driven and no user document was modified.

The first full CI passed fmt and workspace all-target Clippy, then stopped at the pre-existing
layout timing regression `a_word_with_alternating_advances_lays_out_in_linear_time` while the
example was still compiling (30.6x time for 8x input). It skipped subsequent gates. Log:
`/private/tmp/wordcraft-settings-view-ci.log`. After all example compilation stopped, the exact
built layout test passed in isolation (18.3x; log `/private/tmp/wordcraft-layout-timing-isolated.log`).
This is consistent with resource contention; no layout code or timing threshold changed.
A final exact-source `RUST_TEST_THREADS=1 cargo xtask ci` is running without overlapping builds,
log `/private/tmp/wordcraft-settings-view-final-ci.log`. All gates remain enabled.

The first serial run passed all **559 layout tests**, then failed the existing Full Screen
UI fixture's initial lookup. A settled synthetic capture confirms the collapsed Window menu
opens normally. The fixture interacts after only three post-tab frames while fitted group
widths/IDs are still settling; its wait is being corrected without changing toggle assertions.
Independent prechecks: all **57 assets** attributed; all **20 crates** obey layers;
all **16 WASM crates** checked successfully. The initial direct layering invocation lacked
Cargo on PATH; the corrected invocation with `/Users/andy/.cargo/bin` passed.
Logs: `/private/tmp/wordcraft-settings-assets.log`, `wordcraft-settings-layers.log`,
`wordcraft-settings-wasm-precheck.log` (all under `/private/tmp`).

### Completed source verification — not installed

The Full Screen fixture now uses the same six settling frames after switching tabs as at startup,
with all original enter/exit viewport-command and state assertions preserved. Both focused
Fullscreen and Assistant regressions passed; no Full Screen production behavior changed.
Final `RUST_TEST_THREADS=1 cargo xtask ci` completed with **exit 0** and **all eight gates passed**:
fmt, workspace all-target Clippy, **4,628 ordinary tests passed / 6 ignored / zero failures**,
separate localization audit (**one passed**), doctests, **57 attributed assets**,
**20 crates** with no layer violations and **16 WASM crates** checked.
Final log: `/private/tmp/wordcraft-settings-view-verified-ci.log`.
All task-owned compilation and captures were serialized for this final run. The earlier failed
runs remain recorded above and are not represented as passing runs.

Shared changes are committed and published to the user-approved `andykumeda/craft-libs` branch;
WordCraft integration is recorded by this checkpoint's focused local commit
(`git log -1 -- docs/assistant-implementation.md`). No WordCraft remote publication, app
replacement, download/model alias, binding change or Ollama reconfiguration was performed.
The installed preview still uses its earlier UI. The existing 4,096-token Ollama setting can
still reject document work until explicitly increased outside WordCraft's OpenAI-compatible API.
Native installed-app acceptance, physical interaction, other platforms and a live repeat of the
user's document request were not performed for this source revision. Synthetic screenshots,
HTTP reproduction and regression tests are the evidence for this increment.

## 2026-10-29 — upstream maintenance, /Applications delivery and DMG

The owner requested all future preview builds at
`/Applications/WordCraft Assistant Preview.app`, keeping the main WordCraft app separate,
and an Apple-silicon DMG for the other Mac. The in-app pane remains required: another chat/MCP
client window does not meet the goal. Independent background-service updates are a future
design, not part of the current binary. See [update workflow/design](assistant-updates.md).

| Workstream / owner | State | Acceptance and current evidence |
| --- | --- | --- |
| Upstream integration / primary | Verified | Origin/main aadb077e integrated without conflicts. Version/Cargo.lock and platform packaging changes only. Full serial CI passed all eight gates. |
| Preview updater guard / agent | Verified | --disable-updates blocks startup/manual/install routes and shows a disabled notice. Standard app default retained. Flag passed by preview launcher; new regression passed in full CI. |
| DMG packaging / primary | Verified | ZIP extraction, mounted app and copied-from-DMG app pass strict signatures; native version/ARM64 and hashes checked. DMG, checksum and install notes delivered to Downloads. |
| Install/smoke / primary | Deployed | /Applications/WordCraft Assistant Preview.app 0.12.0 (04afd95e) installed; main app native hash unchanged. View/settings/update-guard smoke inspected; normal quit, temporary socket/key released. Prior home preview archived/verified and duplicate retired. |
| Service separation / deferred | Not started | Preserve native pane; typed local service interface and upstream bridge are design only. No extra chat window or service was created. |

`git fetch origin` found upstream commits 65bb8b37, 3de6ff95, 9724f15c and aadb077e.
The current preview was based on 0.11.0; the merge brings it to 0.12.0 while retaining assistant
commits and craft-libs be59354. The initial 0.11 release compilation was stopped via SIGINT
only after identifying its cargo process through this task's private cache lock; it exited130.
No partial 0.11 binary will be distributed as the new source.

Full serial gate log: `/private/tmp/wordcraft-preview-012-ci.log`; unified parent cache
`/private/tmp/wordcraft-preview-delivery-build`. No parallel compilation during timing tests.
Syntax: `bash -n` passes for changed macOS preview/main and upstream FreeBSD packaging scripts.
A synthetic metadata-only check confirms the preview packager rejects an old binary without
--disable-updates support (exit2) before creating output files. No real app window was launched
for that check. Actual code/version, artifact hashes and deployment outcomes will follow below.

The machine is an M1 Mini with 16 GB RAM. Existing Qwen3 4B Instruct metadata advertises 262,144
maximum context, while the earlier failing server allocation was 4,096. The context slider in
the installed Ollama app can increase allocation; 16K was suggested as a starting point, not
verified as a universal document budget. Qwen3.5:4b is a candidate for a future benchmark, not
a demonstrated better WordCraft model. No model or context setting was changed by this work.

Source checkpoint: final `RUST_TEST_THREADS=1 cargo xtask ci` for the merged0.12.0 source
completed with exit0: all eight gates passed, **4,629 ordinary tests / 6 ignored / zero failures**,
localization audit, doctests,57 assets,20-crate layer check and16-crate WASM check. The new
`packaged_build_opt_out_blocks_startup_and_manual_updater_work` regression passed.
No release artifact or installed-bundle acceptance is claimed by those source checks.
The reviewed upstream merge and preview guard/packaging/workflow are committed separately;
release build and actual packaging/deployment verification follow from the feature commit.

### Delivered 0.12.0 checkpoint

Binary source: **04afd95e872e93d64a311a33c2b33f969fc7d118**, based on upstream 0.12.0
(merge 13f0ad01). Release build passed in 3m55. Native `--version`: **wordcraft 0.12.0 (04afd95e)**;
architecture **arm64**. Initial Bash-only packaging environment was first sourced under Zsh and
stopped before a build; the corrected Bash invocation succeeded. No artifact from that failed
invocation was distributed.

Installed target: `/Applications/WordCraft Assistant Preview.app`. Native signed-binary SHA256:
`d6419db1f881859112fe9943303af22edcc2d66771989e5a2724048c2affd77d`.
Installed and packaged signatures pass strict/deep verification. The launcher passes
--disable-updates; standard startup/manual/install updates cannot replace the custom build.
The separate `/Applications/WordCraft.app` remains version 0.10.0 and its executable SHA256 is
unchanged: `afe3d39b6f91a747aff129c7544d202da4d1b660a5ad3278ac112573f866f1ab`.

Artifacts under `dist/assistant-preview/04afd95e/`: matching 0.12.0 ARM64 ZIP/DMG, their SHA256
files and installation notes. DMG SHA256:
`bc44e23543b5c90c7276001541163980b7a7953cbe63e8239007759221c8847f`.
The DMG, its checksum and notes were also copied to `~/Downloads`. Signature checks passed on
staging, ZIP extraction, mounted DMG and an app copied from the DMG before publication.
Actual package log: `/private/tmp/wordcraft-preview-012-package.log`.

Installed source smoke passed using a sample and isolated document/AI profiles: native Assistant
was available, View entry and settings rendered, manual update check showed the disabled notice,
and no document edit/inference was performed. Three screenshots were inspected. Owned PID 84591
held 127.0.0.1:17981; installed executable and flag were verified before authentication. Normal
`app.quit` succeeded; the listener and key were removed. Receipt:
`/private/tmp/wordcraft-preview-012-smoke-receipt.json`; install/artifact receipt:
`/private/tmp/wordcraft-preview-012-install-receipt.json`. The local service inventory was updated
outside Git. A verified rollback ZIP of the prior 0.11 home-folder preview is retained in
`~/Applications/WordCraft Preview Backups/`; the inactive duplicate app was retired after the
new installed smoke passed. Profiles and credentials were preserved and not packaged.

Signing remains **ad-hoc, not notarized**; launch on the receiving Mac is not yet verified.
No Developer ID identity exists on this host. No public release, WordCraft source publication,
recurring update monitor, model download, provider change or context reconfiguration occurred.
The native in-app pane is retained. A separately updateable background service remains a
proposal, not an implemented component. Interim editor updates use reviewed upstream merges,
full gates and a new preview build/DMG; the existing service/runtime remains compiled into it.

Deployment documentation gate: full serial `cargo xtask ci` passed all eight steps again;
log `/private/tmp/wordcraft-preview-012-deployment-docs-ci-unrestricted.log`. The first sandboxed
attempt could not bind test sockets and failed with Operation not permitted; rerunning with
loopback access passed. A subsequent owner report shows App control returning empty activity
and claiming no document access. That new document-context handoff issue is **In progress**;
it is not covered by this delivery's UI-only smoke.

## Document context and model comparison follow-up

Updated 2026-10-29. Owner requires real end-to-end edit/read-back/saved-document/Undo acceptance
before a replacement delivery; UI-only smoke is insufficient. Privacy separation is a soft
requirement for now. General chat may be cloud-based; document instructions/content should
ideally remain local, but the current two-role route sends the task and control results through
General chat and is **not local-only**. No new routing mode is in scope.

| Workstream / owner | State | Acceptance / dependency |
| --- | --- | --- |
| Initial document context / primary | In progress | Shared 80d4fc6 supplies host-selected read before control inference; WordCraft bound inspector opts in; no document read for unrelated chat. Runtime red/green regression passed; consumer gate pending. |
| Background visibility / primary | In progress | Inspector must expose page color/picture presence so the model can identify and verify removal. Engine regression reproduced missing field; consumer gate pending. |
| Model A/B / test agent | In progress | Synthetic identical prompts/docs, Codex gpt-6.1-sol versus local Qwen Instruct; actual edit, selective instruction adherence, read-back/save-reopen/Undo, timing and failure evidence. No deployment until acceptance. |
| Delivery / primary | Not started | After source gates and live acceptance, commit/build/package/mount/copy verify/install exact preview target with rollback and DMG/checksum. Installed 04afd95e remains the earlier UI-smoked build. |

Source checkpoint: full serial `cargo xtask ci` passed all eight gates with **4,631 ordinary
tests passed / 6 ignored / 0 failed**, separate localization audit, doctests, assets/layers and
16 WASM crates. Log `/private/tmp/wordcraft-initial-context-ci.log`. Both the initial-read
regression and missing-page-background regression were first observed failing and now pass.
The candidate is ready for release compilation and model A/B acceptance; no replacement
deployment or model-success claim is made by this automated checkpoint.

### Live candidate failure and Codex-first correction

Candidate **5ae79758** was built and tested before installation. The live matrix found real
failures: Codex delegated and completed the host inspection but its first control request hit
a continuation mismatch; Qwen often did not delegate at all. Neither case counts as a successful
document edit or a valid model speed comparison. That candidate was **not installed**.
The initial matrix also exposed a harness startup-readiness race before any inference; those
receipts remain labeled invalid setup, separately from the actual model failures.

The Codex cause is now repaired in shared revision **b1a7edc**: host context is followed by the
actual user task so the first control inference starts a new exchange; genuine provider tool
results still resume the exact existing exchange. Native deterministic regressions passed,
and the live read-only Codex transport check passed in20.310s with2reads. Full consumer
edit/read-back/save/reopen/Undo acceptance remains pending.

User priority: **Codex first; proceed to Qwen only after the app works as expected**. All further
Qwen probes and routing changes are deferred. The in-flight mixed/Qwen turn was cancelled via
normal assistant Stop, then normal quit/socket cleanup; it is excluded from latency rankings.
Next acceptance is five Codex-only fresh turns: two short background tasks, the full sample
handbook background task, a selective edit and an embedded-document-instruction case.

Evidence: `/private/tmp/wordcraft-ab-5ae79758-ready/baseline-summary.md` and per-run synthetic
receipts; `/private/tmp/craft-host-context-codex-live.log`. The benchmark folders contain an
auth-home symlink and configuration references: never package them recursively. Only synthetic
DOCX/page images/redacted receipts or a report are shareable. Native window capture can be
occluded on this shared machine; engine page exports remain the document-visual oracle.

Codex correction consumer checkpoint: all eight serial `cargo xtask ci` gates passed again,
4,631 ordinary tests / 6 ignored / 0 failed, separate localization audit, doctests, assets/layers
and 16 WASM crates. Log `/private/tmp/wordcraft-codex-context-start-ci.log`. The new candidate
still requires full Codex app acceptance before deployment. After Codex passes, the main A/B
will hold General chat on Codex and vary only App control (Codex vs Qwen), matching the owner's
setup and avoiding a two-variable comparison. Earlier all-local Qwen non-delegation remains an
ancillary finding; no Qwen changes or further tests were made during the Codex-first stage.

### Codex app acceptance and external MCP comparison

Candidate **259ac26797f45cbf84fd92ddc2df61e37682431c** passed the five actual Codex app tasks:
short background removal twice, full sample-handbook background removal, selective editing and
ignoring an embedded document instruction. End-to-end seconds: **23.665, 46.159, 20.431,
21.825, 20.605**; median **21.825**. Each run checked actual edits, saved/reopened DOCX, unchanged
text/formatting and one normal Undo, then normal quit/socket cleanup. Before/after exported
sample pages were visually inspected by the primary agent. Source CI passed all eight gates;
release build passed in3m07. This candidate is **not installed yet**.

Evidence: `/private/tmp/wordcraft-ab-259ac267-codex/results.json`. The sample oracle initially
flagged automatically updated core modified-time/revision metadata; the correction ignores
only those fields. Original receipt and stdout are retained. Blocks, parts, sections, styles,
text and normalized DOCX main XML were unchanged except the requested background.

New owner requirement: compare the native panel to an external Codex MCP client, expecting
at-worst-identical functionality and performance. **In progress**: matching release CLI bridge
and isolated external client, same model/fixtures/oracles, separate raw MCP versus model-task
latencies. Static catalog inspection finds **17 external tools vs 9 native tools**; native
lacks new_document, render_page, list_documents, parity, screenshot, click, key and ui_inspect,
and deliberately binds the turn's document and file grants. Full functional parity is **not
established**. No broad tool/guard changes have been made merely to claim parity.

Qwen inference remains deferred until the Codex external comparison completes; the main
subsequent A/B holds General chat on Codex and varies only App control. No user model bindings,
credentials, global context or real documents changed. No replacement deployment occurred.

### Current integration: performance, visual parity and Terra condition

The owner expects the panel to match the external MCP path at worst. Measured baseline
259ac267: both paths passed5/5document tasks, but native median21.825s vs external12.793s
(~1.71xobservednative). Bridge overhead was noise-scale (50alternatingreads:16.623msMCP
vs16.565msdirectcontrol); this is a small ordered sample, not a universal speed claim.
External17tools/655commands vs native9/363also exposed capability gaps, including visual
background commands. Source and artifacts are in
`/private/tmp/wordcraft-external-259ac267-report/comparison.md` and comparison.json.

Sharedc0177ca now uses one bounded MCP tool loop when both selected connection+model choices
are identical. Different choices retain their explicit two-role routing. Existing validation,
file/target guards, checkpointing, cancellation and no-replay handling remain; general chat
does not automatically read document content. Native52tests/4ignored/0failed and a read-only
live Codex direct-tool probe passed (15.837s); actual app latency must be remeasured.

The consumer visual parity slice adds document/view/interface background inspection, explicit
visual setters, safely scoped document creation/listing and read-only UI metadata. Full
image/UI-input parity is not claimed. Integration gates and model tests remain pending.

Owner also requested GPT6.1Terra: run identical Sol/Terra functional and latency tests; switch
WordCraft's per-app default only if Terra matches task quality and is no slower on the measured
workload. No other app's defaults should change. Official[Codex pricing](https://learn.chatgpt.com/docs/pricing)
checked2026-10-29 lists Standard-speed per-million-token credits Sol50input/2.5cached/250output
and Terra25/1.25/125; Terra's per-token rates are half, while actual cost depends on tokens used.
No model binding has been changed yet. Qwen remains a later comparison with General chat held
fixed so only App control differs. All inference uses synthetic documents.

Integration checkpoint: all eight serial `cargo xtask ci` gates passed: **4,641 ordinary tests
passed / 6 ignored / 0 failed**, separate localization audit, doctests, assets/layers and 16 WASM
crates. Focused host tests passed **34 / 1 ignored**. Logs:
`/private/tmp/wordcraft-direct-parity-ci.log`,
`/private/tmp/wordcraft-direct-parity-host-tests-verified.log`. Initial focused failures were
fixed before these passes: accurate listing expectation, explicit interface-theme frontend
dispatch, and the test's expected dark-page preference for its own requested visual change.
Malformed theme values and unrelated pending UI requests are explicitly covered.

Native scoped tools now number **13 of 17**. Page rendering, screenshots, click and key remain
external-client capabilities; broader app/macro/settings and arbitrary cross-document access
remain guarded. No full capability-parity or new live-performance claim is made by source
gates. Next release candidate will test Sol and Terra on seven identical fresh-chat cases; all
three background sources use the exact original prompt. No replacement installation yet.


### 2026-10-29 — In progress: one model and bounded warm-session setup

Shared source `bf932b91643c287502d539b88f08abd5796c9447` was committed and published to the owner-approved fork/branch; remote SHA verified. WordCraft pins that exact revision in all manifests and Cargo.lock. Shared native tests passed 56 with 5 ignored; UI passed 465 with 2 ignored; Clippy and core/UI WASM passed. The final native recheck required loopback permission for synthetic HTTP tests and passed after rerunning with that permission.

The owner requested one conversation and one model selector. The pending WordCraft candidate
resolves only `assistant.chat` and uses that selected connection/model for the direct MCP tool
loop. A Chat-only binding works without Control. Legacy Control bindings, credentials and chat
history are preserved but cannot reroute a request or serve as fallback. The visible route and
canonical status `model` describe the single choice; legacy status route fields are aliases.
OpenAI subscription through Codex, Ollama and OpenRouter/API remain the supported connection
paths; this checkpoint does not establish new hosted-provider live acceptance.

The candidate keeps one Codex provider template per app session/configuration root, sharing only
successful compatibility proofs in memory. Each turn forks fresh authentication-verification
state and pending callback exchanges; it still checks the account before private context.
Executable identity, model and runtime configuration changes invalidate compatibility proofs.
Sign-in, settings reload and root changes clear the template. Creating/opening documents and
switching windows do not clear it. HTTP providers still receive a fresh secret store per turn.
The cache shares no conversations or pending tool exchanges; compatibility proofs never reach disk.

Background inspection bundles saved page fill, dark-page display and interface appearance with
registry-backed command metadata. Successful explicit background setters include authoritative
post-change state in their tool result, reducing separate discovery/readback requests. Visual
inspection does not authorize reading writing; bound-target, selection, revision and file-grant
checks remain. Full external-MCP parity remains open: native exposes 13/17 tools, while page
rendering, screenshots, click and key remain external capabilities.

The candidate also adds `ui.assistant.newChat` through the existing asynchronous New chat action,
preserving previous chats and busy/cancel guards. Warm-session automation must wait for changed
opaque `chatId`, `settingsBusy:false` and `busy:false` before sending. This allows fresh chat
history while retaining app-session compatibility proofs; it does not reuse a Codex conversation.

| Workstream | State | Verification / next gate |
| --- | --- | --- |
| Single model, template isolation and bundled background metadata/verification | Verified (source) | Full `RUST_TEST_THREADS=1 cargo xtask ci` passed all eight gates; ordinary test stage: 4,649 passed, 6 opt-in tests ignored. Log `/private/tmp/wordcraft-unified-ci.log`. Shared pin `bf932b91643c287502d539b88f08abd5796c9447`; release build and live acceptance remain pending. |
| Codex Sol/Terra comparison | Not started for this candidate | Identical seven-case functional/latency cohort, fresh chats, cold and warm setup distinguished; exact original background prompt across all three background sources; visible/saved/reopened results and Undo |
| Conditional Terra default | Not started | Change only if task quality matches Sol and measured latency is no slower; no binding changed by this checkpoint |
| Qwen comparison | Not started for this candidate | Follow successful Codex acceptance; same selected-model path and synthetic-document oracles |
| Preview replacement and portable DMG | Not started for this candidate | Package/verify/install only after required source and actual-model acceptance |
| Existing preview delivery | Deployed | `/Applications/WordCraft Assistant Preview.app`,0.12.0 source `04afd95e`; earlier delivery evidence remains above |

Documentation checkpoint only: no new compilation, tests, model inference, performance result or
installation is claimed here. The prior `259ac267`/`e3ed7002` measurements remain historical baselines;
no new latency or privacy guarantee follows from this implementation. The installed preview and
separate main app remain unchanged. Other-Mac launch acceptance remains open.


### 2026-10-29 — Verified and Deployed: unified chat with measured latency reduction

Release source **`9c6c2cb04ae122ed0d4bcbf2838ab69c0f249531`**, shared pin
`bf932b91643c287502d539b88f08abd5796c9447`. All eight source gates passed before the
source commit; 4,649 ordinary tests passed and 6 opt-in tests were ignored. Release app and
matching CLI built together successfully. No compilation ran during model timing cohorts.
Detailed case results, failed attempts, cold/warm conditions and comparison caveats are in
[assistant-benchmarks.md](assistant-benchmarks.md).

| Workstream | State | Evidence / limits |
| --- | --- | --- |
| Single chat/model, settings and direct tools | Verified and Deployed | Chat-only canonical binding, preserved legacy Control data; actual installed settings and prominent View Assistant screenshots inspected |
| Sol/Terra document and general-chat acceptance | Verified | Both 7/7 document/display cases, exact readback, saved/reopened DOCX, formatting preservation and normal Undo; same-chat general→document passed; three general repeats each passed with no document tools |
| Panel performance | Verified for measured cases | Sol warm core median8.910s vs matched-source external12.887s; Terra7.218s vs9.937s. Fresh external clients/apps and warm native process are distinct conditions. Native cold setup13.770/11.602s; no universal or first-turn speed guarantee |
| Qwen comparison | Verified comparison; reliable local acceptance not achieved | Local Qwen4B16K: 2/7 warm cases pass, cold handbook separately passes. Five incorrect background/display cases retained, including quoted null and wrong-setting changes/false success. No silent cloud fallback/tuning |
| Terra default | Deployed | Only WordCraft `assistant.chat`: craft-codex/gpt-6.1-terra; previous Sol binding privately backed up. Other app/suite bindings, legacy Control, connections, credentials and histories unchanged; locked atomic update independently compared |
| External MCP comparison | Verified for cases | Sol7/7 and Terra5/5, actual read-before/readback/files/Undo and visual invariants; all clients/apps quit normally and test keys/ports released |
| Full capability parity | In progress | Native13/17 tools; image rendering/capture and guarded mouse/keyboard input remain unsupported. No claim of full parity |
| Preview and portable package | Deployed | Exact /Applications preview, mounted DMG and copied bundle verified; previous preview rollback ZIP independently extracted/signature/hash verified; main WordCraft0.10.0 unchanged |
| Installed model edit | Verified | Terra unchanged prompt “remove the dark background”, synthetic full handbook;11.477s, actual background/content/format/save/reopen/Undo checks pass. Installed before/after screenshot inspected; normal quit/key/17981 cleanup |
| Other Mac / hosted provider / signing | Not started for this build | Other-Mac execution and live OpenRouter inference untested; ad-hoc signing only, no notarization. No public release or WordCraft source push |

Delivery artifacts (Apple silicon):

- Installed `/Applications/WordCraft Assistant Preview.app`, `0.12.0 (9c6c2cb0)`.
  Packaged/installed native SHA-256 `f70fb9ddad4235e6283b40eda7f50ea8325826a86c6b8d3fc82be8d68baaf7e8`.
  Ad-hoc re-signing changes the release binary hash; the benchmark report records the unsigned build hash separately.
- DMG `dist/assistant-preview/9c6c2cb0/wordcraft-assistant-preview-0.12.0-9c6c2cb0-macos-aarch64.dmg`;
  SHA-256 `cea0796333f9ca009ea685a9c58a1a15184dd47546a05216b36ab8b248f288c2`.
- ZIP same prefix; SHA-256 `874126993ee750efaad9afc11bcc5db98206e044da82107d526033d155355992`.
- DMG, checksum and installation instructions copied to Downloads and byte/hash verified.
- Previous04afd95e rollback `/Users/andy/Applications/WordCraft Preview Backups/wordcraft-assistant-preview-0.12.0-04afd95e-replaced-9c6c2cb0.zip`;
  SHA-256 `586fcf288e62da867ddd77801a335fce610a3c05189b96c181c3513505802e44`.
- Installed launcher still passes `--disable-updates`. Main app SHA-256 remains
  `afe3d39b6f91a747aff129c7544d202da4d1b660a5ad3278ac112573f866f1ab`.
- Receipts `/private/tmp/wordcraft-preview-9c6c2cb0-{install,smoke}-receipt.json`,
  `/private/tmp/wordcraft-preview-installed-edit-receipt.json`, `/private/tmp/wordcraft-terra-default-receipt.json`.
  Temporary test profiles and the private settings rollback are not included in distributable artifacts.

Privacy remains a soft preference. The selected provider receives both chat and any document
content required by its tools. Local Ollama requests used only the verified loopback endpoint;
no network packet/air-gap audit was performed. Switching providers in an existing chat sends
that chat history to the newly selected provider; start a new chat to keep earlier local context
separate. A stronger local model or LAN-hosted Ollama is an untested next candidate, not an
installed replacement. No new model was downloaded.

Final delivery-document gate: `RUST_TEST_THREADS=1 cargo xtask ci` passed all eight steps again; log `/private/tmp/wordcraft-unified-delivery-final-ci.log`. No source code changed after model acceptance. Final delivery documentation is committed separately from the packaged source revision.
