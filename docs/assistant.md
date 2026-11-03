# Desktop assistant

Current source uses the independently updated shared helper and the [complete external MCP interface](mcp-assistant-parity.md). The installed preview described below is an earlier build; this task has not replaced it.

Status: preview **0.12.0 (`9c6c2cb0`)** installed at
`/Applications/WordCraft Assistant Preview.app`, 2026-10-29. Single-model settings, View discovery
and disabled-updater smoke passed. Sol/Terra actual document/readback/save/reopen/Undo checks and
external MCP comparisons are recorded in [benchmarks](assistant-benchmarks.md). WordCraft's
per-app model is now Codex `gpt-6.1-terra`; suite/other-app settings and conversations are preserved.
The exact installed bundle's Terra handbook edit passed in **11.477s**, including visible result,
saved/reopened content and formatting, and one normal Undo. Hosted-provider, broader
adversarial containment and other-Mac acceptance remain open. This is a private local preview,
not a public release; see [the implementation ledger](assistant-implementation.md).

Open **View → Assistant** (the first ribbon button) or **Home → Assistant** to show or hide the pane. The pane can dock, float and
resize like the other task panes. **File → Options → Agents** contains its connection and model
settings. Browser inference is not available.

The installed preview uses **one model choice** for general conversation and document work.
Choose an OpenAI subscription through Codex, a local Ollama connection, or an OpenRouter/API
connection. The selected model calls WordCraft's MCP tools directly in the same conversation;
there is no General chat/App control handoff. Fresh settings leave the choice unset. Select a
model with streaming and tool support before sending. The pane shows its effective connection
and model. Missing connections, unavailable models and unsupported capabilities fail visibly;
WordCraft never chooses another provider for you. OpenRouter/API setup is supported; its live
provider acceptance has not been performed.

Expand **Add or edit a connection → API or local server** to add an OpenAI-compatible connection using the OpenAI, OpenRouter, Ollama, LM Studio or custom
endpoint preset. The endpoint is the API base, including its version path. Discover models, or enter
a model ID under **Advanced model options**. Document work requires tool support: use the capability check or
explicitly confirm that a manually selected model supports tools. **Check model tool support → Verify tool support** sends
synthetic prompts, which can incur normal provider usage charges.
**Test connection** only requests the provider's model catalog. Save the connection, choose a
connection and model, then select the Apply button, which names whether the change affects this app or all Craft apps.
**Advanced model options** contains the app override switch and **Use suite default**. API keys go into the platform credential
store, separate from settings and chats. If that store is unavailable, saving the key fails visibly.

Codex uses a separate Craft-managed sign-in. It does not reuse your coding assistant's credentials,
history, settings or MCP registrations. Under **Add or edit a connection → Codex account**, choose **Sign in to Codex**, then
enter the one-time code shown in WordCraft on the browser page. Keep WordCraft open until it shows
**Codex connection saved**, then choose and apply the model. Signing into the browser
account alone does not complete the device connection.

The installed Codex executable must pass compatibility and
containment checks before document context is supplied. Failures remain visible and do not switch
providers. Set `CRAFT_CODEX_BINARY` to an absolute executable path when Codex is not on the
desktop app's PATH. Sign-in and discovery alone do not confirm readiness: a synthetic callback
and result-continuation check must succeed for the selected model before document context is sent.
Earlier Codex basic edit/read-back/Undo acceptance passed; broader adversarial containment remains
a release gate. The preview retains successful compatibility proofs in memory
for the app session. Every turn gets fresh callback exchanges and checks the signed-in account
before private context; conversations and pending tool exchanges are never shared between turns.
Changing the executable, model or runtime limits invalidates the proof. Reloading settings,
starting sign-in or changing the configuration root clears the template. Ordinary document or
window changes keep it. No proof is saved to disk. Source gates and synthetic model acceptance
are recorded in the ledger and benchmarks.

Ask for document work in the same conversation as general chat. Requested edits apply directly and
use normal **Undo**. Relevant document text and tool results go to
the selected provider when the task needs them. Selecting Ollama removes the two-provider handoff;
this is not a network privacy audit. Existing chat history can contain document text and tool
results; changing providers does not erase it. Tool activity and results appear in the pane. **Stop** cancels inference and
invalidates commands that have not been dispatched. Completed edits remain in the document and can
be undone; an uncertain command outcome is never replayed automatically.

A turn captures its document, window, selection and file grants. Switching documents or windows,
closing the document, or changing the selection stops edits for that turn. A changed document
revision must be read again before editing. The assistant's own selection commands remain usable.
An explicitly authorized Open or a built-in document creation may continue in that document after
reading it, preserving the previously open writing. Listing open documents gives metadata, not
permission to edit another document. Assistant tool schemas target that bound document without a document-ID parameter. External MCP
clients retain explicit document targeting; the assistant still rejects foreign document IDs if
a model sends one despite the restricted schema.

