# Native Codex capabilities and DeckCraft panel

Status: In progress. Updated: 2026-11-04.

The requested behavior is the normal Codex skills/plugins environment presented through the
Craft side panel, alongside app document tools. Existing plugin permissions and approvals must
remain effective; the panel must handle requests rather than silently approve or discard them.

| Workstream | Owner | State | Acceptance | Dependencies / release |
| --- | --- | --- | --- | --- |
| Shared Codex runtime | Shared runtime agent | Deployed | Normal profile/capability discovery; skills/plugins/MCP usable; permission and approval UI; document tools, cancellation and Undo retained | Protocol 1 compatible; signed helper 0.1.6 active on both Macs |
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

## Normal-profile rollout correction

Helper 0.1.5 briefly passed signed installation/handshakes on both Macs, but the Mini's actual
WordCraft model turn failed before editing. The adapter forced `--strict-config`, which rejected
an existing account skills field that normal Codex startup accepts. Both devices were explicitly
returned to signature-verified 0.1.4; 0.1.5 and all prior data remain retained. No user process
was killed. The earlier isolated-profile fixture did not expose this normal-profile defect.

Shared fix `1ebb61ac04923c0cd79e11514eddebd38a99793c` is committed/pushed as helper 0.1.6.
It uses normal Codex config parsing, with a regression on the exact startup command; native/helper
tests and strict scoped Clippy/fmt passed. The signed candidate passed all four host handshakes in
a temporary installation before activation. Mini actual WordCraft preview `dbacf0c4` then passed
with normal Codex HOME/config and `gpt-6.1-terra` in 24.27 s: visible requested edit, Undo/Redo,
save/reopen and screenshot inspection. The synthetic listener and key were removed normally.
Mini activation follows that acceptance; MacBook candidate qualification remains in progress.

Candidate source `1ebb61a`, font input `37827adc41fd165f9a9234ac40f94dbad353aa69`;
archive SHA-256 `75085a4ea473e4bea1bcd1a4644a2f92664fd87b469e314766b6ebf8a7486a42`;
executable SHA-256 `224527703fd06f01cfec8a7de637ecedd55209263ff21e48fa91a652caa110e1`.
Local signature is verified; no notarization/public release is claimed. Receipts and isolated QA
are under `/private/tmp/craft-assistant-016-20261104/`.

## Two-Mac helper acceptance completed

Both devices now resolve helper 0.1.6, retaining 0.1.4 for rollback. MacBook preactivation
qualification passed independently in 10.11 s with the same configured Terra model and actual
WordCraft `dbacf0c4` document edit/Undo/Redo/save/reopen checks. Both screenshots were inspected:
requested text visible, compact composer present, tool rows hidden. Installer checks confirmed
unchanged production configuration and all existing app binaries, verified signatures/hashes,
and successful PDF/Word/Grid/Deck handshakes. No existing user window was terminated.

The exact signed 0.1.6 native MCP fixture passed in 16.94 s. Earlier skill and cancellation
checks remain recorded above; the final startup flag change adds a targeted command regression.
WordCraft's required `cargo xtask ci` passed all nine steps with the exact pinned craft-i18n
input and the reviewed font checkout: fmt, Clippy, tests, i18n report, language, doctests, assets,
layers and all 15 WASM crates. Log: `/private/tmp/wordcraft-native-ledger-ci.log`.
Deck's full eight-step CI also passed; final source pin, live model acceptance and preview
packaging remain separate. Native VoiceOver/IME/clipboard and every connected plugin have not
been exhaustively tested. Craft chats stay app-owned; external native edits are outside app Undo.
