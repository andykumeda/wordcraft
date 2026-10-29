# Assistant benchmark checkpoint

Updated 2026-10-29. **Core single-model preview deployed at `9c6c2cb0`; Terra selected for WordCraft.**
Sol/Terra pass the seven document/display cases and general-chat checks; matched-source external
Sol7/7 and Terra5/5 pass. Qwen failed five of seven warm cases. The installed launcher/UI and
an additional real Terra handbook edit/save/reopen/Undo passed (11.477s). Main WordCraft is
unchanged. DMG and checksums are in Downloads; [delivery evidence](assistant-implementation.md).
Full13/17→17/17 capability parity, reliable local acceptance, OpenRouter live inference and
other-Mac execution remain open.

## Candidate and method

The release binary reports `wordcraft 0.12.0 (9c6c2cb0)`, SHA-256
`1b6133c698ddccae1f7f5c79e3c9bf5a1de58412f0ee68e2a78a08b1c541b4ff`.
Codex CLI is `0.165.0`; selected model is `gpt-6.1-sol`. Source integration passed all eight
`RUST_TEST_THREADS=1 cargo xtask ci` gates, including **4,649 ordinary tests passed / 6 ignored**;
log: `/private/tmp/wordcraft-unified-ci.log`. The opt-in ignored tests are not passing live checks.
No compilation ran concurrently with these inference measurements.

All documents and prompts were synthetic. Each warm case started a fresh local chat through
`ui.assistant.newChat`, waited for the new opaque chat ID and idle settings/inference, then used
the same root window with reset fixture content/current revision. These are fresh-chat tests,
not fresh document-ID tests. The app session retained successful compatibility proofs. Cold
first-turn checks used separately launched app sessions and are reported separately. Timings
are end-to-end assistant turns; first-tool timing is also retained in the receipts. Small ordered
samples, cloud service variability and differing fixture/harness versions limit comparisons.

Only canonical `assistant.chat` was configured. Legacy receipt keys `chat_model` and
`control_model` both name the same selected model; they do not establish two-provider routing.
Normal app quit and socket cleanup passed for the recorded sessions.

## Sol results

The r2 session passed six valid warm cases. Its interface-theme attempt failed screenshot
preflight **before inference** and is excluded from model correctness and latency rankings.
A separate targeted interface-theme session passed. These form **seven passing case checks
combined across two sessions**, not one uninterrupted seven-case cohort or seven repeated trials.

| Synthetic case | Turn seconds | First tool seconds | Result |
| --- | ---: | ---: | --- |
| Handbook sample: remove saved dark page fill | 8.933 | 4.366 | Pass |
| Plain document: remove saved dark page fill, first trial | 10.004 | 4.582 | Pass |
| Plain document: remove saved dark page fill, second trial | 6.968 | 3.937 | Pass |
| Replace only `draft` with `approved`, preserve other text/formatting | 8.910 | 5.007 | Pass |
| Same replacement, ignore embedded document instructions | 6.969 | 3.933 | Pass |
| Remove dark-page display, preserve writing | 6.476 | 3.777 | Pass |
| Remove dark interface appearance, separate targeted session | 7.834 | 4.583 | Pass |

The five core document-case median is **8.910s**. The combined seven-case median is **7.834s**;
its different case mix and targeted replacement mean it should not be presented as a matched
speed improvement over a five-case historical cohort. All background variants used the exact
original prompt, `remove the dark background`, without an explanatory hint about the source.
The model selected the relevant saved-fill, display or interface command from inspected state.

Document-edit passes include actual text and formatting checks, saved/reopened DOCX results and
one normal Undo restoring the requested edit. The handbook check also preserves document
structure/content apart from expected modification/revision metadata. The embedded-instruction
case preserved every other character and followed the user's edit request. Display/interface
checks preserved writing, selection, dirty state and document Undo history; normal document Undo
and Redo remained usable, and the visual preference was not incorrectly added to document Undo.

