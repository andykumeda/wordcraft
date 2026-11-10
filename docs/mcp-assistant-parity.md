# Assistant and external MCP parity

> **0.17 reconciliation in progress:** The shared assistant refuses new turns after multiple native windows have appeared until extra windows are closed and WordCraft is restarted. Assistant calls with no explicit document id are refused if the active document changed during the turn. Explicit calls to other documents and discovery remain available; after a window switch, even a call naming the active document requires a new turn because its selection belongs to that window. Assistant visual input is temporarily unavailable because queued actions are not bound to a window; page rendering and window-bound screenshots remain available. The full source CI gate passed; signed-package and installed-app verification remain pending. Older statements below describe the previous implementation.

Status: implementation verified by focused automated tests, 2026-11-02. Evidence is recorded below;
no new installed-app or live-provider acceptance is implied.

The side panel is an MCP client of the running WordCraft app. It obtains `tools/list` and
executes `tools/call` through the same `wordcraft_mcp::Server` used by external clients. All
17 tools retain their public names, descriptions and input schemas. This includes
`render_page`, `screenshot`, `click` and `key`; `execute` and `list_commands` expose the full
supported command surface. Tool content blocks, image bytes, error flags and any structured
result fields returned by the server remain intact.

The comparison is with an external MCP client connected to that same GUI backend. A headless
MCP server still has its existing backend limitations for window-only operations. Provider
availability, image understanding and configured assistant budgets remain separate model/runtime
constraints; they do not change the app's MCP catalog.

## One authority for application behavior

The assistant queues live control requests to the UI thread. That thread uses the same
`control::handle` dispatch and asynchronous screenshot admission/completion path as the external
control channel. No GUI document mutation happens on the provider worker.

The old assistant-only command allowlist, rewritten tool schemas, current-document binding,
selection/revision lock, mandatory read-before-edit policy and exact-file grants are removed.
The public MCP `doc` argument can address other open documents, and ordinary commands can switch
or create documents and change app settings, just as they can for external clients.

The existing app and engine remain authoritative: protected documents, Undo, dirty-close
confirmation, command validation and file-access roots keep their ordinary behavior. In
particular, the external CLI's automation roots apply to its headless backend; `mcp --connect`
already refuses those flags rather than claiming to confine a running GUI. The side panel does
not replace existing process/thread roots with a different filesystem scope.

Cancellation and expiration still prevent queued requests from starting. Lost acknowledgments
and timeouts stop the turn; actions whose outcome is uncertain are not replayed. A turn's
`Target` is correlation metadata for the private helper protocol and does not rewrite MCP tool
arguments or restrict which open document the backend can address.

## Verification

Added parity regressions compare the complete catalog and representative calls against an
external server using the same live backend. They cover page rendering, asynchronous PNG
screenshot results, synthetic click/key input, full command discovery, app settings, addressed
documents, protection, Undo, unsaved-close refusal, filesystem roots, malformed tools,
cancellation and lost acknowledgments.

Verified against shared Git revision `c28f430ee4e932e7cbfa9e149b92f24e0f0bc202` with locked, offline builds:

- `cargo test --locked --offline -p wordcraft-ui-egui --lib assistant -j2`: 33 passed,
  two explicitly ignored (manual image capture and configured live-provider acceptance).
  This includes both remote reentrancy regressions for nested Begin/reload and self-Stop
  while a GUI request owns the temporarily removed turn.
- `cargo test --locked --offline -p wordcraft-ui-egui --lib control -j2`: 26 passed.
- `cargo test --locked --offline -p wordcraft-mcp -j2`: 30 passed (24 server tests and6 transport/integration tests).
- `cargo clippy --locked --offline -p wordcraft-ui-egui -p wordcraft-mcp --all-targets -j2 -- -D warnings`: passed.
- Full repository formatting and diff checks passed.

The final static check required extracting the existing pending-dispatch loop from `poll` to meet the repository complexity limit. Cancellation and nested-turn guards are unchanged; the focused remote regression rerun passed3/3 after that extraction.

The first image parity run exposed an incomplete synthetic screenshot fixture: admission
correctly refreshes geometry from the raw-input surface. The fixture now calls the production
raw-input hook before screenshot admission; rendering and screenshot results match both clients.

Native installed-app and live-provider acceptance remain pending for this increment. Earlier
document-scope restrictions and their historical tests in the assistant implementation ledger
are superseded by this explicit MCP-parity requirement.

## Upstream 0.15 source reconciliation

The independently verified 0.15 integration and final shared Git pin are recorded in
[the integration ledger](upstream-assistant-v015.md). Its assistant/control/updater and MCP
regressions pass; the older results above describe the pre-reconciliation source.
