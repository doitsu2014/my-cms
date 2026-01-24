# Migration Up
sea-orm-cli migrate --database-url postgres://postgres:1234567890@localhost:5432/my-cms up

# Migration Down
sea-orm-cli migrate --database-url postgres://postgres:1234567890@localhost:5432/my-cms down

# Generate Entities
sea-orm-cli generate entity --database-url postgres://postgres:1234567890@localhost:5432/my-cms -o application_core/src/entities --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --seaography
