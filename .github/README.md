# GitHub Copilot Instructions

This directory contains comprehensive coding guidelines and best practices for the My-CMS project to help GitHub Copilot generate better code suggestions that match the project's style and patterns.

## Documentation Files

### 📚 [copilot-instructions.md](./copilot-instructions.md)
**Main Overview Document**

The entry point for all GitHub Copilot instructions. Provides:
- Project overview and technology stack
- General coding principles
- Quick reference links to detailed guides
- Build and run instructions
- Environment configuration

**Use this when:** You need a high-level understanding of the project structure and conventions.

---

### 🦀 [copilot-instructions-rust.md](./copilot-instructions-rust.md)
**Rust Backend Guidelines** (729 lines)

Comprehensive guide for Rust development focusing on:
- **Architecture Patterns**: Command handlers, dependency injection, layered architecture
- **Async and Concurrency**: Tokio runtime, JoinSet for parallel processing, async/await best practices
- **Error Handling**: AppError type, Result propagation, error conversion
- **Database Operations**: SeaORM patterns, query optimization, transaction handling
- **API Development**: Axum handlers, router configuration, request/response models
- **Testing**: Unit tests with mocks, integration tests with testcontainers
- **Real-world Examples**: Complete command handler implementation, AI translation concurrency patterns

**Key Focus Areas:**
- ✅ Tokio concurrency with `JoinSet` for parallel operations
- ✅ Async/await patterns and background task spawning
- ✅ SeaORM best practices and entity generation
- ✅ Command pattern for business logic
- ✅ Error handling with custom AppError type

**Use this when:** Writing Rust backend code, implementing async operations, working with database queries, or creating new API endpoints.

---

### ⚛️ [copilot-instructions-react.md](./copilot-instructions-react.md)
**React Frontend Guidelines** (988 lines)

Detailed guide for React and TypeScript development focusing on:
- **Component Architecture**: Container/presentational pattern, component structure, naming conventions
- **DaisyUI Component Usage**: Comprehensive examples of all DaisyUI components (buttons, cards, forms, modals, badges, etc.)
- **Form Handling**: React Hook Form with Zod validation, field arrays, controlled components
- **State Management**: Local state, Context API, URL state with React Router
- **Data Fetching**: Authenticated fetch utility, GraphQL with Apollo Client
- **Routing**: React Router v7 patterns and navigation
- **Authentication**: Keycloak integration patterns

**Key Focus Areas:**
- ✅ DaisyUI component patterns and CSS classes
- ✅ React Hook Form + Zod validation
- ✅ Form field arrays for dynamic inputs
- ✅ Toast notifications with Sonner
- ✅ Lucide React icons usage
- ✅ Authentication with Keycloak

**Use this when:** Creating React components, building forms, using DaisyUI components, handling authentication, or managing frontend state.

---

### 🏗️ [copilot-instructions-architecture.md](./copilot-instructions-architecture.md)
**Architecture and Design Patterns** (777 lines)

In-depth architectural documentation covering:
- **System Architecture**: High-level overview, layered architecture, component relationships
- **Backend Architecture**: Project structure, command pattern, dependency injection
- **Frontend Architecture**: Component patterns, form patterns, project structure
- **AI Translation Service**: 3-tier lookup strategy (Database → Qdrant → OpenAI), parallel chunk processing, cost optimization
- **Media Management**: Upload flow, caching strategy, S3 integration
- **Database Design**: Schema-first approach, key tables, relationships
- **API Design**: REST API structure, GraphQL schema, request/response formats
- **Security Architecture**: Authentication flow, authorization, input validation
- **Deployment Architecture**: Container strategy, Kubernetes deployment, observability

**Key Focus Areas:**
- ✅ AI translation service with 3-tier lookup optimization
- ✅ Parallel chunk processing with JoinSet
- ✅ SeaORM schema-first approach
- ✅ Microservices communication patterns
- ✅ Security best practices
- ✅ Performance optimization strategies

**Use this when:** Understanding system design, implementing new features that span multiple layers, optimizing performance, or working on the AI translation service.

---

## Quick Start Guide

### For Backend Development (Rust)
1. Read [copilot-instructions.md](./copilot-instructions.md) for project overview
2. Study [copilot-instructions-rust.md](./copilot-instructions-rust.md) for Rust patterns
3. Reference [copilot-instructions-architecture.md](./copilot-instructions-architecture.md) for system design

**Key Patterns to Follow:**
```rust
// Use command handler pattern
pub trait CommandHandlerTrait {
    async fn handle(&self, request: Request) -> Result<Response, AppError>;
}

// Use JoinSet for parallel operations
let mut join_set = JoinSet::new();
for item in items {
    join_set.spawn(async move { process(item).await });
}
```

