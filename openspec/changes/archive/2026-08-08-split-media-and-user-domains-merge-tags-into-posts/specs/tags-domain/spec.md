## REMOVED Requirements

### Requirement: Standalone tags command domain boundary
**Reason**: Tags are a post sub-concept and the standalone `commands::tag` boundary duplicates or conflicts with the post domain ownership model.
**Migration**: Move the tag command modules and tests under the post domain, update all internal and API imports, and retain the existing tag endpoints and behavior. No data migration is required.

#### Scenario: No standalone tag command module remains
- **WHEN** the refactor is complete
- **THEN** `commands::tag` is no longer declared or imported
- **AND** tag handlers remain reachable through the post domain boundary.
