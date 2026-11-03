# WordCraft 0.15 local candidate and installed qualification

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
