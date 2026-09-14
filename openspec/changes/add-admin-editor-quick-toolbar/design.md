## Context

Administrators author post content through
`apps/web/src/app/admin/components/inputs/rich-text-editor/tiptap-editor.tsx`.
The persistent `toolbar/toolbar.tsx` currently owns a single-file-at-a-time
upload flow and image-URL insertion; video insertion uses TipTap's configured
YouTube extension and code-block insertion uses `CodeBlockLowlight`. The
editor already uploads pasted and dropped images directly to the existing
authenticated media endpoint.

The existing admin media page (`apps/web/src/app/admin/media/page.tsx`) lists
the authenticated `GET /media` response as `MediaMetadata[]`. Its items carry
`path`, `url`, `contentType`, `size`, and `lastModified`; the API constructs
the usable `url` before returning the envelope. The page filters only at the
UI level and has no reusable picker. `isImageContentType` already defines the
client-side `image/*` predicate.

This change adds a right-click Quick Toolbar and turns image insertion into a
shared editor concern. It is web-only: the existing protected media list and
upload APIs, media storage, authentication boundary, TipTap extensions, and
persisted post HTML are unchanged.

Graph-gate note: `get_minimal_context(task="add-admin-editor-quick-toolbar")`
returned `stale_graph` because its graph was built on `v4.0.4` at
`9bd2593ee6ae`, while this checkout is `main` at `c6efa1f94cd`. Targeted source
inspection substituted for graph evidence: the editor and toolbar files above,
the media page and components, `MediaModels.ts`, `api.config.ts`, the media
list adapter and `MediaMetadata` contract, existing rich-text and media Vitest
tests, plus recent editor/media Git history. No graph community/caller/callee
finding is claimed.

## Goals / Non-Goals

**Goals:**

- Let an administrator right-click editable editor content to invoke exactly
  Add Image, Add Video, and Add Code Block at the clicked document position.
- Provide one image insertion workflow for the persistent toolbar and Quick
  Toolbar: choose an ordered list of existing media images or upload an ordered
  list of local image files.
- Keep the persistent toolbar's image URL insertion compatible.
- Preserve the intended insertion target while focus moves through menus,
  dialogs, browser file selection, media queries, uploads, and intervening
  editor transactions.
- Handle loading, empty, error, cancellation, partial upload failure, focus,
  and keyboard dismissal predictably.

**Non-Goals:**

- New API routes, media-library permissions, schema/entity changes, storage
  changes, migrations, backfills, or deployment configuration.
- Selecting or inserting non-image assets, video-file uploads, other video
  providers, a code-language chooser in the Quick Toolbar, or redesigning
  unrelated editor tools.
- Replacing paste/drop image behavior; it remains an existing independent path.
- Multi-bucket browsing or a general-purpose media-management redesign. The
  picker consumes the existing default `GET /media` inventory only.

## Decisions

### 1. Put insertion commands and target ownership in `TipTapEditor`; make UI surfaces request them

`TipTapEditor` will own a small insertion controller: begin an insertion at the
current selection or an explicit context-menu position, map its target through
editor transactions, and insert image URLs, a YouTube URL, or a default code
block. `Toolbar` and the new `QuickToolbar` only request a named action and
render their own controls. The image picker and upload completion call the same
controller callback.

This separates editor state and ProseMirror transaction semantics from the
persistent and contextual user interfaces, prevents a second copy of upload
logic, and makes both entry paths conform to the same ordering behavior.

Alternatives considered:

- Keep upload and insertion state in `Toolbar` and duplicate it in Quick
  Toolbar: rejected because target persistence and partial-failure behavior
  would diverge.
- Insert at `editor.chain().focus()` after async completion: rejected because
  focus or selection can change while a dialog/file picker/upload is open.
- Add a temporary document marker node: rejected because it changes the
  document/schema and risks persisting a marker on cancellation.

### 2. Persist a transaction-mapped ProseMirror selection bookmark, not a raw position

