## ADDED Requirements

### Requirement: Editable rich-text content exposes a mouse Quick Toolbar
The system SHALL replace the native browser context menu with an editor Quick
Toolbar only when an authenticated administrator right-clicks inside editable
admin rich-text content and the editor can resolve the pointer to a document
position. The Quick Toolbar SHALL contain exactly these three actions: **Add
Image**, **Add Video**, and **Add Code Block**. The system SHALL NOT show the
Quick Toolbar or suppress the native context menu for a read-only editor, a
right-click outside editable editor content, or an unresolvable document
position.

#### Scenario: Right-click opens the three-action Quick Toolbar
- **WHEN** an administrator right-clicks an editable paragraph at a resolvable
  document position
- **THEN** the browser context menu is suppressed
- **AND** a Quick Toolbar is displayed adjacent to the pointer without
  overflowing the viewport
- **AND** the Quick Toolbar exposes only Add Image, Add Video, and Add Code
  Block actions

#### Scenario: Read-only content keeps normal context-menu behavior
- **WHEN** an administrator right-clicks a read-only rich-text editor
- **THEN** the Quick Toolbar is not displayed
- **AND** the browser's native context-menu handling is not suppressed

### Requirement: Quick Toolbar actions preserve the invocation insertion target
The system SHALL capture the document position targeted by a Quick Toolbar
invocation before opening any menu, dialog, file picker, or asynchronous
operation. Each Quick Toolbar action SHALL apply its result at that captured
position, after mapping the target through intervening editor transactions, and
SHALL NOT replace unrelated existing document content. Closing or cancelling an
action SHALL leave document content unchanged.

#### Scenario: Image selection inserts at the right-clicked location
- **WHEN** an administrator right-clicks between two existing paragraphs,
  chooses Add Image, and confirms one or more images after the editor focus has
  moved to the picker
- **THEN** the selected images are inserted at the original right-clicked
  location in the document
- **AND** the surrounding paragraphs remain present and in their original
  order

#### Scenario: A document edit before confirmation does not discard the target
- **WHEN** a Quick Toolbar image picker is open and an editor transaction
  changes content before the captured insertion target
- **THEN** confirming the images inserts them at the transaction-mapped target
- **AND** the system does not insert images at the editor's later active
  selection

#### Scenario: Cancelling an action changes nothing
- **WHEN** an administrator opens an image or video action from the Quick
  Toolbar and cancels it
- **THEN** the Quick Toolbar and associated dialog are dismissed
- **AND** no image, video, or code block is added to the document

### Requirement: Quick Toolbar reuses the existing video and code-block authoring contracts
The system SHALL use the configured TipTap YouTube extension when Add Video is
confirmed from the Quick Toolbar and SHALL use the configured TipTap code-block
extension when Add Code Block is selected. Add Code Block SHALL create the
same plain-text/default-language code-block form available from the persistent
toolbar; it SHALL NOT introduce a code-language selector or new video provider.

#### Scenario: Quick Toolbar adds a video using the existing provider contract
- **WHEN** an administrator chooses Add Video from the Quick Toolbar and
  confirms a supported YouTube URL
- **THEN** the editor inserts the configured YouTube video node at the captured
  insertion target
- **AND** no video-file upload or alternative video provider is invoked

#### Scenario: Quick Toolbar adds a default code block
- **WHEN** an administrator chooses Add Code Block from the Quick Toolbar
- **THEN** the editor inserts a plain-text/default-language code block at the
  captured insertion target
- **AND** the editor moves focus to the new code block for authoring

### Requirement: Quick Toolbar is dismissible and keyboard operable after opening
The system SHALL render Quick Toolbar actions as labelled interactive controls.
When the toolbar opens, the system SHALL move focus to its first action. Escape
and an outside pointer interaction SHALL dismiss it without mutation; after
dismissal, the system SHALL restore focus to the editor at the captured
selection. Any dialog launched by an action SHALL expose an accessible name,
receive initial focus, support Escape dismissal, and restore focus to the
invoking editor control on close.

#### Scenario: Keyboard dismissal restores editor focus
- **WHEN** the Quick Toolbar is open and an administrator presses Escape
- **THEN** the Quick Toolbar closes without changing document content
- **AND** focus returns to the editable rich-text content at the captured
  selection

#### Scenario: Image dialog exposes a usable focus lifecycle
- **WHEN** an administrator opens image selection from the Quick Toolbar
- **THEN** the image selection dialog has an accessible name and receives
  focus
- **AND** pressing Escape closes it without inserting images and returns focus
  to the editor control that invoked it