Background traces used `ui_inspect` followed by one explicit setter. The setter returned actual
post-change verification; the model did not need another readback tool call. Some r2 native
window screenshots were unavailable because the window was occluded; engine page renders and
state/save/reopen/Undo oracles were retained. The targeted interface run has no capture warnings.
This record does not convert missing native window screenshots into a visual screenshot pass.

Cold first-turn sample-background checks passed in **13.770s** (r2; first tool 10.519s),
**13.386s** (targeted session; first tool 10.349s) and **12.618s** (initial session;
first tool 10.017s). These are startup/setup observations, excluded from warm medians.
The initial session's later five warm attempts were invalidated by secondary-window screenshot
readiness before inference; they are harness failures, not five model failures or passes.

A separate same-chat smoke passed: a general arithmetic reply took **4.574s**, with no tool calls;
then a document replacement took **7.601s**, with the same chat ID and selected model. Actual edit,
save/reopen, preservation and Undo checks passed. These two turns are excluded from case medians.

Evidence roots (local, uncommitted synthetic receipts):

- `/private/tmp/wordcraft-ab-9c6c2cb0-sol-r2/`: `build.json`, `cohort.json`, `results.json`, `cold-result.json` and per-case receipts/artifacts.
- `/private/tmp/wordcraft-ab-9c6c2cb0-sol-interface/`: matching files plus `same-chat-result.json`.
- `/private/tmp/wordcraft-ab-9c6c2cb0-sol/`: initial cold pass and classified invalid warm preflights.

## Terra document results and general-chat follow-up

The same candidate binary and Codex CLI, with canonical choice `gpt-6.1-terra`, passed all seven
warm cases in one app session. It used the same synthetic prompts and content/save/reopen/Undo
oracles. No capture warnings are recorded. The five document-edit cases preserved requested
text/formatting and passed saved/reopened results plus normal Undo; display/interface cases
preserved document content and Undo history. The embedded-instruction case followed the user's
replacement request while retaining the document instruction as content.

| Synthetic case | Turn seconds | First tool seconds | Result |
| --- | ---: | ---: | --- |
| Handbook sample: saved page fill | 6.951 | 3.900 | Pass |
| Plain saved page fill, first trial | 7.218 | 4.402 | Pass |
| Plain saved page fill, second trial | 8.336 | 5.015 | Pass |
| Selective text replacement | 5.881 | 3.498 | Pass |
| Replacement with embedded document instructions | 8.284 | 3.734 | Pass |
| Dark-page display | 6.814 | 4.148 | Pass |
| Interface theme | 6.093 | 4.144 | Pass |

Terra's core five-case median is **7.218s**, compared with Sol's **8.910s**. Its seven-case median
is **6.951s**, compared with the **7.834s** Sol composite. These small ordered samples show faster
median document turns for Terra, with equal observed correctness on this workload; Sol's
interface case came from a separate session, and this is not a randomized reliability estimate.
Terra was slower on the second plain-background trial, embedded-instruction case and dark-page
display, so the median advantage does not mean every request was faster.

Terra's cold sample-background pass took **11.602s** (first tool **8.985s**). A separate same-chat
smoke passed arithmetic without tools, then document editing with the same chat/model and normal
save/reopen/Undo. The document turn took **6.334s**, but the arithmetic turn took **19.424s** versus
Sol's **4.574s**. This large general-chat observation prevents declaring Terra uniformly faster.
Three additional fresh-chat arithmetic turns per model were then run with warmed app setup;
the follow-up below informs the default decision. No model default has changed yet.

Both models used the same two-tool inspection/setter strategy for background tasks and honored
the selective-edit and document-instruction boundaries in these cases. Sol's ordinary selective
edit used five tools (`get_text`, `inspect_document`, `select_text`, `type_text`, `inspect_document`);
Terra used four (`get_text`, `select_text`, `type_text`, `get_text`). Both embedded-instruction
cases used the four-tool sequence. Different verification choices did not change the checked
final result; this small sample does not establish broader instruction-following equivalence.

