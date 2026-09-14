## ADDED Requirements

### Requirement: Both image entry points offer shared existing-media selection
The system SHALL offer an existing-media-library selection path from both the
persistent rich-text toolbar Image control and the Quick Toolbar Add Image
action. The selection experience SHALL retrieve the existing authenticated
`GET /media` inventory, use each returned `MediaMetadata.url` as the image
source, and only present entries whose `contentType` starts with `image/`.
It SHALL not add a media API, change media permissions, or expose non-image
media as insertable images.

#### Scenario: Both image entry points open the same media inventory
- **WHEN** an administrator requests existing-media selection from either the
  persistent Image control or Quick Toolbar Add Image action
- **THEN** the system requests the authenticated existing media inventory
- **AND** the picker presents only returned assets whose content type is an
  image type
- **AND** it uses the returned media URLs without constructing alternate image
  URLs from object paths

#### Scenario: Non-image media cannot be selected for image insertion
- **WHEN** the existing media inventory contains a PDF or another non-image
  asset together with image assets
- **THEN** the non-image asset is not offered as an image-insertion choice
- **AND** confirming the picker can insert only image assets

### Requirement: Existing-media selection inserts a deliberate ordered list of images
The system SHALL allow an administrator to select more than one existing image
in a single picker session. The picker SHALL record each selected image's order
of selection, visibly communicate that order, and insert the confirmed image
URLs in that order at the active image insertion target. Cancelling or
confirming an empty selection SHALL not change the document.

#### Scenario: Multiple selected media images retain selection order
- **WHEN** an administrator selects media images B, then A, then C and confirms
  the picker
- **THEN** the editor inserts images B, A, and C consecutively at the requested
  insertion target
- **AND** the picker does not reorder the images by filename, upload time, or
  grid position

#### Scenario: No confirmed media selection makes no edit
- **WHEN** an administrator opens the media picker and cancels it or confirms
  without any selected image
- **THEN** no image is inserted
- **AND** existing editor content is unchanged

### Requirement: The existing-media picker handles query states without blocking authoring
The existing-media picker SHALL expose a loading state while its media query is
pending, an empty state when no returned image assets are available, and an
error state with a retry control when the query fails. The confirm control SHALL
remain unavailable until at least one image is selected and the query is not
pending. Dismissing any state SHALL preserve the current document.

#### Scenario: Empty media library remains usable
- **WHEN** the authenticated media query succeeds but returns no image assets
- **THEN** the picker explains that no images are available
- **AND** it does not offer an enabled insert control
- **AND** the administrator can close the picker and continue editing

#### Scenario: Failed media query can be retried
- **WHEN** the authenticated media query fails
- **THEN** the picker presents a non-destructive error state and retry control
- **AND** selecting retry issues a new authenticated media query
- **AND** no image is inserted unless a later successful query is explicitly
  confirmed

### Requirement: Both image entry points support multiple local image uploads with partial-success reporting
The system SHALL allow an administrator to select multiple local files matching
`image/*` from either image entry point. It SHALL submit the selected files to
the existing authenticated media upload endpoint one at a time in file-selection
order, continue after an individual failure, and insert every successful upload
in file-selection order at the active image insertion target. It SHALL clear
the file control after completion and report the number of successful and failed
uploads; it SHALL not insert a failed upload or duplicate a successful one.

#### Scenario: All selected local images upload and insert in order
- **WHEN** an administrator selects local files `one.png`, `two.jpg`, and
  `three.webp` from either image entry point and all uploads succeed
- **THEN** the system calls the existing image upload endpoint in that file
  order
- **AND** it inserts the three returned URLs consecutively in the same order
- **AND** it reports three successful uploads and zero failures

#### Scenario: A later local upload still runs after an earlier failure
- **WHEN** an administrator selects `one.png`, `two.jpg`, and `three.webp`,
  and the upload for `two.jpg` fails while the other uploads succeed
- **THEN** the system inserts only the returned URLs for `one.png` and
  `three.webp` in that order
- **AND** it reports two successful uploads and one failure
- **AND** it does not retry, insert, or duplicate `two.jpg` automatically

#### Scenario: All local uploads fail without an editor mutation
- **WHEN** every selected local image upload fails
- **THEN** the system reports that no images were added and the failure count
- **AND** the editor content remains unchanged

### Requirement: Persistent toolbar image URL insertion remains available
The persistent rich-text toolbar SHALL retain its existing image-URL entry
path. Confirming a non-empty URL SHALL insert one image at the active image
insertion target; cancelling or submitting an empty URL SHALL not modify the
document. This change SHALL not add URL insertion to the Quick Toolbar.

#### Scenario: Toolbar URL insertion remains compatible
- **WHEN** an administrator enters a non-empty image URL in the persistent
  toolbar and confirms it
- **THEN** the editor inserts one image using that URL at the active insertion
  target
- **AND** the media-library and local-upload choices remain available from the
  same persistent Image control
