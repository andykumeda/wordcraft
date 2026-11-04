# Assistant preview updates

Status: 2026-11-03 — independent shared-helper updates are implemented. WordCraft's document
engine and host controls still require a rebuilt app binary. The current compact-layout source
and installed candidate state are in the [integration ledger](upstream-assistant-v015.md#compact-assistant-rollout--2026-11-03).
The gear contains helper update checks; preview host app updates remain disabled.
No public release is implied by a local signed candidate.

## Historical delivery (2026-10-29)

The owner uses `/Applications/WordCraft Assistant Preview.app`, separately from the ordinary
`/Applications/WordCraft.app`. Every delivered preview also has a version/commit-named
Apple-silicon DMG and SHA-256 checksum. Existing profiles, conversations and credentials stay
outside the bundle and are never copied into packages.

The then-installed preview was **0.12.0 (`9c6c2cb0`)**. Mounted/copied DMG apps, rollback copy and
installed native hash were verified; launcher/View/settings/updater smoke passed. Sol/Terra
synthetic document acceptance and external comparisons are in [benchmarks](assistant-benchmarks.md).
WordCraft's canonical per-app choice is now Terra; other settings, credentials and chats remain.
The exact installed Terra handbook edit passed in **11.477s** with visible background result,
saved/reopened content/formatting and one normal Undo verified; normal quit/socket/key cleanup
passed. Evidence: `/private/tmp/wordcraft-preview-installed-edit-receipt.json`. The main
WordCraft 0.10 app is unchanged.
The DMG/checksum/install notes are in `dist/assistant-preview/9c6c2cb0/` and Downloads. DMG SHA-256:
`cea0796333f9ca009ea685a9c58a1a15184dd47546a05216b36ab8b248f288c2`.
Ad-hoc signing, no notarization and other-Mac launch acceptance remain explicit limitations;
no WordCraft push or public release occurred.

The preview launcher passes `--disable-updates`. This blocks startup checks, manual checks and
the standard updater's install/prompt path. Manual Check for Updates explains that updates are
disabled. The ordinary WordCraft app retains its normal updater. Preview updates use a new
preview DMG, so an official release cannot silently overwrite the assistant additions.

For each preview update:

1. Fetch/review upstream `storytold/wordcraft` changes and merge them into the preview branch.
   Preserve the assistant adapter and the pinned, reviewed shared-library revision. Resolve
   conflicts explicitly; do not substitute an upstream binary for the preview.
2. Run `cargo xtask ci` without concurrent builds during timing tests. Commit the verified source.
3. Build the release binary with its exact source SHA. Package with
   `packaging/macos/assistant-preview.sh`; it checks version/commit and updater-opt-out support.
4. Verify ZIP extraction, the signed app mounted from the DMG, a copy from that DMG, architecture,
   checksums and the preview launcher. Install only the preview under `/Applications`, keeping
   a rollback copy of any version replaced. Never force-quit or discard unsaved writing.
5. Smoke the installed app with synthetic content and a disposable profile; verify its version,
   View button, settings and disabled updater. Run the actual selected model on a synthetic
   document and verify the visible result, saved/reopened content and Undo. Record actual results and remaining other-Mac
   acceptance. A currently open older instance must be saved/quit and reopened at the exact
   new installation path before it uses the new code.

This is an owner-operated workflow, not an automatic scheduled updater. No recurring job or
public preview release channel is configured. Signing is ad-hoc unless a real signing identity
is explicitly supplied; notarization and successful launch on another Mac are separate checks.

## What can update independently today

Ollama models and the configured providers are already external to WordCraft. Compatible model
updates and model selection do not require an editor rebuild. Discovery and a synthetic tool
capability test are still needed; a model's larger advertised context does not change Ollama's
actual allocated context. Increase the context allocation in Ollama when necessary.

The assistant runtime, shared settings widgets and WordCraft host adapter are currently compiled
into the editor. Updating their code requires a new preview build. Taking new upstream editor
code also requires integrating and rebuilding this branch. These code lifecycles are not yet
fully independent.

## Proposed separation: keep the panel, move work into a background service

Only the WordCraft window is visible. A separately installed, signed Assistant service runs in
the background; it is not another chat or MCP-client window.

| Component | Responsibility | Update lifecycle |
| --- | --- | --- |
| WordCraft host | Native panel, document/selection/revision guards, exact file grants, command dispatch and Undo | Editor releases |
| Assistant service | Provider adapters, model discovery/routing, inference and conversation orchestration/history | Independent service releases |
| Ollama/provider | Model weights, context allocation and provider account | Existing external lifecycle |

The host and service need a versioned local protocol with capability negotiation. Preserve the
current per-window/document boundaries and file authorization at the host; an updated service
must not gain unrestricted document/file access. Keep credentials in the platform store and
version history/settings migrations without deleting old data. No document-derived instructions
may change routing, permissions or update policy.

A service update should activate between turns, preserve an earlier version for rollback and
fail visibly when incompatible. Editor and service releases should run contract tests for reads,
edits, Undo, cancellation, stale targets and file grants against their supported version range.
Protocol changes can still require coordinated releases; independence is not a promise that
arbitrary breaking changes will work together.

The integrated panel/bridge must be accepted upstream, or maintained through upstream merges.
A separate service alone cannot make the official WordCraft updater preserve an unmerged native
panel. Panel UI changes may still require an editor build. No service split, plugin system or
separate update channel has been implemented by this delivery.

## Revision history

| Date | Change |
| --- | --- |
| 2026-10-29 | Record verified 9c6c2cb0 package/install and Terra per-app default; retain updater isolation and destination-Mac acceptance gate. |
| 2026-10-29 | Define preview delivery and updater isolation; retain in-app UI as the requirement for future service separation. |
