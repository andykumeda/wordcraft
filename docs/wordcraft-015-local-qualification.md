# WordCraft 0.15 local candidate and installed qualification

Current compact-layout candidate and its installation boundary are recorded in the [November 3 follow-up](#compact-layout-candidate--2026-11-03). The older installation evidence below remains historical.

Status: signed local candidates verified; both installed variants and the matching versioned
CLI verified on the Mini and MacBook, 2026-11-03 UTC. No public release was created.

Code source: `8762d9869fe92fd61a2894b0085b4f458b5ca13b`, shared libraries
`986cb16234cfdac1ee1337fb4cdf00945cad4e5a`. Version **0.15.0**, Apple silicon arm64,
minimum macOS **11.0**. The optimized release build used the exact required inputs:
craft-fonts `b076417b5880bd4154bd22f00695b1d28c6bea83` and craft-i18n
`059cbf95c2a3eb95d637ca8584faca1129d17f48`.

## Candidate artifacts

Directory: `/Users/andy/Dev/wordcraft/dist/assistant-candidates/0.15.0-8762d986/`.
`candidate-manifest.json`, individual checksum files and `SHA256SUMS.txt` accompany the outputs.
Both bundles and the CLI are signed by the explicitly approved local certificate
`D7B1B5524FFEB37AB01D3960017BDF6743E3EDFD`; they are **not notarized**.

| Artifact / signed executable | SHA-256 |
|---|---|
| `wordcraft-cli` | `f135170a85972a4892ae0df759f4efcbf8a184b28dbe70c4ea257f0de02bd4cf` |
| `WordCraft.app/Contents/MacOS/WordCraft` | `65db39964764166b22ab806801c73f31383bc3bd1b7ecaeece925bae26fc6553` |
| `WordCraft Assistant Preview.app/Contents/MacOS/WordCraft-native` | `0f3c79baa9b7655d834153d659cdea8a67fe1510f41dc7d7854f9808773cf2a1` |
| `wordcraft-0.15.0-macos-aarch64.dmg` | `bbe797385e449c89642de5b7b6a7a2e0cfc51bf4a777e5589db34e6258ff9520` |
| `wordcraft-assistant-preview-0.15.0-8762d986-macos-aarch64.zip` | `e43afd3964ab185b20892282e5054e190bbe76946dd70338c978bad8c4537a21` |
| `wordcraft-assistant-preview-0.15.0-8762d986-macos-aarch64.dmg` | `8802d1c06a6c45be4cca547052868582527b86d2853063c92e012351fccd6c1e` |
| `wordcraft-cli-0.15.0-macos-aarch64.zip` | `cd0a5a110eb070c7ad38b95eadfc21c3d46c2c126ddab3e928d21de719fe2e4c` |

Both bundles passed strict signature checks in staging, ZIP extraction where applicable,
read-only DMG mounts and copied bundles. Exact certificate-leaf and bundle-identifier
requirements passed on the final copies. Both installed variants and the versioned CLI on
both Macs matched these hashes and exact signing requirements.

The standard bundle is `ai.storyteller.wordcraft`; the preview remains
`ai.storyteller.wordcraft.assistant-preview`. Both signed plists contain integer
`CraftAssistantBridgeProtocol=1` and `CraftUpdateRepository=andykumeda/wordcraft`.
The preview launcher still uses its separate profile and `--disable-updates`. The matching
external MCP binary remains versioned at `~/.local/share/wordcraft/0.15.0/wordcraft-cli`;
these verification runs changed no wrapper or client configuration.

Required MIT/Apache/copyright/attribution notices, embedded Latin and pinned shared-font
licenses, and pinned translation/proofing attribution and license texts were compared byte
for byte with their sources. Upstream trademark artwork is omitted; original copyright and
trademark terms remain included.

## Installed smoke on both devices

Each run opened a separate **background** installed-app instance with an owner-only temporary
document profile, preferences disabled and an empty temporary AI store. Only public installed
helper configuration was copied. No personal document, conversation or provider configuration
was read/copied and no model inference was requested. The installed helper reported **0.1.0**,
with the panel available and no component error; the empty store correctly had no model chosen.

All four runs passed: 17-tool MCP catalog, synthetic edit/read, ordinary Undo/Redo, Inter font
selection, save/close/reopen from disk and native page-render PNG. The Mini's rendered PNG was
also inspected. Both variants' results were 1,793-byte synthetic renders.

| Device / variant | Owned PID / loopback port | Report directory |
|---|---|---|
| Mini / standard | 3366 / 17981 | `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/wordcraft-installed-smoke-xenau7q7` |
| Mini / preview | 7113 / 17981 | `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/wordcraft-installed-smoke-2k4xlv6k` |
| MacBook / standard | 48630 / 17981 | `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-installed-smoke-6mtcko2w` |
| MacBook / preview | 48744 / 17981 | `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-installed-smoke-ulpd3bg1` |

Actual listener ownership and fresh owner-only control keys were checked before use. Every
owned instance quit normally, released its listener and removed its key. No unknown process
was stopped, no proxy/startup entry was added and no installed app/helper/CLI or deployment
configuration was changed by packaging or these smoke runs. The primary delivery workstream
performed installation and backups separately. Both devices' services inventories record the
stopped tests; those inventories are outside Git and cannot be committed here.

## Scope of qualification

These installed checks cover MCP document operations and the real helper's load/readiness.
They do not establish configured-model inference correctness, native VoiceOver/text-composition,
clipboard or streaming acceptance, or an automated published-release self-update. See
[the source integration ledger](upstream-assistant-v015.md) for automated source evidence and
remaining gates. Public release remains a separate delivery step.

## Compact-layout candidate — 2026-11-03

State: **Verified candidate**, ready for the primary two-Mac installation workstream. These
packaging checks performed no installed-app/CLI replacement or user process termination.

Code source `dbacf0c4b5c9571ad1c03df83efb926895ee7444`; host libraries
`83e61084dee3117d34f95600d6ec68bad8b0b9b5`. Independently installed helper UI revisions
(including 0.1.2) remain protocol/settings/history 1 compatible with this host. Actual binary
`--version` is `wordcraft 0.15.0 (dbacf0c4)`; architecture arm64, minimum macOS 11.0.
Required font/proofing inputs match the pinned revisions recorded above and are clean checkouts.

Candidate directory: `dist/assistant-candidates/0.15.0-dbacf0c4/`. Its `candidate-manifest.json`
contains the stable extracted preview path, exact certificate/identifier results and archive
hashes. Standard app staging remains in the task-owned optimized target's `macos-package`.

| Artifact / signed executable | SHA-256 |
| --- | --- |
| Standard `WordCraft` native executable | `62c471ed4ee8da52a918a6a94fe88f2d7135557f8e3d95ef19fbd80110796d8b` |
| Preview `WordCraft-native` executable | `2ca6180e268a003ccd638bb086f00718255b8fd5aa6cf0cb56f9b17cb9e6651b` |
| `wordcraft-0.15.0-macos-aarch64.dmg` | `0f4a36de566e47475d58881b8298d1c81b6d033b734dd541f003fcc5fdd89db7` |
| `wordcraft-assistant-preview-0.15.0-dbacf0c4-macos-aarch64.dmg` | `ca85a350c90f09f81b9b58e60cf8d217ff5a7b0f6e2f474626158075c51304a2` |
| `wordcraft-assistant-preview-0.15.0-dbacf0c4-macos-aarch64.zip` | `6d1d465d7da9bc37bee18a9e7ff621dc3d737a64be74d55eb65e747a3dd15782` |

Both supported packagers passed strict staging, extracted ZIP (preview), mounted DMG and
copied-app signature checks. Independent verification also passed exact certificate leaf
`D7B1B5524FFEB37AB01D3960017BDF6743E3EDFD` plus each bundle identifier, compiled/plist source,
protocol/repository metadata and byte-for-byte embedded Latin/shared-font/proofing notices.
Verification used macOS signature-validation access; a constrained invocation could not access
that service and was not treated as evidence of valid signatures. No trust check was weakened.

Notarization credentials were explicitly cleared; packages are locally signed, not notarized
or publicly released. Installed CLI remains unchanged; any unused CLI archive generated by
the standard packager is excluded from this app rollout. Exact full CI passed all nine stages,
including the deterministic unsupported-language fixture correction documented in the source ledger.
Final installed-app smoke and preservation of any running user preview are the primary workstream's
remaining acceptance steps; these candidate results do not claim deployment or live inference.

## Compact-layout installed acceptance checkpoint — 2026-11-03

Mini **standard and preview: Deployed**. Primary installed source dbacf0c4 atomically and
verified both candidates' signatures/native hashes. Two independent isolated native runs
passed with helper **0.1.2**: settings-gear update placement, all **17** MCP tools listed,
synthetic edit/read/Undo/Redo, save/close/reopen and a **1,793-byte** page PNG. Both runs quit
their test-owned instances normally and released the listener and control key. The preview
panel screenshot was visually inspected: New chat/history and gear header, compact bottom
file/model controls and no main-chat version/update toolbar. Empty isolated model settings
are not a configured-provider acceptance claim. No inference or personal settings were used.

MacBook **standard and preview: Deployed**. After the user saved/closed Preview and restored access, both signed dbacf0c4 bundles were atomically installed. Exact certificate/identifier, source stamp and native SHA-256 match the candidates above. Previous bundles are retained under `~/Applications/Craft Rollout Backups/2026-11-03-compact/`; existing CLI and user settings/history were preserved.

Independent standard and Preview native runs passed helper0.1.2, settings-gear update placement,17 MCP tools, synthetic edit/read/Undo/Redo, save/close/reopen and1793-byte PNG rendering. Each test instance quit normally and released its control listener/key. Standard receipt root: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-installed-smoke-1hmcgwgf`; Preview: `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-installed-smoke-rr3j8hfg`. The Preview main-chat screenshot was visually inspected.

The initial MacBook UI test failed to find an exact OCR row; the temporary test now accepts the same explicit navigation labels within a row, including combined text. Successful reruns passed. No product code changed. A dropped connection was resolved by the user waking/unlocking the MacBook; successful reruns used a temporary sleep hold only for the test duration. No inference or personal settings were used. Live provider, IME, clipboard, VoiceOver and public-feed activation remain unverified; packages are locally signed, not notarized.
