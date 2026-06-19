# My-CMS Documentation

## Overview

My-CMS is a modern headless CMS built with Rust and React, featuring AI-powered translation, GraphQL API, and comprehensive media management.

## Documentation Index

### GitHub Copilot Instructions
For comprehensive coding guidelines and best practices, see the [.github directory](../.github/README.md):

- **[Main Instructions](../.github/copilot-instructions.md)** - Project overview and general guidelines
- **[Rust Backend Guide](../.github/copilot-instructions-rust.md)** - Rust patterns, async/concurrency with Tokio
- **[React Frontend Guide](../.github/copilot-instructions-react.md)** - React components, DaisyUI usage, form handling
- **[Architecture Guide](../.github/copilot-instructions-architecture.md)** - System design, AI translation service

### Quick Links

- **[Main README](../README.md)** - Project setup and overview
- **[Postman Collection](./postman_collection/)** - API testing collection
- **[AI Translation Documentation](../apps/api/application_core/src/commands/ai/README.md)** - Detailed AI service documentation

## Key Features

### Backend (Rust)
- **Axum Framework**: Fast and ergonomic web framework
- **SeaORM**: Type-safe ORM with schema-first approach
- **Tokio Async Runtime**: Efficient concurrent processing
- **AI Translation Service**: 3-tier lookup optimization (DB → Qdrant → OpenAI)
- **GraphQL API**: Auto-generated from database schema
- **OpenTelemetry Tracing**: Comprehensive observability

### Frontend (React)
- **React 19**: Latest React features
- **DaisyUI**: Beautiful UI components
- **TipTap Editor**: Rich text editing
- **React Hook Form + Zod**: Type-safe form validation
- **Apollo Client**: GraphQL integration
- **Keycloak**: Enterprise authentication

## Getting Started

### Prerequisites
- Rust 1.75+
- Node.js 20+
- PostgreSQL 15+
- Docker (for dependencies)

### Backend Setup

```bash
# Clone repository
git clone https://github.com/doitsu2014/my-cms.git
cd my-cms/apps/api

# Set up environment
cp .env.example .env
# Edit .env with your configuration

# Run migrations
sea-orm-cli migrate --database-url <your-connection-string> up

# Build and run
cargo build --release
cargo run
```

### Frontend Setup

```bash
cd apps/web

# Install dependencies
pnpm install

# Start development server
pnpm dev

# Build for production
pnpm build
```

## API Documentation

Use the [Postman Collection](./postman_collection/) to explore the API endpoints.

### Key Endpoints

- `GET /api/categories` - List categories
- `GET /api/posts` - List posts
- `POST /api/posts/:id/translate` - AI-powered translation
- `POST /api/media/upload` - Upload media files
- `POST /graphql` - GraphQL endpoint

## Development Guidelines

### Coding Standards

See the comprehensive guides in [.github/](../.github/README.md) for:
- Rust backend patterns and async/concurrency
- React component patterns and DaisyUI usage
- Architecture and design patterns
- Testing strategies

### Testing

```bash
# Backend tests
cd apps/api
cargo test

# Frontend tests
cd apps/web
pnpm test
```

### Code Coverage

```bash
cd apps/api
./take-coverage.sh
```

## Architecture

My-CMS follows a clean layered architecture:

```
API Layer (REST + GraphQL)
    ↓
Application Core (Business Logic)
    ↓
Database Layer (SeaORM + PostgreSQL)
```

See [Architecture Guide](../.github/copilot-instructions-architecture.md) for detailed information.

## Special Features

### AI Translation Service

Intelligent post translation with cost optimization:
- **3-Tier Lookup**: Database cache → Vector similarity → OpenAI translation
- **95% Similarity Threshold**: Reuse existing translations when similar
- **Parallel Processing**: Concurrent chunk translation with Tokio JoinSet
- **Background Jobs**: Non-blocking translation for large content

[Learn more](../apps/api/application_core/src/commands/ai/README.md)

### Media Management

- S3-compatible storage
- Image processing and optimization
- Folder organization
- Metadata caching with Moka

## Contributing

1. Fork the repository
2. Create a feature branch
3. Follow the [coding guidelines](../.github/README.md)
4. Write tests for new features
5. Submit a pull request

## Resources

### Documentation
- [Rust Backend Guide](../.github/copilot-instructions-rust.md)
- [React Frontend Guide](../.github/copilot-instructions-react.md)
- [Architecture Overview](../.github/copilot-instructions-architecture.md)

### External Resources
- [Rust Documentation](https://doc.rust-lang.org/)
- [Tokio Documentation](https://tokio.rs/)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [Axum Documentation](https://docs.rs/axum/)
- [React Documentation](https://react.dev/)
- [DaisyUI Components](https://daisyui.com/components/)

## License

MIT OR Apache-2.0

## Contact

For questions or issues, please open an issue on GitHub.
