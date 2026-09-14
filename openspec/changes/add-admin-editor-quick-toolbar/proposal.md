## Why

Admin authors currently have to move from the writing area to the persistent editor toolbar to add common rich-content blocks. They also cannot reuse one or more images already managed in the CMS from the image action, which adds unnecessary upload and copy/paste work.

## What Changes

- Add a contextual **Quick Toolbar** to the editable admin rich-text area when an author right-clicks with a mouse. It offers **Add Image**, **Add Video**, and **Add Code Block** actions at the clicked insertion location.
- Let the Quick Toolbar's Add Image action and the persistent toolbar's image action insert one or more existing image assets selected from the admin media library.
- Preserve local image upload in both image-entry paths and allow authors to select multiple local image files in one action.
- Insert each selected or successfully uploaded image into the document in the author's selected order, without replacing existing content.
- Preserve the current video and code-block authoring behavior when those actions are invoked from the Quick Toolbar.

## Capabilities

### New Capabilities

- `admin-editor-quick-toolbar`: Contextual right-click actions that let an admin author add an image, video, or code block at the intended location in rich-text content.
- `admin-editor-image-insertion`: Image insertion workflow shared by the persistent editor toolbar and Quick Toolbar, covering multiple existing media-library images and multiple local image uploads.

### Modified Capabilities

- None. No canonical OpenSpec capability currently defines admin rich-text editor authoring requirements.

## Impact

- Affects the admin web rich-text editor and its persistent toolbar.
- Adds a reusable admin media-library selection experience; the existing media listing is the source of selectable assets.
- Reuses existing authenticated media access and local image-upload behavior; no new public content, media-management, or API capability is requested by this proposal.

## Scope

- Applies to authenticated administrators editing content through the existing admin rich-text editor.
- The Quick Toolbar is mouse right-click initiated and contains only the three named actions.
- Existing media-library selection is limited to image assets and supports selecting more than one asset before insertion.
- Local image selection supports more than one image file before upload and insertion.

## Non-Goals

- Redesigning the full rich-text editor or its other toolbar actions.
- Adding a new media library, changing media storage, or changing media permissions.
- Supporting audio, document, or arbitrary non-image assets in the image picker.
- Adding video-file upload, new video providers, or a code-language selector to the Quick Toolbar.
- Replacing keyboard, paste, drag-and-drop, URL-image, or existing toolbar behavior unrelated to this flow.

## Assumptions and Dependencies

- Confirmed: “list of images” includes choosing multiple existing images from the admin media library, as well as multiple local image files for upload.
- The current admin media listing remains the authoritative inventory of reusable image assets and returns usable image URLs to authorized administrators.
- The persistent toolbar continues to provide its existing image URL option unless product review later removes it.

## Risks

- A contextual menu can interfere with native browser behavior or lose the clicked insertion point; the experience must clearly insert at the location the author targeted.
- Media-library listings can be empty, loading, inaccessible, or contain non-image files; these states must not block editing or result in invalid inserts.
- Partial local-upload failure must communicate the result accurately and retain successfully inserted images without duplicating content.

## Acceptance Outcomes

- An administrator can right-click in editable rich-text content and see only Add Image, Add Video, and Add Code Block in the Quick Toolbar.
- Choosing any Quick Toolbar action inserts or begins the corresponding existing authoring flow at the location that was right-clicked.
- From either image entry point, an administrator can select multiple existing media-library images and insert all selected images into the document in selection order.
- From either image entry point, an administrator can select multiple local image files, receive clear progress/result feedback, and have every successful upload inserted in file-selection order.
- The editor remains usable when the media library is empty, fails to load, or one or more local uploads fail.