### For Frontend Development (React)
1. Read [copilot-instructions.md](./copilot-instructions.md) for project overview
2. Study [copilot-instructions-react.md](./copilot-instructions-react.md) for React patterns
3. Reference DaisyUI component examples in the React guide

**Key Patterns to Follow:**
```tsx
// Use React Hook Form + Zod
const { register, handleSubmit, formState: { errors } } = useForm({
  resolver: zodResolver(schema)
});

// Use DaisyUI components
<button className="btn btn-primary">Save</button>
<div className="card bg-base-100 shadow-xl">
  <div className="card-body">Content</div>
</div>
```

### For Full-Stack Features
1. Start with [copilot-instructions-architecture.md](./copilot-instructions-architecture.md) to understand the flow
2. Implement backend following [copilot-instructions-rust.md](./copilot-instructions-rust.md)
3. Implement frontend following [copilot-instructions-react.md](./copilot-instructions-react.md)

## Key Technologies and Patterns

### Backend (Rust)
- **Framework**: Axum 0.8.x
- **ORM**: SeaORM 1.1.x (schema-first approach)
- **Async Runtime**: Tokio with JoinSet for concurrency
- **Database**: PostgreSQL
- **AI Services**: OpenAI GPT-4o-mini, Qdrant vector database
- **Patterns**: Command pattern, dependency injection, async/await

### Frontend (React)
- **Framework**: React 19.x
- **UI Library**: DaisyUI 5.x with Tailwind CSS 4.x
- **Forms**: React Hook Form + Zod validation
- **Rich Text**: TipTap editor
- **GraphQL**: Apollo Client
- **Authentication**: Keycloak JS
- **Patterns**: Container/presentational, form composition

## Code Quality Standards

All code should follow these principles:

1. ✅ **Type Safety**: Use strong typing (Rust types, TypeScript strict mode)
2. ✅ **Error Handling**: Explicit error handling with Result types and try-catch
3. ✅ **Async Best Practices**: Proper async/await usage, avoid blocking operations
4. ✅ **Testing**: Unit tests for business logic, integration tests for workflows
5. ✅ **Documentation**: Clear comments for complex logic
6. ✅ **Consistency**: Follow existing patterns and naming conventions

## Special Features Documentation

### AI Translation Service
The project includes a sophisticated AI translation service with:
- **3-Tier Lookup**: Database cache → Vector similarity (Qdrant) → OpenAI translation
- **Cost Optimization**: Reuse similar translations (≥95% similarity)
- **Parallel Processing**: Use JoinSet to translate multiple chunks concurrently
- **Background Execution**: Non-blocking translation for large content

See [Architecture Guide](./copilot-instructions-architecture.md#ai-translation-service) for detailed implementation.

### Concurrency Patterns (Rust)
- Use `JoinSet` for parallel operations that need to be joined
- Use `tokio::spawn` for background tasks
- Use `Arc<T>` for shared ownership across async tasks
- Avoid blocking operations in async contexts

See [Rust Guide](./copilot-instructions-rust.md#async-and-concurrency) for examples.

### Form Patterns (React)
- Use React Hook Form for all forms
- Define Zod schemas for validation
- Use `useFieldArray` for dynamic fields
- Show validation errors inline

See [React Guide](./copilot-instructions-react.md#form-handling) for examples.

## Contributing

When adding new features:

1. **Follow existing patterns** documented in these guides
2. **Update documentation** if introducing new patterns
3. **Write tests** for new functionality
4. **Use consistent naming** across frontend and backend
5. **Add comments** for complex business logic

## Resources

- [Rust Documentation](https://doc.rust-lang.org/)
- [Tokio Documentation](https://tokio.rs/)
- [SeaORM Documentation](https://www.sea-ql.org/SeaORM/)
- [Axum Documentation](https://docs.rs/axum/)
- [React Documentation](https://react.dev/)
- [DaisyUI Components](https://daisyui.com/components/)
- [React Hook Form](https://react-hook-form.com/)
- [Zod Documentation](https://zod.dev/)

---

## File Statistics

| File | Lines | Focus |
|------|-------|-------|
| copilot-instructions.md | 206 | Project overview, quick start |
| copilot-instructions-rust.md | 729 | Rust patterns, async/concurrency |
| copilot-instructions-react.md | 988 | React components, DaisyUI, forms |
| copilot-instructions-architecture.md | 777 | System design, AI service, deployment |
| **Total** | **2,700** | **Complete coding guidelines** |

## Maintenance

These documents should be updated when:
- New architectural patterns are introduced
- Technology versions are upgraded
- Coding standards change
- New best practices are established

Last Updated: January 2026
