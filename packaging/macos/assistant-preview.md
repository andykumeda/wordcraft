# Assistant app packages

The owner requires a single normal **WordCraft.app** with the integrated Assistant
(2026-11-05). Use the existing script's `--standard` mode:

```sh
CRAFT_UPDATE_REPOSITORY=OWNER/wordcraft \
CRAFT_ASSISTANT_BRIDGE_PROTOCOL=1 \
CRAFT_UPSTREAM_TAG=vVERSION CRAFT_UPSTREAM_SHA=FULL_OFFICIAL_COMMIT \
MACOS_SIGN_IDENTITY=SELECTED_CERTIFICATE \
packaging/macos/assistant-preview.sh --standard \
  --binary /path/to/verified/wordcraft --version VERSION \
  --build-sha FULL_CUSTOM_SOURCE_COMMIT --output-dir /path/to/output
```

This uses the normal bundle name/identity and existing standard document profile.
The wrapper preserves executable discovery and disables app self-updates; ArtCraft
Launcher owns compatible app updates. Shared assistant settings/chats and old
Preview profiles are not copied or changed. The script verifies the official
version tag and full commit exist in the supplied binary's custom source history
and signs this provenance alongside the assistant compatibility metadata.

Run `test-standard-package.sh /path/to/WordCraft.app` on the extracted result.
It checks standard identity, compatibility and underlying-source metadata, the
wrapper's lack of Preview profile overrides, strict signatures, version/source,
and rejection of mismatched underlying tags/commits. Full native acceptance on each
Mac and a launcher that preserves the assistant are separate delivery requirements.

## Historical Preview packaging (superseded for owner delivery)

`assistant-preview.sh` packages an explicitly supplied release binary without building,
installing or publishing it. The default is Apple silicon; `--arch x86_64` and
`--arch universal` require matching Mach-O slices. The minimum macOS version is read from
the binary. Version and full source commit are mandatory; the script runs the binary's
`--version` and checks its version and eight-character commit before writing outputs.
The full commit remains caller-supplied provenance. Package on a Mac that can execute that
architecture; this check does not establish source equivalence for an arbitrary binary.

```sh
packaging/macos/assistant-preview.sh \
  --binary /path/to/release/wordcraft \
  --version 0.11.0 \
  --build-sha FULL_40_CHARACTER_SOURCE_COMMIT \
  --output-dir /path/to/output
```

Outputs are version/commit/architecture-named ZIP and drag-to-Applications DMG files,
adjacent SHA-256 checksums and an installation README. The ZIP contains its own installation README and **WordCraft Assistant Preview.app**.
Existing output files are refused. Temporary staging is removed on exit. For binaries built
with optional fonts, supply the same `CRAFT_FONTS_DIR` so `copy_docs` includes their licences.

The bundle uses `ai.storyteller.wordcraft.assistant-preview`. Its shell launcher derives the
document profile from the destination user's `HOME`, respects `WORDCRAFT_PROFILE_DIR`, and
adds standard Homebrew paths and `~/.local/bin` for Codex discovery while preserving `CRAFT_CODEX_BINARY`.
There is no machine-specific `LSEnvironment`. The launcher executes the separately signed
`WordCraft-native` binary with `--disable-updates`. The supplied binary must advertise that
flag in `--help`; the packager refuses older binaries. Only that binary, original project assets, licences and generated
metadata are packaged: no settings, chats, credentials, Codex installation or Ollama models.

`MACOS_SIGN_IDENTITY` and optional `MACOS_KEYCHAIN` follow the main macOS packaging script.
The default is ad-hoc signing. This script does **not** notarize; a non-default identity is not
automatically a Developer ID identity. The installation README reports that limitation.
For public distribution with normal Gatekeeper trust, a valid Developer ID signature and
successful notarization remain separate delivery work. Do not treat local signature validation
as acceptance on another Mac. The standard WordCraft updater is not a preview update channel and is disabled by the
launcher. The main WordCraft app remains separate with its normal updater. New preview builds
are installed at `/Applications/WordCraft Assistant Preview.app`; existing profiles are retained.

The script signs inside-out and verifies both the assembled bundle and a fresh extraction of
the final archive. For a packaging smoke, use a temporary output directory and record the
actual binary source; an old installed binary is not evidence for a later repaired build.
Before delivery, independently check `--version` through the extracted launcher, architecture,
checksum, effective destination profile, and native acceptance using the intended release binary.

The packager verifies the mounted DMG app and a copy from the DMG, in addition to ZIP extraction,
before publishing outputs. It never installs the app itself. The owner's delivery workflow
installs the verified preview and retains a rollback copy. See [assistant updates](../../docs/assistant-updates.md).