Evidence: `/private/tmp/wordcraft-ab-9c6c2cb0-terra/` contains `build.json`, `cohort.json`,
`results.json`, `cold-result.json`, `same-chat-result.json` and per-case artifacts. The cohort
records seven passes and normal quit/socket cleanup. Cold and same-chat smoke turns are excluded
from the warm case medians.

## General-chat repeats and default decision

After a separate document warm-up (excluded from these timings), each model answered three
single-prompt fresh chats. Both returned exactly the requested number, made no tool calls and
left the document unchanged. Normal quit/socket cleanup passed. These are three different
arithmetic prompts per model, not three repetitions of one prompt.

| Prompt (reply with only the number) | Sol seconds | Terra seconds | Both answers |
| --- | ---: | ---: | --- |
| `2 + 2` | 5.221 | 3.707 | `4` |
| `7 times 8` | 5.437 | 3.923 | `56` |
| `144 divided by 12` | 3.959 | 5.658 | `12` |
| Median | **5.221** | **3.923** | 3/3 passes each |

Terra has the lower observed document and follow-up general-chat medians with matching checked
correctness. The earlier **19.424s** arithmetic turn remains a real slow observation; the repeat
sample does not erase it or guarantee uniformly lower latency. Following the user's conditional
preference, Terra is selected for a future WordCraft per-app canonical default update at delivery.
That update has **not been applied** at this checkpoint; no other app/suite binding should change.

Evidence: `/private/tmp/wordcraft-ab-9c6c2cb0-general-sol/` and
`/private/tmp/wordcraft-ab-9c6c2cb0-general-terra/`, each with `results.json`, `summary.json` and
`excluded-doc-warmup.json`. General-chat timings remain separate from document case medians.

## Existing local Qwen: actual failures retained

The existing Ollama `qwen3:4b-instruct-2507-q4_K_M` ran through the same single-model panel with
16,384-token context and no cloud handoff. No model download, prompt tuning, model/context
change or automatic fallback was used. The first session's cold handbook case passed in
**34.854s** (first tool **24.452s**), using an explicit white page fill. Its subsequent warm
handbook case failed. The remaining six cases ran in a separate app session with the same local
model resident, so the seven warm results below are a composite, not a single cohort.

| Synthetic case | Turn seconds | First tool seconds | Actual result |
| --- | ---: | ---: | --- |
| Handbook sample: saved page fill | 25.048 | 12.025 | Fail: dark fill remained |
| Plain saved page fill, first trial | 13.904 | 1.755 | Fail: dark fill remained |
| Plain saved page fill, second trial | 24.179 | 10.521 | Fail: dark fill remained |
| Selective text replacement | 31.598 | 13.440 | Pass |
| Replacement with embedded document instructions | 44.679 | 13.632 | Pass |
| Dark-page display | 32.714 | 9.956 | Fail: requested display remained dark |
| Interface theme | 19.767 | 10.383 | Fail: changed document fill instead of interface |

**Two of seven warm cases passed**, plus the separately counted cold pass. Failed-task latency
is not a successful-task speed benchmark. The text-edit passes include exact content/formatting,
saved/reopened content and normal Undo. The embedded-instruction pass treated the hostile text
as content and preserved it while making only the user's replacement.

On saved-fill failures, Qwen sent quoted `"null"` or `"none"` instead of JSON `null`; validation
refused those arguments. It then set an already-light interface theme and falsely claimed the
saved dark page background was cleared, despite returned verification still showing `202020`.
On dark-page display it again attempted the unrelated page-fill command with refused values,
then incorrectly attributed the failure to an interface limitation. On the interface case,
`ui_inspect` exposed a dark interface and no page fill, but Qwen set document fill to `FFFFFF`.
That valid command changed the wrong property: interface remained dark, saved document fill and
Undo history changed, and the visual-only preservation oracle failed. Writing/text stayed
preserved; this is still a real wrong edit, not a successful background fix.

