# Desktop assistant

Status: desktop implementation verified by automated gates, 2026-10-28. Local-model MCP editing and Undo passed;
Codex, hosted-provider and other-platform live acceptance remain open; evidence is tracked in
[the implementation ledger](assistant-implementation.md). This feature has not been released.

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
history, settings or MCP registrations. The installed Codex executable must pass compatibility and
containment checks before document context is supplied. Failures remain visible and do not switch
providers. Set `CRAFT_CODEX_BINARY` to an absolute executable path when Codex is not on the
desktop app's PATH. Sign-in and discovery alone do not confirm readiness: a synthetic callback
and result-continuation check must succeed for the selected model before document context is sent.
The full live Codex acceptance gate is still open; see the ledger.

Ask for document work in the same conversation as general chat. Requested edits apply directly and
use normal **Undo**. Relevant document text is read when the task needs it and may be sent to **both**
configured providers. Tool activity and results appear in the pane. **Stop** cancels inference and
invalidates commands that have not been dispatched. Completed edits remain in the document and can
be undone; an uncertain command outcome is never replayed automatically.

A turn captures its document, window, selection and file grants. Switching documents or windows,
closing the document, or changing the selection stops edits for that turn. A changed document
revision must be read again before editing. The assistant's own selection commands remain usable.
An explicitly authorized Open may continue in the opened document after reading it.

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
| 2026-10-28 | Document settings, two-model routing, local history, document/file boundaries and current release gates |
