## MODIFIED Requirements

### Requirement: Post operations preserve tag behavior within the post domain
The post domain SHALL own tag creation, reading/classification, deletion, tag DTOs, and the post-to-tag command interactions under its post boundary, while preserving existing tag behavior and public contracts.

#### Scenario: Post creation and modification preserve tag synchronization
- **WHEN** a post is created or modified with tag names
- **THEN** the post command invokes the relocated tag commands within the post domain
- **AND** tags are created, reused, classified, and associated with the post exactly as before.

#### Scenario: Existing tag API operations preserve behavior
- **WHEN** an existing tag route is called
- **THEN** the API adapter resolves the tag command through the post domain boundary
- **AND** HTTP routes, request/response shapes, validation, persistence, errors, and authorization remain unchanged.

#### Scenario: Tag command tests remain valid
- **WHEN** tag command tests run after relocation
- **THEN** they pass without changing their behavioral assertions
- **AND** cross-command calls use post-domain module paths rather than a standalone tag boundary.
