# Craft suite — two-Mac update

Status: Single-app consolidation **Deployed on both Macs**, 2026-11-05. This supersedes the historical
split official/Assistant Preview installation policy below. The owner requires one normal app
with its assistant panel on each Mac, and ArtCraft Launcher updates must retain the integration.
Do not install ordinary upstream binaries over these customized apps.

| App | Official base | Normal assistant source | Mini / MacBook | Active Preview |
| --- | --- | --- | --- | --- |
| WordCraft | 0.15.2 | fccdfe38 | Deployed / Deployed | Removed both |
| GridCraft | 0.16.1 | f5785668 | Deployed / Deployed | Removed both |
| DeckCraft | 0.15.1 | dfa02f39 | Deployed / Deployed | Removed both |
| PdfCraft | 0.18.2 | 5181ac82 | Deployed / Deployed | Removed both |

Managed ArtCraft Launcher0.14.1/source1917013 and shared helper0.1.7/sourcef2295dd are deployed
and independently checked on both Macs. Normal Word/Grid/Deck installs used the installed
launcher's actual protected update path. Their live model candidate checks and installed
no-model light/dark panel checks pass; normal profiles/recovery/shared AI data stay unchanged.
The normal PdfCraft replacement passed actual configured-model edit/no-early-save/Undo/Redo/
save/reopen/render independently on both Macs, with all five bundled OCR resources retained
exactly. Helper0.1.7 bottom message-box outlines passed light/dark idle/focus checks inside PDF
on both hosts. All four active Preview apps and legacy duplicate shortcuts are retired. See the [current central ledger](https://github.com/andykumeda/pdfcraft/blob/codex/independent-assistant/docs/single-app-assistant-rollout.md).

Use **Check for updates** in the managed ArtCraft Launcher. App details retain the official
underlying version and show **Updates preserve Assistant** with the configured source.
Save/quit the app before updating. Update Mini first, then MacBook: Book obtains the signed
compatible artifact from Mini over its existing SSH connection. Source conflicts, unavailable
builder or failed validation leave the installed app intact; there is no upstream-only fallback.
The launcher protects its own integration from replacement by an ordinary official self-update.
No public release or notarization is claimed. Rollback app bundles/installers are retained as
non-launchable ZIPs under each host's `~/Library/Application Support/Craft/retired-apps/`.

## Historical split rollout (before single-app consolidation)


Status: Deployed on both Macs. Updated: 2026-11-05 (America/Los_Angeles).

The owner requested the latest apps on Mini and MacBook. Scope is the installed Craft suite, confirmed by the owner on2026-11-05. Standard official releases and custom
assistant previews are separate workstreams. Preserve all documents, profiles, credentials,
conversations, helper routing and rollback copies. Never replace a running app or force it to quit.

| Component | Target | Mini | MacBook | Owner / verification |
| --- | --- | --- | --- | --- |
| ArtCraft Launcher | 0.14.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| DeckCraft standard | 0.15.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| FigCraft | 0.15.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| LightCraft | 0.18.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| WordCraft standard | 0.15.2 | Deployed | Deployed | Primary; Book owner saved/quit; signature/hash/version and native GUI/MCP edit/Undo/save/reopen/render passed |
| PdfCraft standard | 0.18.2 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| GridCraft standard | 0.16.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| WordCraft Assistant Preview | 0.15.2 + custom changes | Deployed | Deployed | Word owner; release reconciliation, CI, signed package, native Terra edit/Undo/save/reopen |
| PdfCraft Assistant Preview | 0.18.2 + custom changes | Deployed | Deployed | Primary; release reconciliation, CI, signed package, native Terra edit/Undo/save/reopen |
| GridCraft Assistant Preview | 0.16.1 + custom changes | Deployed | Deployed | Grid owner; release reconciliation, CI, signed package, native Terra edit/Undo/save/reopen |
| DeckCraft Assistant Preview | 0.15.1 + custom changes | Deployed | Deployed | Deck owner; release reconciliation, CI, signed package, native Terra edit/Undo/save/reopen |
| Shared assistant helper | 0.1.6 | Verified | Verified | Previously deployed/qualified; recheck active version while preserving configuration |

Latest releases resolved from official `storytold` GitHub release metadata, not from Mini's installed
versions. Launcher's repository is `storytold/craft-launcher`. Source tags and full SHA-256 digests
are recorded in `/private/tmp/craft-suite-rollout-20261104/manifest.json`; downloads are checked
against both GitHub asset digests and the published SHA256SUMS. A successful download alone does
not establish installation or live behavior.

MacBook identity verified `AKs-MBPro.local`, andy, arm64, macOS27.3. SSH works; Desktop Commander
reports stale offline status, so SSH is the available fallback. Data volume has about15GiB free;
process archives sequentially and retain app rollback copies without unrelated cleanup.

Existing standard apps with custom assistant additions will be preserved as rollback bundles;
current custom behavior remains in the separately updated Assistant Preview apps. No public
release, upstream-main push, new remote or account/configuration copying is authorized by this
ledger. Per-app owners keep source/verification/install records synchronized in their repositories.

## Official installation checkpoint — 2026-11-04 (historical)

All seven standard apps are installed at their latest resolved versions on Mini. MacBook has
six updated; its running WordCraft is retained pending the owner saving/quitting. All six
companion CLIs are installed under versioned `.local/share` paths with `.local/bin` aliases;
old binaries and running MCP servers remain untouched. Existing MCP registrations have not been
rewritten or restarted. Original app bundles are retained under the per-host
`~/Library/Application Support/Craft/app-update-backups/2026-11-04/` directories.

Official mounted Deck/Fig/Grid bundles contain unsealed Finder metadata that strict signature
verification refuses. Only `com.apple.FinderInfo`/`com.apple.ResourceFork` attributes are removed
from the disposable staging copy; the original code signature then verifies. No re-signing or
quarantine removal is performed. Archive hashes, exact tag-source metadata, arm64 support and
installed binary hashes/version commands pass. Standard Mini Word additionally passed an actual
authenticated GUI/MCP synthetic edit/read/Undo/Redo/save/reopen/render check; its owned listener
and key were released. App configuration and user documents were not copied or changed.

Temporary official reports: `/private/tmp/craft-suite-rollout-20261104/reports/` on each host.
Word, Grid and Deck preview reconciliations, full gates, signed packages and actual configured Terra edit/Undo/save/reopen qualification passed on both hosts. PDF full CI (6586 passing tests,16 ignored), direct dependency-policy check and signed-package validation pass; Mini signed-candidate/installed actual Terra checks pass (23.49s/28.65s), and Book signed-candidate/installed checks pass (23.82s/20.21s). At the2026-11-04 checkpoint, MacBook standard WordCraft had PID88575 open; the2026-11-05 completion below supersedes that blocker.

## Assistant preview delivery evidence

| Preview | Source | Native SHA-256 | DMG SHA-256 | Actual candidate Terra latency Mini / Book |
| --- | --- | --- | --- | --- |
| Word0.15.2 | `fccdfe385a07e70f51ab71f9544e0be08f7a0484` | `e956b241a39c106f9c3b8ce32bf229df0587134f0741d24c7814d4ccb1b969aa` | `92f75302526e5c6d5b7e2a968d6f6e164f64003b4643c23e3865323f2f6d3f24` | 29.21s / 14.15s |
| Grid0.16.1 | `2b995d6d16303a2c49a2094c12d39cddc1b52265` | `c319cb1db89ebd37a5617284c8af4b25c898675ea8183e54a3eb4a5bed05fe7f` | `8b2f378906214359834873a11b90c284de089ba9d3cf774bf72053f242ebc1ca` | 16.29s / 14.31s |
| Deck0.15.1 | `dfa02f392524c9b23c1d8cc04616efdb24d7adbf` | `7046140977f312199b60784fa1372606d11656b97dc85c2b6df31df0c056889c` | `9098ddd5a7e995ecb0aa21a817a8a79d982c952173fb0cb4bcf1ecd93aba0c47` | 18.54s / 15.59s |
| PDF0.18.2 | `32b9ca14cbd5cb3609b29978f66c2d1dfe8ab40b` | `94d2b2f7fdf27919ff822d4c647ad117705773a446a03d85728a70e09013f1e4` | `31dac5f15399838e7584d3ab062eb2819980b7655b60a16dbd124e28294498ac` | 23.49s / 23.82s |

Each uses the normal Codex profile and shared helper0.1.6. Synthetic documents verified requested visible edits, disk unchanged until explicit save, Undo/Redo, independent reopen and render. Grid used fresh single-document processes for saved/restored-file rendering after its document revision guard correctly refused a second-document screenshot. All owned fixtures and listeners closed. Profiles, standard bundles and active helper pointers were preserved. Preview launchers disable the normal updater; local signed builds are not notarized or public releases. Model-backed native external actions can change files outside application Undo.

Source-of-truth preview records: Word `docs/upstream-assistant-v015.md`, Grid `../gridcraft/docs/assistant-v0161-rollout.md`, Deck `../deckcraft/docs/assistant-implementation.md`, PDF `../pdfcraft/docs/assistant-v0182-rollout.md`. Deck has only read access to its upstream remote, so its source/doc commits remain local; no new fork was created. VoiceOver/IME/clipboard checks were not repeated in this rollout.

Coordinator completion gate: all nine Word repository CI steps passed with pinned inputs (`/private/tmp/craft-suite-word-final-ci-unsandboxed.log`). An initial sandboxed run denied loopback binds in control-server tests; rerunning with required socket access passed without source changes. Latest official release tags were independently rechecked and all seven still match targets (`/private/tmp/craft-suite-rollout-20261104/latest-recheck.json`).


## Final state

Independent final inventories `/private/tmp/craft-suite-rollout-20261104/final-{mini,book}.json`
match full source commits, versions and native hashes for all four previews and six official
standard apps on both Macs; Mini official Word0.15.2 also matches its release. Helper active
version is0.1.6 on both. Every preview has a version/commit DMG containing a signed app, checksum and instructions
in each Downloads directory. Final PDF installed native confirmation additionally passes on both
hosts with unchanged standard app/config/helper hashes. Owned test processes exit normally and
loopback listeners release; no user process is stopped.

### MacBook Word completion — 2026-11-05

Owner confirmed Craft-suite scope and saved/quit WordCraft. Verified AKs-MBPro.local/andy/arm64
and absence of running standard WordCraft before installation. The unchanged guarded installer
(`305405fd51bc89c5d787054573d37d0003ece2dab2b653ef75f6e15535403f87`) installed official0.15.2
source `edfea68101220c9bdc4b2b0ccf4acd9612bedb6e` from the previously verified archive
SHA-256 `1ce120e09aa842fe24a82f0f0023d850416bc1f0bef6e4848dbfcb69d60ddadf`.
Strict original signature, universal arm64/x86_64, version and installed native hash
`c9e2cc8d5217162b8866e4923cfa39bf7cd23a70b8a3bd9c865e431749c3c0cc` passed.
Rollback retained intact: `/Users/andy/Library/Application Support/Craft/app-update-backups/2026-11-04/wordcraft/before-0.15.0-adf9fce8.app`.

Independent native GUI/MCP smoke passed on a disposable profile/document:17tools, exact typed
text readback, Undo/Redo, explicit save/close/reopen and PNG rendering. No inference requested.
Owned PID89460 quit normally through the control channel; listener17981 released and private
key removed. Receipt: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-installed-smoke-uuf3qua7/report.json`
on Book; copied report/render under `/private/tmp/craft-suite-rollout-20261104/book-word-completion-evidence/` on Mini.
No owner document, assistant profile or conversation was changed; no user process force-quit.

Latest official releases were rechecked2026-11-05: all seven target tags still current.
Fresh complete inventories match exactly for seven standard apps, four Assistant Previews and
helper0.1.6: `/private/tmp/craft-suite-rollout-20261104/final-{mini,book}-20261105.json`.
There are no remaining rollout blockers. Existing MCP processes and their
absolute registrations remain unchanged; current companion CLIs are available via `.local/bin`.

Completion gates rerun2026-11-05: all nine `cargo xtask ci` steps passed with pinned font/language inputs, including5202 native tests (9 ignored) and15 WASM crates. Log `/private/tmp/craft-suite-word-completion-20261105-ci.log`. The copied Book synthetic render was inspected and its text matches the requested fixture. Native test PID89460 is confirmed exited.
