# WordCraft architecture

> **Last reviewed:** 2026-10-28 · **Last updated:** 2026-11-02 · **Change:** trivial (craft-proof's row says what a build without the Language data keeps: English spelling from an installed Hunspell English or the built-in list, rule hyphenation for Bulgarian, Turkish and Russian; previously minor: proofing moved out: `wordcraft-proof` is craft-libs' `craft-proof`, its Language data craft-i18n's `language-data/` embedded from `CRAFT_I18N_DIR`; previously major: crate and layer inventory, source counts, proofing languages and module boundaries re-measured from this checkout; previously minor: `proof` is per language: English and Polish; previously major: first committed version; `plan/architecture.md` is local-only) · **Target:** Microsoft Word (Microsoft 365) for Mac 16.113.4
> **Assistant integration:** WordCraft 0.15 source reconciliation is tracked in [the integration ledger](upstream-assistant-v015.md); the embedded helper and complete MCP parity are retained. Installed previews are unchanged.

How WordCraft is built today. The longer design notes in
`plan/architecture.md` are gitignored and local to the original author's machine; this file is the
committed, current summary. Rules for working in the code are in [`AGENTS.md`](../AGENTS.md).

## Layers

On 2026-10-28, `find crates apps xtask -type f -name '*.rs' -print0 | xargs -0 wc -l` counted
333,448 Rust source lines across 16 crates, 3 apps and `xtask`. `Cargo.toml` declares workspace
members with `crates/*`, `apps/*` and `xtask`; `cargo xtask layers` verified 20 workspace packages
(16 crates, 3 apps and `xtask`) and no layering violations. The layer and line counts below are
from that command and per-directory `find ... -name '*.rs' ... | wc -l` counts on that date. Each
crate name links to its source tree.

| Layer | Crate | Lines | Job |
|---|---|---|---|
| L0 standalone | [`control-key`](../crates/control-key/src) | 87 | Per-window control-port key files shared by the app and MCP bridge |
| L0 | [`geom`](../crates/geom/src) | 1,271 | Units (points, twips, EMU, inches, cm), rectangles and measurement parsing |
| L1 | [`doc`](../crates/doc/src) | 23,719 | Document model: stories, paragraphs, runs, tables, sections, styles, numbering, fields, notes, comments, revisions and equations; edit primitives and bidi |
| L1 | [`fonts`](../crates/fonts/src) | 7,088 | Font database, metrics, outlines, HarfRust shaping, Word font substitution and OpenType MATH |
| L1 | [`metafile`](../crates/metafile/src) | 3,614 | Shared Windows Metafile parsing and drawing primitives used by format readers and renderers |
| L1 (external) | [`craft-proof`](https://github.com/storytold/craft-libs/tree/main/crates/proof) (package alias `wordcraft-proof`) | — | Proofing moved to craft-libs (CP1, 2026-11): fifteen spelling dictionaries: English (US, GB, CA), Polish, Portuguese, Italian, Croatian, Turkish, Russian, Czech, Indonesian and Spanish (Spain, Mexico, Latin America, Uruguay); language modules provide spelling, suggestions and grammar, while Bulgarian and German have hyphenation only. Its word lists and patterns are craft-i18n's `language-data/`, embedded from `CRAFT_I18N_DIR` at the commit in `craft-i18n.rev` (CI and releases set `CRAFT_I18N_REQUIRED=1`); without them English spelling falls back to an installed Hunspell English or craft-proof's built-in Moby list, and pattern hyphenation is off (Bulgarian, Turkish and Russian hyphenate by rule) |
| L2 | [`docbin`](../crates/docbin/src) | 8,815 | Word 97–2003 `.doc` reader (FIB, piece table, FKPs, SPRMs, stylesheet, lists, tables, notes, pictures) over a CFB container |
| L2 | [`docx`](../crates/docx/src) | 37,376 | OOXML WordprocessingML reader/writer (Transitional and Strict), OMML, custom properties and preserved VBA parts |
| L2 | [`formats`](../crates/formats/src) | 23,058 | TXT, Markdown, HTML, RTF, ODT and LaTeX import/export through the shared document model |
| L2 | [`layout`](../crates/layout/src) | 35,990 | Paragraph line layout (`para`), page/section flow and table layout, then display-list generation (`display`); hit testing maps points to document positions. `para` owns shaping, breaks, tabs and line geometry; page flow composes paragraphs, tables, headers/footers, notes and floats. |
| L3 | [`pdf`](../crates/pdf/src) | 4,513 | krilla PDF export: font subsets, selectable text, links, outline and tagging |
| L3 | [`render`](../crates/render/src) | 4,499 | vello_cpu rasterizer: pages and thumbnails from the display list; PNG/JPEG |
| L4 | [`engine`](../crates/engine/src) | 72,851 | `Session` and editing state; command registry in `cmd/`; file dispatch in `io`; catalog, keymap, recovery, ribbon model, speech and view state |
| L5 | [`mcp`](../crates/mcp/src) | 1,822 | MCP server exposing commands as tools; headless or bridged to the running app |
| L5 | [`zotero`](../crates/zotero/src) | 2,372 | Zotero word-processor integration protocol answered against a `Session` |
| L6 | [`ui-egui`](../crates/ui-egui/src) | 81,759 | Front end: ribbon, keytips, mini-toolbar, Backstage, canvas, rulers, panes, dialogs, i18n, themes, control channel and headless `ui_shot` example |
| apps | [`wordcraft`](../apps/wordcraft/src) | 3,740 | Desktop app (eframe + wgpu; logging, window geometry and file associations) |
| apps | [`wordcraft-cli`](../apps/wordcraft-cli/src) | 1,901 | `convert`, `text`, `inspect`, `run`, `commands`, `parity`, `mcp` and `zotero` |
| apps | [`wordcraft-web`](../apps/wordcraft-web/src) | 1,159 | Browser build (eframe web runner, WebGPU with WebGL2 fallback) |
| tooling | [`xtask`](../xtask/src) | 1,488 | Workspace maintenance and quality-gate commands, including `layers` |

## Data flow

1. **Open:** `engine::io` picks a reader by extension (`docx`, `docbin`, `formats`, JSON) and
   produces a `doc::Document`.
2. **Edit:** every user-visible action is a command id run through `Session::run` (ribbon,
   shortcuts, command search, CLI, control channel and MCP all dispatch by id). `Session::run`
   snapshots for undo (blocks are `Arc`-shared, so snapshots are cheap) and catches an escaped
   panic, restoring the document (never-crash standard).
3. **Layout:** `layout` turns the document into pages of positioned glyphs, boxes and images
   (display list), incrementally: an edit relayouts only what changed (1.4 ms on a 188-page
   document, measured 2026-10-06).
4. **Draw:** the UI paints pages through `render` (CPU raster, cached per page, pixel-aligned) on
   an egui/wgpu canvas; PDF export walks the same display list through `pdf`.
5. **Save:** `engine::io` picks a writer by extension.

## Testing and gates

- 821 `#[test]` functions and 10 property-test blocks: model, layout, DOCX round-trip and hostile
  input, `.doc`, formats, typing parity (`engine/src/tests/typing.rs`), bidi, i18n catalogs, UI
  interaction (egui_kittest), MCP acceptance; `hostile_params_never_panic` runs every command with
  junk params.
- `cargo xtask ci`: fmt, clippy `-D warnings`, tests, asset attribution, layering, wasm build.
- `cargo xtask parity` regenerates [`parity-checklist.md`](parity-checklist.md); a test keeps the
  catalog percentage above a floor that only rises.
- GitHub Actions on PRs: packaging lint, and FreeBSD / Windows ARM64 when build files change;
  release builds on pushes to `release`.

## Portable desktop assistant

The desktop assistant uses `craft-ai-core` contracts, `craft-ai-native` execution/storage and
`craft-ui::assistant` state/action widgets from craft-libs. Only the WordCraft host adapter knows
the engine: it sends internal requests through `wordcraft_mcp::Server`, dispatches on the UI thread,
checks the originating window/document/selection and revision, and confines exact file access at
the engine I/O boundary. The assistant catalog omits external MCP's document selector because the
turn already owns its target; supplied foreign IDs still fail the dispatch guard. External MCP
retains its document selectors. Provider workers have no WordCraft
document dependency. In the installed single-model preview, the selected `assistant.chat` choice
handles conversation and MCP tools directly; legacy Control bindings remain stored but do not
route WordCraft turns. An app-wide Codex template retains only successful compatibility proofs
in memory. Each turn forks fresh pending exchanges and authentication-verification state, checks
the account before private context, and closes its exchanges on exit. Executable/model/runtime
changes invalidate proofs; settings reload, sign-in and configuration-root changes clear the
template. Document/window changes retain it. HTTP providers and standalone probes keep separate
lifetimes. Craft remains the canonical history owner. Native services are excluded on WASM.
Background tool metadata and authoritative post-setter verification are bundled without granting
visual inspection access to writing. Source gates, Sol/Terra synthetic document acceptance and external comparisons passed; the
existing Qwen model failed five of seven warm cases. Native exposes 13/17 external tools; full
parity, hosted-provider and other-Mac acceptance remain open. Installed preview is `9c6c2cb0`. See [assistant help](assistant.md) and
[the implementation ledger](assistant-implementation.md) for configuration, portability and gates.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-11-02 | trivial | craft-proof's row: without the Language data English spelling falls back to an installed Hunspell English or the built-in Moby list, and only pattern hyphenation is off (#5202 review) |
| 2026-11-01 | minor | `proof` left the workspace: it is `craft-proof` in storytold/craft-libs (with its history), and the spelling and hyphenation data is craft-i18n's `language-data/` (`CRAFT_I18N_DIR`); `assets/spelling` and `assets/hyphenation` are gone |
| 2026-10-28 | major | Recounted workspace crates, layers and Rust source lines; refreshed proofing languages and documented current engine/layout module boundaries and source links |
| 2026-10-29 | minor | Record installed 9c6c2cb0 single-model preview, measured acceptance and remaining local-model/tool/platform limits. |
| 2026-10-29 | minor | Record pending single-model routing, isolated turn forks with app-session compatibility proofs, and bundled background metadata/verification; live acceptance pending. |
| 2026-10-28 | minor | Specialize assistant tools for the bound document; preserve external MCP selectors and dispatch guards. |
| 2026-10-28 | minor | Native Codex callback continuation and mixed Codex/Ollama routing verified through installed WordCraft; shared runtime remains app-independent. |
| 2026-10-28 | minor | Codex returns actual tool results to bounded ephemeral exchanges while Craft keeps canonical history; provider lifetime is scoped to each turn or standalone probe. |
| 2026-10-28 | minor | Add portable assistant architecture and guarded internal MCP dispatch; live acceptance remains open. |
| 2026-10-10 | minor | `proof`: proofing languages (English, Polish), affix engine, Polish patterns; layout and engine route each run by its language |
| 2026-10-10 | major | First committed architecture doc, from the crates' sources and `AGENTS.md` |
