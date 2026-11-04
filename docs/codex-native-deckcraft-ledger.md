# Native Codex capabilities and DeckCraft panel

Status: In progress. Updated: 2026-11-04.

The requested behavior is the normal Codex skills/plugins environment presented through the
Craft side panel, alongside app document tools. Existing plugin permissions and approvals must
remain effective; the panel must handle requests rather than silently approve or discard them.

| Workstream | Owner | State | Acceptance | Dependencies / release |
| --- | --- | --- | --- | --- |
| Shared Codex runtime | Shared runtime agent | In progress | Normal profile/capability discovery; skills/plugins/MCP usable; permission and approval UI; document tools, cancellation and Undo retained | Define compatible helper/host contract; no installation claimed |
| DeckCraft host | DeckCraft agent | In progress | Embedded shared panel and settings, full existing MCP tools, command/control access, synthetic deck edit/Undo/save/reopen, real UI and WASM checks | Clean upstream cloned to `/Users/andy/Dev/deckcraft`; branch integration and shared contract required |
| Runtime and device acceptance | Primary | In progress | Installed runtime schema and capability probes; per-device model/tool/approval checks; signed package and rollback verification | Existing helper 0.1.4 remains deployed until a verified replacement is ready |

The shared runtime and DeckCraft host own separate mutable repositories. Shared protocol/API
changes must be agreed before DeckCraft depends on them. App document edits must continue through
engine tools so unsaved state, Undo and serialization remain under the owning app.

DeckCraft has no existing personal fork. No upstream-main push or public release is authorized
by this ledger. Sharing/delivery decisions will use the concrete verified result.

## Evidence

- DeckCraft cloned from `https://github.com/storytold/deckcraft` on the Mini; clean initial tree.
- Installed Codex public app-server schema generated under `/private/tmp/craft-native-codex-schema`.
- Official app-server documentation includes skills discovery, installed connector snapshots,
  command/file/permission approvals, MCP elicitation and user input. The current adapter suppresses
  these features and rejects unfamiliar events; this requires runtime integration work.
- DeckCraft baseline CI is running with two build jobs before product edits.

## Discovery and contract checkpoint

Normal-profile read-only probes passed without starting inference turns: Mini 246 discovered
skills; MacBook 199. Both runtimes accept the plugin-discovery API. Probe records contain
counts only, not account details or skill contents.

Bridge protocol 1 and the current Controller host request enum will remain compatible.
Permission/approval UI belongs to the shared helper and uses its existing frame/input channel.
DeckCraft will reuse its existing MCP catalog/dispatcher through the in-process proxy, with
no additional network listener. Existing dirty-close and updater limitations must be handled
before delivery; the baseline engine/CLI gaps will remain explicitly documented.

The existing standard DeckCraft 0.15.0 app is present on the Mini. A custom preview must use
a separate bundle identity and retain its updater opt-out before installation.