The preview distinguishes saved page fill, dark-page display and interface appearance.
Visual inspection exposes only those settings and the registry-backed commands needed to change
them; it does not grant access to document content. Successful background setters return the
actual post-change state with their result, avoiding a separate verification request. Explicit
setter values and existing target/selection/revision guards still apply.
Correct display/interface changes leave writing and its normal Undo history unchanged. Sol and
Terra passed all three background-source checks. The existing Qwen model passed only two of
seven warm cases and made a wrong document-fill edit in the interface case; model narration is
not edit proof. Built-in document creation/listing and feature metadata retain their host guards. Page-image rendering, screenshots and simulated mouse/keyboard input remain
external-client capabilities; full MCP parity is not established.

For file operations, choose individual files in the pane or include an exact absolute path in your
prompt. Quote paths containing spaces, for example `Save to "/Users/me/Documents/New report.docx"`.
An unquoted path must be a complete token separated by whitespace; use quotes when punctuation
follows a path. Grants cover individual files, including indirect reads such as linked HTML images;
each linked file or image needs its own grant. Grants do not cover the containing folder.
Enter a full path and select **Allow path** for a new output file. An existing open file still needs a grant to be opened by
the assistant. File grants are captured for each turn and are not restored from chat history.

**New chat**, saved-chat selection and **Delete chat** manage local conversations independently of
document files. Interrupted conversations reopen as interrupted. Reopening a chat never resumes
inference or document edits. Chats can contain document text and tool results; delete them from the
pane when they are no longer needed. **Reload settings** retries a failed settings load; **Stop**
also cancels settings operations and sign-in.

## Local preview on AK's Mini

The preview is installed at `/Applications/WordCraft Assistant Preview.app`.
Version `0.12.0 (9c6c2cb0)` includes upstream 0.12.0, one model selector, direct MCP tools,
app-session compatibility proofs, bundled background inspection/verification, the View button,
safe context-limit diagnostics and disabled standard updates. Native View/settings/updater smoke
passed with synthetic content. The installed Terra handbook edit also passed in **11.477s**, with
visible result, saved/reopened content/formatting and one normal Undo verified.
Open **View → Assistant**, then configure the model under **File → Options → Agents**.
Codex `gpt-6.1-terra` is the applied WordCraft-only default after matching observed correctness
and lower measured medians than Sol. Those small samples do not guarantee every turn is faster.
The main `/Applications/WordCraft.app` (0.10) and its MCP wrappers are unchanged. Existing legacy
Control choices and chats are preserved but no longer route requests. Ollama retains its configured
loopback endpoint; this task added no reboot autostart. See [current benchmarks](assistant-benchmarks.md)
for Sol, Terra, Qwen and external-client results.

The following measurements describe earlier two-role previews, not the installed single-model
workflow; they are retained as historical evidence.

On the same synthetic edit/read-back/Undo task, Codex control passed in about 30s and Gemma control
passed in about 4m16s. Qwen's original identical-prompt run stopped after 46s without editing because
it guessed an invalid document ID. A separate diagnostic with explicit current-document guidance
completed in about 67s after Codex corrected another refused Qwen write. Its final edit and normal
Undo passed, but Qwen also falsely reported success during the failed intermediate attempt.
After the tool-contract repair, the original unchanged prompt passed in about 61s with four
successful MCP calls, zero errors, exact read-back and one normal Undo. Codex delegated twice
(edit, then verification); no extra user guidance was needed. These observations do not establish
a reliability rate, and the earlier guided run is not the same-prompt speed benchmark. Inspect
actual tool results and the document rather than relying on a model's
summary. The full evidence and exact model identity are in the implementation ledger.

**Preview update policy:** newly packaged previews pass `--disable-updates`; the standard
updater cannot replace them. Install a newer preview DMG into `/Applications`, preserving the
separate main WordCraft app. Earlier previews may still offer official updates: choose **Later**.
See [update maintenance and future service separation](assistant-updates.md).

This preview uses an ARM64 release build with local ad-hoc signing, an English interface and the
existing bundled/system-font defaults, without the optional shared translation/font build inputs.
It is not notarized for distribution to other Macs. Its own document preferences, logs and recovery
use `/Users/andy/Library/Application Support/WordCraft Assistant Preview`, with recovery enabled.
Assistant connections and conversations still use the normal shared Craft configuration below.
The portable launcher finds Codex on PATH, including standard Homebrew locations and `~/.local/bin`;
`CRAFT_CODEX_BINARY` can override it. Craft sign-in and compatibility gates are still required.
No credentials or model choices are copied. The Apple silicon DMG, ZIP, checksums and install guide are
in `dist/assistant-preview/9c6c2cb0/`; the DMG and notes are also in Downloads. See the ledger
for exact files and verification. Mounted and extracted DMG apps, installed native hash and
rollback copy were verified. DMG SHA-256:
`cea0796333f9ca009ea685a9c58a1a15184dd47546a05216b36ab8b248f288c2`. Share the DMG and checksum privately and drag the preview
to `/Applications` on the receiving Mac. Each Mac needs its own Codex/Ollama setup. This preview is ad-hoc signed and not notarized;
the install guide links Apple's per-app approval instructions.

