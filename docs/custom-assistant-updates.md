# Integrated assistant app updates

Status: **Deployed for WordCraft on both Macs**, 2026-11-05 (America/Los_Angeles).
The active `/Applications/WordCraft.app` is 0.15.2/native `fccdfe38` with the integrated
Assistant and shared helper 0.1.7. Active Preview apps and obsolete MCP shortcuts are retired;
profiles, recovery data and shared AI data are preserved. Exact signed hashes/provenance,
actual-model candidate checks and final installed no-model evidence are in
[the current per-app ledger](upstream-assistant-v015.md#current-single-app-deployment--2026-11-05).
[The suite ledger](https://github.com/andykumeda/pdfcraft/blob/codex/independent-assistant/docs/single-app-assistant-rollout.md) owns overall progress; this app's deployment does not claim all four apps are complete.

## Current update route

Use the **managed ArtCraft Launcher 0.14.1/source19170137**, deployed on both Macs.
It checks official release freshness, reconciles the exact requested tag with the Assistant
source, runs checks/build/signing and verifies standard identity, exact certificate,
assistant protocol/repository and signed underlying-release metadata before replacement.
A failed build, unavailable build host or source conflict preserves the current app; there is
no stock-package fallback. The MacBook receives signed artifacts from the Mini, which must be
reachable for a new build. The trusted builder policy is configured in the user-owned
`~/Library/Application Support/ArtCraft Launcher/assistant-updates.json`.

The normal app wrapper disables its built-in updater. Preserve the managed launcher's own
self-update protection too; ordinary upstream app/launcher packages are not this delivery route.
The separately installed helper keeps its independent update lifecycle. Current source and
installed checks are already recorded; this documentation-only closeout does not rerun CI.

## Single normal app packaging — 2026-11-05

The owner has replaced the separate Preview delivery requirement with one normal
WordCraft.app containing the Assistant on both Macs. `packaging/macos/assistant-preview.sh --standard` packages a verified existing native under the normal name and bundle ID.
It preserves the existing standard document profile, leaves old Preview profiles intact,
and continues to use the shared Craft AI store for assistant settings/conversations.
The standard wrapper disables app self-updates; ArtCraft Launcher owns compatible app updates.
Packaging alone does not make an unchanged official launcher preserve the Assistant.

Required explicit inputs: `CRAFT_UPDATE_REPOSITORY`, `CRAFT_ASSISTANT_BRIDGE_PROTOCOL`,
`CRAFT_UPSTREAM_TAG` and `CRAFT_UPSTREAM_SHA`. The underlying tag must match the app
version, resolve to the supplied full commit, and be an ancestor of the binary's
verified source commit. The signed plist carries `CraftUnderlyingTag` and
`CraftUnderlyingCommit` alongside the assistant protocol/repository metadata.

Packaging verification: shell syntax, required-input rejection, metadata valid/invalid
cases, signed ZIP extraction, mounted DMG and copied-app strict signature checks.
`packaging/macos/test-standard-package.sh /path/to/WordCraft.app` checks standard
identity, updater-only wrapper, absence of Preview profile overrides, signed protocol
and underlying-source metadata, and binary version/source. The full `cargo xtask ci` pipeline passed all nine gates on 2026-11-05
(`/private/tmp/wordcraft-standard-ci.log`). Standard package source is existing
`fccdfe38`; the native app was reused rather than rebuilt. Signed native SHA-256:
`3b6d253ef92373dae725ae500e24fe4cc0415be9bfbbad2c28e4b98fc56a8328`.
The standard DMG SHA-256 is
`cd891f32fd9a57a44cfcebde32ea5c11ed02d4655113797a451c516383cb8dd4`.
The installed managed-launcher delivery and final normal-path checks are recorded above and in [the central suite ledger](https://github.com/andykumeda/pdfcraft/blob/codex/independent-assistant/docs/single-app-assistant-rollout.md).

### Standard candidate native acceptance — 2026-11-05

The signed standard candidate (native `3b6d253e`, source `fccdfe38`) passed independent
Mini and MacBook native acceptance using the existing configured Codex/Terra connection
and shared helper0.1.6: model latency24.62s Mini and13.19s Book. Exact synthetic edits,
no save until requested, Undo/Redo, explicit save/close/reopen and rendered edited PNGs
passed; screenshots were inspected. Owned processes quit normally and their listeners
and keys were cleaned up. Protected standard/Preview profiles and shared AI data were
unchanged. This is candidate qualification, not an installation claim.

Receipts: Mini `/var/folders/cg/796c5bys74db15ww0b_qvw8c0000gn/T/wordcraft-standard-native-fy6qwyin`;
Book `/var/folders/z6/7ct4dgns2236tw0d27zdb9s40000gn/T/wordcraft-standard-native-sw8ydspk`.
Consolidated local evidence: `/private/tmp/craft-single-app-evidence/word-{mini,book}`.
Installed acceptance and update delivery remain in [the central suite ledger](https://github.com/andykumeda/pdfcraft/blob/codex/independent-assistant/docs/single-app-assistant-rollout.md).

## Historical built-in updater contract (not the current owner route)

The source/configuration contract below is retained for developers. The owner's installed
macOS wrapper disables this built-in app updater; use the managed launcher described above.

### Configure the app source

Add this app to `sources.json` in the shared assistant settings directory (macOS:
`~/Library/Application Support/Craft/ai`; `CRAFT_AI_CONFIG_DIR` overrides that directory):

```json
{
  "schema_version": 1,
  "apps": {
    "wordcraft": {
      "repo": "YOUR_GITHUB_ACCOUNT/wordcraft",
      "macos_team_id": "YOURTEAMID",
      "bridge_protocol": 1
    }
  }
}
```

Replace the repository and team with the actual compatible release repository and its
10-character Apple Developer team identifier. The bridge protocol must match this build.
Configure the other apps as additional entries in the same file. Restart the app after editing
source configuration. Missing, malformed, unavailable or incompatible sources report an error;
they never select upstream or another publisher.

For an explicitly approved local-only signed distribution, the same entry may additionally
set `local_certificate_sha1` to the signing certificate's exact 40-character hexadecimal SHA-1
identity. This opts into that certificate instead of Apple's Developer ID/notarization trust
requirement; cryptographic signature verification, exact bundle/source metadata and package
checksum still apply. The certificate pin is the publisher identity in this explicit mode;
the existing team-ID field remains required by configuration validation. Omitting it keeps the ordinary Apple-trusted policy.
Unsigned and ad-hoc packages remain refused.

## Release contract

Customized macOS releases must preserve the installed app's `ai.storyteller.wordcraft` bundle
identifier and the standard package/asset names understood by `craft-update`. Separately named
older Assistant Preview bundles remain update-disabled until replaced by a compatible package.

Each stable GitHub release includes `craft-assistant-app.json`:

```json
{
  "schema_version": 1,
  "app_id": "wordcraft",
  "release_tag": "v1.2.3",
  "repository": "YOUR_GITHUB_ACCOUNT/wordcraft",
  "bridge_protocol": 1
}
```

The tag must equal the release tag. Before code signing, the app's `Contents/Info.plist` must
include `CraftAssistantBridgeProtocol` (integer) and `CraftUpdateRepository` (string) matching
that source policy. The package must publish a SHA-256 digest through GitHub's asset digest or
`SHA256SUMS.txt`. Installation verifies the checksum, configured Developer ID signature, bundle
identity, version and signed bridge/source metadata before replacing anything (or the explicitly
configured local certificate identity under that opt-in policy). A release-side
manifest alone cannot establish compatibility or publisher identity.

Updates retain the existing save-before-restart safeguard. No app update starts over unsaved
work or an active/pending assistant operation. The host also preserves assistant drafts and
settings edits. If a pending frame keeps an update deferred, finish or stop the chat, save its
settings and close the assistant panel before restarting. App updates are macOS-only for this first delivery; no unsigned/ad-hoc package or other
publisher is accepted. The assistant's own update remains independent of this flow.

## Delivery state

No source file has been written into a user's settings, and no public release or installed
app was changed by this update integration. The initial read-only check for
`andykumeda/wordcraft` returned HTTP 404 on 2026-11-02; the task fork was subsequently created
and source task branches pushed. A compatible signed release remains a separate delivery step;
do not configure an unverified release as an update source.

## Embedded component controls

The desktop entry point calls `enable_shared_assistant()` before enabling startup checks.
Once enabled, panel drawing, chat/settings operations and inference use the shared child process;
missing or incompatible components report errors rather than invoking the older in-process
assistant. The older adapter is retained for library compatibility and regression tests.
Document tools still execute through this app's existing guarded tool host on its UI thread.

Existing `ui.assistant.send`, `newChat`, `stop` and `status` commands retain their names.
Additional native control commands take an empty parameter object:

- `ui.assistant.updateCheck`: check the configured shared component release source.
- `ui.assistant.updateStatus`: report installed/offered component and update state.
- `ui.assistant.updateInstall`: explicitly approve the currently offered component update.
- `ui.assistant.reloadComponent`: reload configuration when no app-authorized turn is active.

These commands return a clear error when shared mode is not enabled. Checking or installing a
component never changes the app's release source or bypasses `--disable-updates` for app updates.

## Verification and remaining checks

The production UI library passed its scoped Cargo check against the local shared component
implementation. Shared updater regressions passed (175 tests, four environment/live tests
ignored); the additional incompatible-DMG preservation test passed separately. Shared proxy
regressions passed for wrong targets, repeated request IDs, cancellation, backpressure and
argument limits.

New app-level command/grant regression tests are included but have not been executed. Native
app linking, installed-app update replacement, real helper-to-document edits, VoiceOver and
cross-app binary-unchanged update demonstrations remain unverified. The development volume
ran below 1 GiB free, so further app builds/test linking were stopped to preserve existing
work. No published release or installed application was changed.

Shared implementation pin: `andykumeda/craft-libs@17c5ea725866fa2e8f7f89a6cb3bec269ad8d571` (task branch `codex/independent-assistant`). All shared dependencies use that revision together. Cargo locks must resolve the published Git source, never the temporary local verification patch.

The shared assistant/config/installer/updater final test run passed **228 tests, 4 ignored**. PDF production UI Clippy and Word/Grid production UI checks passed against byte-identical reviewed local source before publishing this pin. Native consumer test links were deferred for disk headroom; do not treat compilation as installed acceptance. No release or user configuration was changed.

## Signed package metadata

Both macOS packagers accept the paired explicit inputs `CRAFT_UPDATE_REPOSITORY=owner/repository`
and `CRAFT_ASSISTANT_BRIDGE_PROTOCOL=1`. When supplied, they write the source string and integer
protocol into `Info.plist` before signing. Both inputs are required together; neither has a
source default. Omitting both preserves ordinary package metadata. The chosen protocol must
match the built shared bridge.

`packaging/macos/test-assistant-metadata.sh` verifies both bundle variants, omitted metadata and
invalid inputs with synthetic plists. The preview retains its distinct bundle identifier,
profile directory and launcher `--disable-updates`; compatibility metadata does not enable its
updater or make it interchangeable with the standard app.

Signed candidate resources include the app licenses/attribution, embedded Latin and pinned
craft-fonts font licenses, and the pinned craft-i18n attribution, proofing manifest and standard
license texts. Upstream trademark artwork is omitted from this modified build; original
trademark terms and copyright notices remain included.
