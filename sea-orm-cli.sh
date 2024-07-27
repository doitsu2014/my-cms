# Migration Up
sea-orm-cli migrate up --database-url postgres://postgres:1234567890@localhost:5432/my-cms

# Migration Down
sea-orm-cli migrate down --database-url postgres://postgres:1234567890@localhost:5432/my-cms

# Generate Entities
sea-orm-cli generate entity --database-url postgres://postgres:1234567890@localhost:5432/my-cms -o application_core/src/entities --with-serde both
