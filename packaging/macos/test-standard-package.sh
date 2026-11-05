#!/usr/bin/env bash
# Check --standard admission and optionally an extracted signed standard app.
set -euo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
fixture="$(mktemp -d "${TMPDIR:-/tmp}/wordcraft-standard-check.XXXXXX")"
trap 'rm -rf "$fixture"' EXIT
if CRAFT_UPDATE_REPOSITORY= CRAFT_ASSISTANT_BRIDGE_PROTOCOL= bash "$HERE/assistant-preview.sh" --standard >"$fixture/error" 2>&1; then
  echo 'error: standard packaging accepted missing compatibility metadata' >&2; exit 1
fi
rg -q -- '--standard requires CRAFT_UPDATE_REPOSITORY, CRAFT_ASSISTANT_BRIDGE_PROTOCOL and CRAFT_UPSTREAM_TAG/SHA' "$fixture/error"
if [[ $# = 1 ]]; then
  python3 - "$1" "$HERE" "$fixture" <<'CHECK'
import os, pathlib, plistlib, subprocess, sys
app = pathlib.Path(sys.argv[1])
info = plistlib.loads((app / 'Contents/Info.plist').read_bytes())
assert app.name == 'WordCraft.app'
assert info['CFBundleIdentifier'] == 'ai.storyteller.wordcraft'
assert info['CFBundleName'] == info['CFBundleDisplayName'] == 'WordCraft'
assert type(info['CraftAssistantBridgeProtocol']) is int and info['CraftAssistantBridgeProtocol'] > 0
assert '/' in info['CraftUpdateRepository']
executable = app / 'Contents/MacOS' / info['CFBundleExecutable']
launcher = executable.read_text()
assert '--disable-updates' in launcher
assert 'PROFILE_DIR' not in launcher and 'Assistant Preview' not in launcher
native = executable.parent / 'WordCraft-native'
assert native.exists()
assert info['CraftUnderlyingTag'] == 'v' + info['WordCraftVersion']
assert len(info['CraftUnderlyingCommit']) == 40
assert all(c in '0123456789abcdef' for c in info['CraftUnderlyingCommit'])
subprocess.run(['codesign', '--verify', '--strict', '--deep', str(app)], check=True)
version = subprocess.check_output([str(executable), '--version'], text=True).strip()
assert info['WordCraftVersion'] in version
assert info['WordCraftBuildCommit'][:8] in version
environment = dict(os.environ, CRAFT_UPDATE_REPOSITORY=info['CraftUpdateRepository'],
                   CRAFT_ASSISTANT_BRIDGE_PROTOCOL=str(info['CraftAssistantBridgeProtocol']),
                   CRAFT_UPSTREAM_TAG=info['CraftUnderlyingTag'], CRAFT_UPSTREAM_SHA=info['CraftUnderlyingCommit'])
command = ['bash', str(pathlib.Path(sys.argv[2]) / 'assistant-preview.sh'), '--standard',
           '--binary', str(native), '--version', info['WordCraftVersion'],
           '--build-sha', info['WordCraftBuildCommit'], '--output-dir', sys.argv[3]]
for key, value in [('CRAFT_UPSTREAM_TAG', 'v0.0.0'), ('CRAFT_UPSTREAM_SHA', '0' * 40)]:
    result = subprocess.run(command, env=dict(environment, **{key: value}), capture_output=True, text=True)
    assert result.returncode != 0 and 'underlying tag/commit must match' in result.stderr
CHECK
fi
echo 'standard package: required compatibility metadata and supplied bundle checks passed'
