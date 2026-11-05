# Craft suite — two-Mac update

Status: Deployed except blocked MacBook standard WordCraft. Updated: 2026-11-04 (America/Los_Angeles).

The owner requested the latest apps on Mini and MacBook. Scope is the installed Craft suite;
a clarification about other installed apps is pending. Standard official releases and custom
assistant previews are separate workstreams. Preserve all documents, profiles, credentials,
conversations, helper routing and rollback copies. Never replace a running app or force it to quit.

| Component | Target | Mini | MacBook | Owner / verification |
| --- | --- | --- | --- | --- |
| ArtCraft Launcher | 0.14.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| DeckCraft standard | 0.15.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| FigCraft | 0.15.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| LightCraft | 0.18.1 | Deployed | Deployed | Primary; official artifact/checksum, signature, version/startup |
| WordCraft standard | 0.15.2 | Deployed | Blocked | Primary; MacBook app open (PID88575), owner asked to save/quit |
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

## Official installation checkpoint

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
Word, Grid and Deck preview reconciliations, full gates, signed packages and actual configured Terra edit/Undo/save/reopen qualification passed on both hosts. PDF full CI (6586 passing tests,16 ignored), direct dependency-policy check and signed-package validation pass; Mini signed-candidate/installed actual Terra checks pass (23.49s/28.65s), and Book signed-candidate/installed checks pass (23.82s/20.21s). MacBook standard WordCraft still has PID88575 open and is never force-closed.

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

Only blocked item: `/Applications/WordCraft.app` on MacBook remains0.15.0, PID88575 open.
Owner has been asked to save/quit. The verified official0.15.2 archive and guarded installer are
already staged at `/private/tmp/craft-suite-rollout-20261104/` on Book. After the app has closed,
run `python3 /private/tmp/craft-suite-rollout-20261104/install-official.py wordcraft` on Book,
then verify its report/version/signature and update this row. Do not substitute AppleEvent
termination, force quit, or replacing the running bundle for the owner's normal save/quit.
No other installed/non-Craft application scope was confirmed. Existing MCP processes and their
absolute registrations remain unchanged; current companion CLIs are available via `.local/bin`.
