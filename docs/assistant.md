# Desktop assistant

Status: desktop implementation verified by automated gates, 2026-10-28. Codex and mixed Codex-chat/Ollama-control MCP editing
and Undo passed on the Mini; hosted-provider, adversarial containment and other-platform acceptance remain open; evidence is tracked in
[the implementation ledger](assistant-implementation.md). A separate local preview is installed
and its native launch verified; this feature has not been publicly released.

Open **Home → Assistant**, or find **Assistant** in command search. The pane can dock, float and
resize like the other task panes. **File → Options → Agents** contains its connection and model
settings. Browser inference is not available.

Choose two models independently: **General chat** answers ordinary prompts and delegates document
work; **App control** reads and edits through WordCraft's MCP server. Fresh settings leave both
choices unset. Configure both roles with models that support streaming and tools before sending.
The conversation shows which connection and model is being used. A missing connection, unavailable
model or unsupported capability is an error; WordCraft never chooses another provider for you.

Add an OpenAI-compatible connection using the OpenAI, OpenRouter, Ollama, LM Studio or custom
endpoint preset. The endpoint is the API base, including its version path. Discover models, or enter
a model ID manually. Automatic delegation requires tool support: use the capability check or
explicitly confirm that a manually selected model supports tools. **Verify tool support** sends
synthetic prompts, which can incur normal provider usage charges.
**Test connection** only requests the provider's model catalog. Save the connection, choose a
connection and model for each role, then select **Apply model**. API keys go into the platform credential
store, separate from settings and chats. If that store is unavailable, saving the key fails visibly.

Codex uses a separate Craft-managed sign-in. It does not reuse your coding assistant's credentials,
history, settings or MCP registrations. Under **Codex account**, choose **Sign in to Codex**, then
enter the one-time code shown in WordCraft on the browser page. Keep WordCraft open until it shows
**Codex connection saved**, then choose and apply a model for each role. Signing into the browser
account alone does not complete the device connection.

The installed Codex executable must pass compatibility and
containment checks before document context is supplied. Failures remain visible and do not switch
providers. Set `CRAFT_CODEX_BINARY` to an absolute executable path when Codex is not on the
desktop app's PATH. Sign-in and discovery alone do not confirm readiness: a synthetic callback
and result-continuation check must succeed for the selected model before document context is sent.
The installed Codex basic edit/read-back/Undo test passed; broader adversarial containment remains a release gate. See the ledger.

Ask for document work in the same conversation as general chat. Requested edits apply directly and
use normal **Undo**. Relevant document text is read when the task needs it and may be sent to **both**
configured providers. Tool activity and results appear in the pane. **Stop** cancels inference and
invalidates commands that have not been dispatched. Completed edits remain in the document and can
be undone; an uncertain command outcome is never replayed automatically.

A turn captures its document, window, selection and file grants. Switching documents or windows,
closing the document, or changing the selection stops edits for that turn. A changed document
revision must be read again before editing. The assistant's own selection commands remain usable.
An explicitly authorized Open may continue in the opened document after reading it.
Assistant tool schemas target that bound document without a document-ID parameter. External MCP
clients retain explicit document targeting; the assistant still rejects foreign document IDs if
a model sends one despite the restricted schema.

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

The test build is installed at `/Users/andy/Applications/WordCraft Assistant Preview.app`.
Version `0.11.0 (ca279f8c)` includes corrected Codex continuation, compact history controls and
document-scoped tools. This build passed Codex-chat/Qwen-control editing, read-back and normal Undo.
Open **Home → Assistant**, then configure models under
**File → Options → Agents**. The existing `/Applications/WordCraft.app` and its MCP wrappers are
preserved. The current WordCraft-only choices are Codex `gpt-6.1-sol` for General chat and
Ollama `qwen3:4b-instruct-2507-q4_K_M` for App control, selected for the user-requested trial.
Ollama uses the configured loopback endpoint. This task added no reboot autostart.

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

**Choose Later on update prompts while testing.** The existing updater offers the standard
WordCraft release. Updating this custom preview bundle has not been verified.

This preview uses an ARM64 release build with local ad-hoc signing, an English interface and the
existing bundled/system-font defaults, without the optional shared translation/font build inputs.
It is not notarized for distribution to other Macs. Its own document preferences, logs and recovery
use `/Users/andy/Library/Application Support/WordCraft Assistant Preview`, with recovery enabled.
Assistant connections and conversations still use the normal shared Craft configuration below.
The portable launcher finds Codex on PATH, including standard Homebrew locations and `~/.local/bin`;
`CRAFT_CODEX_BINARY` can override it. Craft sign-in and compatibility gates are still required.
No credentials or model choices are copied. The Apple silicon ZIP, checksum and install guide are
in `dist/assistant-preview/ca279f8c/`; see the ledger for the exact files and verification.
Share the ZIP and checksum privately, extract it and place the preview in `~/Applications` on the
receiving Mac. Each Mac needs its own Codex/Ollama setup. This preview is ad-hoc signed and not notarized;
the install guide links Apple's per-app approval instructions.

## Verify a model route

The pane shows the effective **General chat** and **App control** models. During a document request,
its status changes to the active provider/model. Expand `control_app` and the individual MCP actions
to inspect delegation, reads, edits and returned results. Check the actual document, then use normal
Undo to confirm the edit belongs to its undo history.

Model labels describe the selected route. For independent local execution evidence, also check the
Ollama server request log and `ollama ps` while the turn runs. A successful request after the handoff,
the expected loaded model, the completed MCP results and exact document read-back together verify
that route. Saved chat files preserve tool calls/results but do not currently persist model provenance;
capture the effective settings and live status alongside a test transcript. Relevant document context
can still reach Codex when General chat uses Codex and App control uses Ollama.

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

The roles are `assistant.chat` and `assistant.control`. A WordCraft override takes precedence over
the suite default; an invalid override fails explicitly. Runtime defaults are defined once in
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
`ui.assistant.stop {}` and `ui.assistant.status {}`. Status reports routing, busy state and errors without
returning credentials or chat contents. Sending a prompt requires configured desktop services.

Connections, provider adapters, routing, persistence and widgets live in `craft-libs`. WordCraft's
adapter supplies app identity, the opaque target and permitted MCP handling. Other Craft apps can
implement that adapter without importing WordCraft's document model. GridCraft, PdfCraft and other
app integrations are deferred, as is browser inference.

## Revision history

| Date | Change |
| --- | --- |
| 2026-10-28 | Record installed ARM64 preview, separate document profile, native launch checks and standard-release updater caveat |
| 2026-10-28 | Document settings, two-model routing, local history, document/file boundaries and current release gates |
