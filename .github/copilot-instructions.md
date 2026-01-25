# GitHub Copilot Instructions for My-CMS Project

This document provides comprehensive coding guidelines and best practices for the My-CMS project to enhance GitHub Copilot's code generation quality and maintain consistency across the codebase.

## Project Overview

My-CMS is a headless CMS system with:
- **Backend**: Rust-based API using Axum framework, SeaORM, and PostgreSQL
- **Frontend**: React-based admin panel using DaisyUI, React Hook Form, and TipTap editor
- **Architecture**: Modular design with clear separation between API layer, Application Core, and Database layer
- **Key Features**: AI-powered translation service, GraphQL API, media management, and comprehensive tracing

## Quick Reference

- [Rust Backend Guidelines](./copilot-instructions-rust.md)
- [React Frontend Guidelines](./copilot-instructions-react.md)
- [Project Architecture](./copilot-instructions-architecture.md)

## General Principles

### Code Quality Standards

1. **Type Safety**: Use strong typing throughout the codebase
   - Rust: Leverage the type system for compile-time guarantees
   - TypeScript: Use strict mode and avoid `any` types

2. **Error Handling**: Always handle errors explicitly
   - Rust: Use `Result<T, AppError>` for fallible operations
   - React: Use error boundaries and proper error states

3. **Async Programming**: Follow best practices for asynchronous code
   - Use `async/await` syntax consistently
   - Handle concurrent operations efficiently
   - Avoid blocking operations in async contexts

4. **Documentation**: Write clear, concise comments for complex logic
   - Document public APIs thoroughly
   - Include examples for non-obvious usage
   - Keep comments up-to-date with code changes

### Naming Conventions

- **Rust**: 
  - `snake_case` for functions, variables, and module names
  - `PascalCase` for types, structs, and traits
  - `SCREAMING_SNAKE_CASE` for constants

- **TypeScript/React**:
  - `camelCase` for functions and variables
  - `PascalCase` for components, types, and interfaces
  - `SCREAMING_SNAKE_CASE` for constants

### Testing Strategy

- Write unit tests for business logic
- Use mock databases for testing (SeaORM mock feature)
- Use testcontainers for integration tests
- Maintain test coverage for critical paths

## Technology Stack

### Backend
- **Language**: Rust (edition 2021)
- **Web Framework**: Axum 0.8.x
- **ORM**: SeaORM 1.1.x
- **Database**: PostgreSQL
- **Async Runtime**: Tokio
- **Tracing**: OpenTelemetry with Jaeger
- **Authentication**: Keycloak integration
- **AI Services**: OpenAI API, Qdrant vector database
- **Storage**: S3-compatible storage

### Frontend
- **Framework**: React 19.x
- **Build Tool**: Rsbuild
- **UI Library**: DaisyUI 5.x with Tailwind CSS 4.x
- **Forms**: React Hook Form with Zod validation
- **Rich Text Editor**: TipTap
- **GraphQL Client**: Apollo Client
- **Authentication**: Keycloak JS
- **Routing**: React Router v7

## Project Structure

```
my-cms/
├── services/                    # Rust backend
│   ├── src/
│   │   ├── api/                # API layer (routes and handlers)
│   │   ├── common/             # Shared utilities
│   │   └── presentation_models/# API request/response models
│   ├── application_core/       # Business logic
│   │   ├── src/
│   │   │   ├── commands/       # Command handlers
│   │   │   ├── entities/       # SeaORM entities (auto-generated)
│   │   │   └── common/         # Common domain logic
│   ├── migration/              # Database migrations
│   └── test_helpers/           # Test utilities
├── frontend/                    # React frontend
│   ├── src/
│   │   ├── app/                # Pages and layouts
│   │   ├── components/         # Reusable components
│   │   ├── domains/            # Domain types
│   │   ├── models/             # Data models
│   │   ├── schemas/            # Zod validation schemas
│   │   ├── auth/               # Authentication logic
│   │   └── config/             # Configuration
└── docs/                        # Documentation
```

## Build and Run

### Backend
```bash
# Run migrations
sea-orm-cli migrate --database-url <connection_string> up

# Generate entities (after schema changes)
sea-orm-cli generate entity --database-url <connection_string> \
  -o application_core/src/entities --with-serde both \
  --model-extra-attributes 'serde(rename_all = "camelCase")' --seaography

# Build
cargo build --release

# Run tests
cargo test

# Run with coverage
./take-coverage.sh
```

### Frontend
```bash
cd frontend

# Install dependencies
pnpm install

# Development server
pnpm dev

# Build for production
pnpm build

# Lint
pnpm lint

# Format
pnpm format
```

## Environment Configuration

Key environment variables (see `.env` file):
- `DATABASE_URL`: PostgreSQL connection string
- `HOST`, `PORT`: Server configuration
- `S3_ENDPOINT`, `S3_BUCKET_NAME`: Media storage
- `ENABLED_OTLP_EXPORTER`, `OTEL_EXPORTER_OTLP_TRACES_ENDPOINT`: Tracing
- OpenAI API key and Qdrant URL for AI features

## Key Design Patterns

### Backend Patterns
1. **Command Pattern**: Business logic organized as command handlers
2. **Repository Pattern**: Database access through SeaORM entities
3. **Dependency Injection**: Pass dependencies through struct fields
4. **Error Propagation**: Use `?` operator with custom `AppError` type

### Frontend Patterns
1. **Component Composition**: Build complex UIs from simple components
2. **Form Management**: Use React Hook Form with Zod schemas
3. **Data Fetching**: Apollo Client for GraphQL, fetch API for REST
4. **Authentication Context**: Shared auth state via React Context

## Security Considerations

1. **Input Validation**: Validate all user inputs
2. **SQL Injection**: Use parameterized queries (SeaORM handles this)
3. **XSS Prevention**: Sanitize HTML content in rich text editor
4. **Authentication**: Verify JWT tokens on all protected routes
5. **CORS**: Configure appropriately for production

## Performance Guidelines

1. **Database**: Use indexes, optimize queries, implement caching
2. **API**: Implement pagination for list endpoints
3. **Frontend**: Use lazy loading, code splitting, optimize bundle size
4. **Concurrency**: Use parallel processing where appropriate (e.g., AI translations)

## Contributing Guidelines

1. Follow the existing code style and patterns
2. Write tests for new features
3. Update documentation when making changes
4. Use meaningful commit messages
5. Keep changes focused and atomic

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [Axum Documentation](https://docs.rs/axum/)
- [React Documentation](https://react.dev/)
- [DaisyUI Components](https://daisyui.com/components/)
- [TipTap Documentation](https://tiptap.dev/)
