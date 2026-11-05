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

## Implementation and acceptance checkpoint

- Shared source `eb0abbf` (helper 0.1.5) and font compatibility `0ff4f97` are committed/pushed.
  Combined UI/native/helper/fonts tests, strict scoped lint and UI WASM passed. This is a candidate,
  not an installed update. The normal account/config/skills/plugins environment is reused;
  approvals are explicit helper UI requests with headless pending/respond commands.
- Live synthetic candidate checks passed on the Mini: explicit native skill (10.9 s), native
  stdio MCP call (10.78 s), Stop cleanup (18.52 s), and abrupt helper-death/EOF cleanup
  (27.68 s). Fake document mutations were reversible in the fixture. These are not Deck engine
  Undo proofs; Deck has its own tests. No existing app process was signalled.
- The first cancellation fixture used an inherited working directory outside its declared
  temporary permissions and was rejected before execution. Correcting its CWD made it pass.
- A further rollout gate remains: inherited Finder CWD may be `/`, so it must not become a
  default writable Codex workspace. The shared follow-up uses an explicit validated
  `CRAFT_CODEX_WORKSPACE` override or a retained private workspace under the Craft AI root,
  while keeping the normal Codex profile and its policies. No installation until verified.
- Deck's original missing-glyph test passes with pinned fonts and the original deep-group test
  passes in the normal build profile; the stack overflow occurred only under a temporary
  unoptimized profile. Both were classified before source integration.
- Deck's initial implementation compiles. Nine focused assistant/preview tests, desktop preview
  startup tests and the narrow layering test pass. The full 25-tool catalog is retained.
  Dirty raw quit and pathless native picker bypasses found in independent review are guarded
  and tested alongside dirty close/revert, stale target/selection/revision, and cancellation.
- Root-owned separate preview packager is prepared; packaging/source builds, live Deck model
  acceptance, final full CI and deployment remain pending.

Temporary test fixture and receipts: `/private/tmp/craft-native-helper-acceptance.py` and
`/private/tmp/craft-assistant-015-20261104/`. Test authentication used a temporary link to the
existing account file, removed after each test; no credentials or personal chats were copied.