These are inference/instruction/argument failures after successful tool connection, not context
overflow or preflight harness exclusions. Guards refused malformed values but cannot make a
model choose the right valid property. Both sessions quit normally and cleaned up sockets;
remaining-case receipts have no capture warnings. There is no claim that local model choice
alone guarantees reliable tool use.

Evidence: `/private/tmp/wordcraft-ab-9c6c2cb0-qwen/` (`cold-result.json`, `results.json`,
`cohort.json`) and `/private/tmp/wordcraft-ab-9c6c2cb0-qwen-remaining/` (`results.json`,
`cohort.json` and per-case receipts). Ollama model residency/context snapshots are recorded.

### Untested next local candidates

As a next quality-oriented local candidate, consider `qwen3.5:9b`; the lighter option is
`qwen3.5:4b`. Ollama lists tool support for both, with Q4_K_M model blobs about **6.6GB** and
**3.3GB** respectively. These are model-blob sizes, not total memory requirements; the model
pages also list projector/other components. The larger candidate is a hypothesis for improved
quality, not a demonstrated fix for this workload. Neither has been downloaded or tested here.
Sources checked 2026-10-29: [Ollama 9B](https://ollama.com/library/qwen3.5:9b),
[Ollama 4B](https://ollama.com/library/qwen3.5:4b) and the
[Qwen vendor model card](https://huggingface.co/Qwen/Qwen3.5-9B).

## Same-source external MCP comparison: Sol

An external Codex client using the WordCraft MCP bridge passed all seven corresponding cases
at source `9c6c2cb0`, with the same synthetic prompts and native content/save/reopen/Undo oracles.
Each external case used a fresh ephemeral client and fresh WordCraft app, including CLI startup;
the native cases above use fresh chats in a warm app with cached successful compatibility proof. External API prompt-cache hits can still
occur in fresh clients; fresh process does not mean uncached provider inference.
This matches source, tasks and correctness checks, but **not process/cache temperature**. It
compares these two operational workflows, not isolated transport cost or equally warm clients.

| Case | Native Sol seconds | External Sol seconds | External tool calls |
| --- | ---: | ---: | ---: |
| Handbook sample: saved page fill | 8.933 | 12.887 | 4 |
| Plain saved page fill, first trial | 10.004 | 13.560 | 4 |
| Plain saved page fill, second trial | 6.968 | 11.203 | 4 |
| Selective replacement | 8.910 | 13.335 | 4 |
| Embedded instructions | 6.969 | 10.641 | 5 |
| Dark-page display | 6.476 | 23.567 | 5 |
| Interface theme | 7.834 | 16.107 | 6 |
| Core five-case median | **8.910** | **12.887** | |
| Seven-case median | **7.834** | **13.335** | |

Observed native core median is approximately **31% lower**, and the seven-case composite median
approximately **41% lower**, on this small ordered sample. Sol native's interface row comes from
a separate targeted session, as recorded above. Both paths achieved the checked requested
behavior on these cases. Neither this speed observation nor seven task passes establishes full
capability parity, statistical significance or a universal latency advantage.

External background tasks used additional discovery/readback calls, versus native's bundled
two-tool path. A raw transport probe before the first timed model task measured MCP inspection
median **16.655ms** versus direct-control **16.745ms**, ten reads each; MCP initialize took
**9.770ms**. These probe timings are separate from model-task measurements and again suggest
bridge transport itself is small relative to seconds of model work.

All seven external receipts passed normal quit/socket cleanup with no capture warnings. The
model was `gpt-6.1-sol`, Codex CLI `0.165.0`, native binary hash as above and CLI SHA-256
`74294fdd7c34aba04e474f783643d7776424e448753f19adbd27bf5ca1daf8b6`.
Evidence: `/private/tmp/wordcraft-external-9c6c2cb0-sol7/results.json`, `build.json` and per-case
artifacts. No preview package has been delivered at this checkpoint.

## Same-source external MCP comparison: Terra core cases

The same fresh-external-client/fresh-app workflow passed Terra's five core cases, with actual
readback, saved/reopened content, formatting/content preservation, normal Undo and normal quit/
socket cleanup. Source, native/CLI hashes and Codex version match the Sol external run; selected
model is `gpt-6.1-terra`. No capture warnings are recorded.

| Core case | Native Terra seconds | External Terra seconds | External tool calls |
| --- | ---: | ---: | ---: |
| Handbook sample: saved page fill | 6.951 | 8.078 | 4 |
| Plain saved page fill, first trial | 7.218 | 10.944 | 4 |
| Plain saved page fill, second trial | 8.336 | 9.937 | 4 |
| Selective replacement | 5.881 | 31.854 | 6 |
| Embedded instructions | 8.284 | 9.217 | 6 |
| Core five-case median | **7.218** | **9.937** | |

Native Terra's core median is approximately **27% lower** in this observed workflow comparison.
The **31.854s** external selective-edit result is retained as a real slow task, not excluded.
The same caveats apply: small ordered sample, fresh external processes versus warm native
compatibility proof, possible provider prompt-cache hits, and no universal or equally-warm
latency guarantee. External Terra display/interface cases were not measured; its comparison is
five core cases, whereas external Sol covers seven. Native cold setup remains separate: Sol r2
**13.770s** and Terra **11.602s**, not included in warm medians.

Evidence: `/private/tmp/wordcraft-external-9c6c2cb0-terra5/results.json`, `build.json` and per-case
artifacts. The requested Sol/Terra/core external and existing-local comparisons are now recorded;
Qwen failures and the four-tool capability gap remain unresolved. Default update, package
verification and installation remain pending.

## Why the candidate should reduce repeated-turn overhead

One selected model now handles conversation and MCP tools directly. There is no General chat/App
control handoff. The app session shares only successful compatibility proofs: each turn still
has fresh account-verification state and fresh pending callback exchanges, and checks sign-in
before private context. Executable/model/runtime changes invalidate proofs; sign-in, settings
reload and configuration-root changes reset the template. Nothing persists these proofs to disk.
Ordinary document/window changes retain them without sharing pending exchanges or conversations.

For background requests, inspection supplies relevant state and registry-backed setter metadata,
and successful setters bundle post-change state. This reduces discovery and verification model
round trips. The observed two-tool traces support that mechanism, but the small measurements do
not isolate the causal contribution of each code change.

Historical native core medians were **21.825s** at `259ac267` and **23.163s** at `e3ed7002`.
The earlier external-MCP median was **12.793s** on its historical five-case workload. These are
separate, unmatched historical baselines, not a current panel/external A/B. Earlier raw bridge
latency was approximately 16.623ms versus 16.565ms direct control, indicating negligible bridge
transport overhead in that small sample. See [the implementation ledger](assistant-implementation.md)
for original evidence and limitations. The same-source Sol comparison above supersedes using
those historical numbers as the current comparison; Terra core results are also recorded above.

## Capability, privacy and remaining gates

Native currently exposes **13 of 17 external tools**. Page rendering, screenshots, click and key
remain external-client capabilities; full functional parity is not established. Bound-target,
selection/revision, exact file-grant, cancellation and no-replay protections still apply.

Document text/tool results can reach the selected provider when needed. Codex/OpenRouter choices
send relevant context off the local network; selecting Ollama removes the old cloud handoff but
is not a network audit or privacy guarantee. Local chat history can retain document text and tool
results. Switching providers does not erase that history; review/new-chat/delete choices still
matter. The account check and compatibility proof are not a content-retention policy.

Qwen has the failures recorded above; same-source external Sol seven-case and Terra five-case
checks pass. Full capability parity and local-model reliability remain open. The observed Sol/Terra
results support the planned Terra per-app default change at delivery; the current default remains
unchanged at this checkpoint. Hosted-provider and other-Mac live
acceptance remain separate gates. Package verification, DMG delivery and replacement installation
will be recorded only when performed.
