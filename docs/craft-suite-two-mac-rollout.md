# Craft suite — two-Mac update

Status: In progress. Updated: 2026-11-04 (America/Los_Angeles).

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
| WordCraft Assistant Preview | 0.15.2 + custom changes | In progress | In progress | Word owner; release reconciliation, CI, signed package, native Terra edit/Undo/save/reopen |
| PdfCraft Assistant Preview | 0.18.2 + custom changes | In progress | In progress | Primary; release reconciliation, CI, signed package, native Terra edit/Undo/save/reopen |
| GridCraft Assistant Preview | 0.16.1 + custom changes | In progress | In progress | Grid owner; release reconciliation, CI, signed package, native Terra edit/Undo/save/reopen |
| DeckCraft Assistant Preview | 0.15.1 + custom changes | In progress | In progress | Deck owner; release reconciliation, CI, signed package, native Terra edit/Undo/save/reopen |
| Shared assistant helper | 0.1.6 | Verified | Verified | Previously deployed/qualified; recheck active version while preserving configuration |

Latest releases resolved from official `storytold` GitHub release metadata, not from Mini's installed
versions. Launcher's repository is `storytold/craft-launcher`. Source tags and full SHA-256 digests
are recorded in `/private/tmp/craft-suite-rollout-20261104/manifest.json`; downloads are checked
against both GitHub asset digests and the published SHA256SUMS. A successful download alone does
not establish installation or live behavior.

MacBook identity verified `AKs-MBPro.local`, andy, arm64, macOS27.3. SSH works; Desktop Commander
reports stale offline status, so SSH is the available fallback. Data volume has about18GiB free;
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
Preview reconciliations and required native model qualification remain in progress.