## Verify a model route

The pane shows one effective provider/model. Expand the individual MCP actions
to inspect reads, edits and returned results. Check the actual document, saved/reopened content
and normal Undo; a model's success message alone does not verify an edit.

For independent local execution evidence, check the Ollama server request log and `ollama ps`
while the turn runs. The expected loaded model, successful MCP results and exact document
read-back together verify that route. Saved chat files preserve tool calls/results but do not
currently persist model provenance; capture effective settings and live status with each test.
Current synthetic routing and performance evidence is in the benchmarks. The existing Qwen
model is not a reliable substitute for the tested Codex models on background requests.

## Shared configuration

The configuration root is `CRAFT_AI_CONFIG_DIR`, if set, otherwise the platform configuration
directory followed by `Craft/ai` (on macOS, `~/Library/Application Support/Craft/ai`).

| File or location | Contents |
| --- | --- |
| `connections.json` | Shared connections, model capabilities, suite role bindings and per-app overrides; credential references only |
| `runtime.json` | Validated request timeouts, context/response limits and tool-round limits |
| `chats/wordcraft/` | WordCraft conversations; each Craft app has a separate namespace |
| Platform credential store | API credentials; never embedded in documents or transcript files |
| `codex/` under the shared root | Craft account state and restricted runtime configuration; child HOME, USERPROFILE and CODEX_HOME all use this isolated home |

The preview uses the existing `assistant.chat` binding as its canonical model
choice. Legacy `assistant.control` bindings, credentials and conversations are preserved but do
not route WordCraft requests. An unset or invalid chat binding never falls back to that legacy
control choice. A WordCraft override takes precedence over the suite default; an invalid override
fails explicitly. Runtime defaults are defined once in
`craft-ai-core::assistant::RuntimeConfig` and documented in the shared `craft-ai-native` README.
Changes to connections and runtime settings are locked and atomically saved. Never put API keys
in either JSON file.

Runtime limits are configured in `runtime.json`, for example
`{"request_timeout_secs":180,"max_rounds":12}`; omitted fields retain the documented defaults.
An oversized conversation fails explicitly at `max_context_bytes`. Start a new chat or increase
that limit; history is not silently truncated or summarized.

## Automation and portability

The existing external MCP/control connections keep working. The assistant uses an internal request
channel to the same MCP server and executes permitted commands on the UI thread, through existing
protection and Undo handling. No additional TCP listener is started.

The UI commands are `ui.assistant {value?}`, `ui.assistant.send {text, files?}`,
`ui.assistant.stop {}` and `ui.assistant.status {}`. Use
`ui.assistant.newChat {}` using the pane's normal New chat action and busy guards; it starts an
asynchronous operation and returns `{"started":true}`. Wait for a changed opaque `chatId`,
`settingsBusy:false` and `busy:false` before sending to the new chat. Status reports canonical
`model`, busy state and errors without credentials or chat contents. Legacy `chatModel` and
`controlModel` fields alias the same selected route. Sending requires configured desktop services.


Connections, provider adapters, routing, persistence and widgets live in `craft-libs`. WordCraft's
adapter supplies app identity, the opaque target and permitted MCP handling. Other Craft apps can
implement that adapter without importing WordCraft's document model. GridCraft, PdfCraft and other
app integrations are deferred, as is browser inference.

## Revision history

| Date | Change |
| --- | --- |
| 2026-10-29 | Install verified portable 9c6c2cb0 preview with one model, proof reuse and background verification; apply WordCraft-only Terra default after measured acceptance; retain Qwen/parity/platform limits |
| 2026-10-29 | Document pending single-model routing, app-session compatibility proofs and background metadata/verification bundling; installed revision and live acceptance unchanged |
| 2026-10-28 | Record installed ARM64 preview, separate document profile, native launch checks and standard-release updater caveat |
| 2026-10-28 | Document settings, two-model routing, local history, document/file boundaries and current release gates |

## Provider request errors

HTTP failures name the selected connection/model and provide status-specific next steps.
HTTP 400 means the provider rejected the request; it does not by itself identify the rejected
field. Check the model's streaming/tool support and conversation context limit. Connection tests
only check the catalog; use the separate synthetic tool-support check to test model capability.
Provider error bodies remain hidden because they may echo private prompts or credentials.
For Ollama's known context-overflow error, WordCraft displays only the validated token counts
and guidance. A new chat can exceed a small context after reading the document. Ollama's
OpenAI-compatible API cannot change this setting; configure a larger context in Ollama
(for example, `PARAMETER num_ctx` in a model's Modelfile). See
[Ollama's context configuration](https://github.com/ollama/ollama/blob/main/docs/api/openai-compatibility.mdx#setting-the-local-context-size).
No automatic retry or provider substitution occurs; completed edits can still be undone.
