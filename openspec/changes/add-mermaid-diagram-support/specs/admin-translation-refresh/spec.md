## ADDED Requirements

### Requirement: Background translation does not refresh the editing form

The admin blog edit form SHALL NOT issue recurring requests to `GET /posts/{post_id}/translate/jobs`. Starting a translation or retranslation SHALL continue to create the existing server-side background job and SHALL NOT replace the form's current values, TipTap editor state, or open Article Preview while that job runs.

#### Scenario: Translation begins while an author previews Mermaid content
- **WHEN** an author starts a background translation and opens an Article Preview containing a valid Mermaid block
- **THEN** no recurring job-status request is sent by the form
- **AND** the diagram and preview snapshot remain visible while the server job changes state

#### Scenario: Translation finishes while an author edits
- **WHEN** a server-side translation job completes while an author has unsaved changes in the base or translated editor
- **THEN** the form does not reload the post or overwrite those changes
- **AND** the completed translation is available after an explicit content refresh or reopening the edit form

### Requirement: Authors can explicitly refresh translated content

The admin form SHALL provide an author-initiated way to reload the post after starting a background translation. The refresh SHALL replace editor content only after warning about unsaved changes through the existing form interaction pattern or when the form is clean.

#### Scenario: Author refreshes after translation completion
- **WHEN** an author selects Refresh content after a background translation has completed
- **THEN** the form reloads the post once and displays the new translation
- **AND** no background polling resumes after the refresh