On a valid context-menu event, the editor resolves `view.posAtCoords`, sets a
selection at that position, and captures a `SelectionBookmark`. The controller
maps that bookmark for every subsequent editor transaction and resolves it only
immediately before insertion. A normal toolbar request captures the current
selection in the same way. On success it creates consecutive image nodes or
the requested block at that resolved selection; it clears the pending target on
completion or cancellation.

This keeps insertion deterministic even when a mounted dialog changes focus or
the document is edited before an asynchronous operation completes. It also
prevents the default `setImage` behavior from replacing the editor's later
active selection.

Alternatives considered:

- Store only `from`/`to` numbers: rejected because unmapped positions become
  stale after a transaction.
- Lock editor editing while a picker/upload is open: rejected because it
  unnecessarily blocks authoring and does not solve all focus transitions.

### 3. Implement a small contextual menu, not the browser context menu or a new dependency

`EditorContent` handles `onContextMenu` only when `editor.isEditable` and a
document position resolves. It calls `preventDefault`, stores viewport-aware
coordinates, and renders `quick-toolbar.tsx` as a fixed menu with exactly the
three product-labelled actions. The menu clamps its origin against its measured
size and viewport margins. It closes on Escape, outside pointer interaction,
read-only transition, or action selection. If the editor is read-only, the
target is unavailable, or the event is outside editor content, the handler
does nothing so native context-menu behavior remains available.

The menu uses labelled `button`/`menuitem` controls, focuses the first action
when opened, and returns focus to the editor selection when dismissed. Its
action dialogs have an accessible name, initial focus, Escape and cancel paths,
and return focus to the invoking editor control.

Alternatives considered:

- Use a third-party context-menu package: rejected because DaisyUI and React
  already provide the required primitives, and a new dependency adds no
  necessary behavior.
- Suppress every editor context menu: rejected because a read-only or
  unresolvable target must retain native behavior.

### 4. Add a reusable image-insertion dialog backed by current media contracts

`image-insertion-dialog.tsx` will be mounted once by `TipTapEditor` and
opened by either UI surface. It will:

1. Fetch `getApiUrl('/media')` through `authenticatedFetch` using `useAuth()`.
2. Decode `{ data: MediaMetadata[] }`, filter with `isImageContentType`, and
   present the returned `media.url` and filename/path metadata.
3. Keep selected items in a sequence (not a set) so the first selection is
   order 1; show that ordinal in the grid and use this sequence on confirm.
4. Present explicit loading, empty, and error-with-retry states. Confirm is
   disabled unless the successful query has one or more selected images.
5. Accept a native `multiple accept="image/*"` file input. Upload files
   sequentially to the existing `getMediaUploadApiUrl()` with
   `createAuthHeaders(token)`, accumulating successes and failures without
   abandoning later files.

After a media confirm or upload completion, the controller receives one ordered
list of source URLs and creates/insert images consecutively at its active
target. The dialog reports `N added` and `M failed` via the existing Sonner
pattern. It clears its file input and selection after closing or completion.
All-failure, empty, and cancel paths produce no editor mutation. The persistent
toolbar retains its image URL input and calls the same controller for one URL;
the Quick Toolbar intentionally has no image-URL action.

Alternatives considered:

- Reuse `MediaGridItem` unchanged: rejected because it couples selection to
  preview/copy/delete administration callbacks and has no ordered insertion or
  dialog focus contract.
- Create a new API that returns image-only media: rejected because current
  authenticated metadata already includes content type and usable URL.
- Upload all selected files concurrently: rejected because response completion
  order would not reliably preserve the author-selected file order and would
  complicate partial-result reporting.

### 5. Reuse existing TipTap node contracts for video and code blocks

Quick Toolbar Add Video opens a small accessible URL prompt that submits to the
same `setYoutubeVideo({ src })` command as the persistent toolbar. Quick
Toolbar Add Code Block invokes the same default/plain-text code-block command
as the persistent toolbar's Plain Text option and focuses the new block. It
does not add a language selector, provider validation layer, or video upload.

