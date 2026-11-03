#!/usr/bin/env bash
# Synthetic plist regression; never builds, signs, installs, or changes settings.
set -euo pipefail
. "$(dirname "${BASH_SOURCE[0]}")/../env.sh"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/wordcraft-metadata.XXXXXX")"
trap 'rm -rf "$fixture"' EXIT
for variant in standard preview; do
  cp "$ROOT/packaging/macos/Info.plist.in" "$fixture/$variant.plist"
  if [[ "$variant" = preview ]]; then
    plutil -replace CFBundleIdentifier -string ai.storyteller.wordcraft.assistant-preview "$fixture/$variant.plist"
  fi
  CRAFT_UPDATE_REPOSITORY=example/wordcraft CRAFT_ASSISTANT_BRIDGE_PROTOCOL=1 write_assistant_update_metadata "$fixture/$variant.plist"
  [[ "$(plutil -extract CraftUpdateRepository raw -o - "$fixture/$variant.plist")" = example/wordcraft ]]
  [[ "$(plutil -extract CraftAssistantBridgeProtocol raw -o - "$fixture/$variant.plist")" = 1 ]]
  plutil -lint "$fixture/$variant.plist"
done
cp "$ROOT/packaging/macos/Info.plist.in" "$fixture/ordinary.plist"
CRAFT_UPDATE_REPOSITORY= CRAFT_ASSISTANT_BRIDGE_PROTOCOL= write_assistant_update_metadata "$fixture/ordinary.plist"
if plutil -extract CraftUpdateRepository raw -o - "$fixture/ordinary.plist" >/dev/null 2>&1; then exit 1; fi
for pair in 'example/wordcraft|' '|1' 'https://github.com/example/wordcraft|1' '../wordcraft|1' 'example/wordcraft|0' 'example/wordcraft|4294967296' 'example/wordcraft|0001'; do
  before="$(sha256 "$fixture/ordinary.plist")"
  if CRAFT_UPDATE_REPOSITORY="${pair%|*}" CRAFT_ASSISTANT_BRIDGE_PROTOCOL="${pair#*|}" write_assistant_update_metadata "$fixture/ordinary.plist"; then
    echo "error: invalid metadata pair accepted" >&2; exit 1
  fi
  [[ "$(sha256 "$fixture/ordinary.plist")" = "$before" ]]
done
mkdir -p "$fixture/notices"
copy_docs "$fixture/notices"
for notice in LICENSE-MIT LICENSE-APACHE NOTICE ATTRIBUTION.md; do
  cmp "$ROOT/$notice" "$fixture/notices/$notice"
done
cmp "$ROOT/docs/brand/LICENSE-brand.txt" "$fixture/notices/docs/brand/LICENSE-brand.txt"
for license in "$ROOT"/assets/fonts/OFL-*.txt; do
  cmp "$license" "$fixture/notices/assets/fonts/$(basename "$license")"
done
if [[ -n "${CRAFT_FONTS_DIR:-}" ]]; then
  for license in "$CRAFT_FONTS_DIR"/fonts/*/OFL.txt; do
    cmp "$license" "$fixture/notices/OFL-$(basename "$(dirname "$license")").txt"
  done
fi
if [[ -n "${CRAFT_I18N_DIR:-}" ]]; then
  for source in "$CRAFT_I18N_DIR"/LICENSE-MIT "$CRAFT_I18N_DIR"/LICENSE-APACHE "$CRAFT_I18N_DIR"/ATTRIBUTION.md "$CRAFT_I18N_DIR"/language-data/manifest.toml "$CRAFT_I18N_DIR"/language-data/licences/*.txt; do
    cmp "$source" "$fixture/notices/build-inputs/craft-i18n/${source#"$CRAFT_I18N_DIR"/}"
  done
fi
python3 - "$fixture" <<'PYTEST'
import pathlib, plistlib, sys
root = pathlib.Path(sys.argv[1])
for name, identity in [('standard', 'ai.storyteller.wordcraft'), ('preview', 'ai.storyteller.wordcraft.assistant-preview')]:
    value = plistlib.loads((root / (name + '.plist')).read_bytes())
    assert value['CFBundleIdentifier'] == identity
    assert type(value['CraftAssistantBridgeProtocol']) is int
PYTEST
echo 'assistant metadata: both bundle variants, omitted metadata, and seven invalid pairs passed'