## Component and flow contract

```text
Persistent toolbar Image ─┐
                         ├─> TipTapEditor insertion controller ─> TipTap transaction
Quick Toolbar Add Image ─┘             │
                                       ├─ Image insertion dialog
                                       │    ├─ GET /media (authenticated) -> image MediaMetadata.url[]
                                       │    └─ POST /media (authenticated, sequential) -> uploaded url[]
                                       ├─ YouTube dialog -> setYoutubeVideo
                                       └─ Code command -> default code block
```

Affected web artifacts:

- `tiptap-editor.tsx`: editor event wiring, transaction-mapped target,
  controller lifecycle, and dialog composition.
- `toolbar/toolbar.tsx`: persistent image UI delegates media/local/URL insert
  requests to the controller instead of owning a divergent upload loop.
- `quick-toolbar.tsx` (new): viewport-positioned three-action contextual menu.
- `image-insertion-dialog.tsx` (new): media query, ordered selection, local
  upload, feedback, and accessible dialog lifecycle.
- A small pure insertion-target helper (new) isolates mapping/insertion
  mechanics for unit testing.

No API/data contract changes are required. The consumed list response is
`{ data: MediaMetadata[] }`; each chosen source is the existing `url: string`.
The upload response remains `{ data: { url: string, ... } }`. Existing backend
authentication continues to authorize both requests. The client must use the
returned URLs rather than construct access URLs from `path`.

## Security, privacy, and operational behavior

- UI access remains within the existing authenticated admin route and existing
  protected media endpoints; this change adds no client-side privilege bypass.
- The picker renders only `image/*` metadata, but server-side upload
  validation/authorization remains authoritative. Client acceptance filtering
  is a usability guard, not a security control.
- Error feedback must not display auth tokens, raw response bodies, or storage
  credentials. Console logging follows existing client practice but must not
  log file content or bearer tokens.
- There is one list request each time the picker opens or the user retries; no
  polling or background prefetch is introduced. Sequential upload deliberately
  trades throughput for stable order and bounded request pressure.
- No new telemetry, runtime configuration, database migration, backfill, or
  service rollout is necessary. Existing network failures are surfaced as
  actionable UI state/toasts.

## Risks / Trade-offs

- **Target mapping implementation differs from TipTap/ProseMirror API
  expectations** → Isolate it in a helper and unit-test mapping before and
  after inserted content; use the existing `@tiptap/pm` dependency rather than
  introducing a parallel ProseMirror version.
- **A custom menu can clip or leave focus behind** → Clamp after layout,
  centralize open/close paths, and test Escape/outside click/focus restoration.
- **Large media inventories can make the dialog slow or dense** → Scope to the
  current endpoint and a responsive scrollable grid; pagination/search is a
  follow-up rather than silently changing API scope.
- **Some uploads fail after earlier successes** → Continue sequentially,
  insert only collected successful URLs once, state exact success/failure
  counts, and never auto-retry or duplicate a node.
- **Concurrent document changes move an insertion target** → Map the pending
  bookmark on every transaction and clear it after each terminal path.
- **Existing toolbar dropdown behavior regresses during refactor** → Preserve
  its URL field and verify URL, local upload, and media selection against the
  same controller in focused Vitest coverage.

## Migration Plan

1. Add unit/component coverage before behavior changes, then implement the
   controller, picker, and contextual surface in the admin web bundle.
2. Run focused rich-text and media-picker tests, followed by the admin build.
3. Deploy as a normal frontend bundle update; browser reload picks up the new
   controls. Existing stored rich-text HTML requires no transformation.
4. If a production regression is detected, roll back the frontend bundle to
   the previous version. No database/API/storage rollback or data repair is
   needed; already-inserted image/video/code HTML remains valid existing TipTap
   content.

## Open Questions

None blocking. The picker intentionally uses the existing default media
inventory; cross-bucket browse, search, pagination, and drag-to-reorder are
separate product changes if needed.
