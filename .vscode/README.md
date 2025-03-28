# Guidelines and Rules for Copilot in My-CMS Project

## 1. General Rules
- Follow Rust's best practices for performance and safety.
- Use idiomatic Rust patterns and avoid unsafe code unless absolutely necessary.
- Ensure all code is well-documented with comments where appropriate.
- Prioritize readability and maintainability over clever or overly complex solutions.
- Follow Clean Architecture principles:
  - Separate concerns into layers (e.g., domain, application, infrastructure, and presentation).
  - Keep the domain layer independent of external frameworks or libraries.
  - Use dependency inversion to ensure that high-level modules are not dependent on low-level modules.
  - Avoid mixing business logic with infrastructure or framework-specific code.

## 2. Project-Specific Rules
- Use **SeaORM** for all database interactions. Avoid raw SQL queries unless required for performance reasons.
- Follow the **Schema First** approach for database design. Always generate entities from the schema.
- Use the `.env` file for configuration. Avoid hardcoding sensitive information like database credentials or API keys.

### GraphQL with SeaORM
- Use **async-graphql** crate for implementing GraphQL APIs.
- Define GraphQL schemas using Rust structs and derive macros provided by `async-graphql`.
- Use SeaORM entities as the data source for GraphQL resolvers.
- Avoid embedding business logic directly in GraphQL resolvers. Delegate business logic to the application or domain layer.
- Use pagination and filtering for queries that return large datasets. Implement these features using `async-graphql`'s built-in support for arguments.
- Ensure proper error handling in resolvers by returning `Result` types with meaningful error messages.
- Use `#[guard]` attributes to implement authorization checks on GraphQL fields or resolvers.
- Document all GraphQL queries, mutations, and subscriptions in the Postman Collection.

## 3. Testing Guidelines
- Write unit tests for all new functions and modules.
- Use SeaORM's built-in mock feature for unit tests involving database interactions.
- For integration tests, use **testcontainers** to set up the required infrastructure.
- Ensure all tests pass before committing changes.

## 4. CI/CD Rules
- Ensure all changes are compatible with the existing **GitHub Actions** CI/CD pipeline.
- Write meaningful commit messages to describe the changes.
- Ensure the Docker image builds successfully and passes all tests.

## 5. Code Style
- Follow the **Rustfmt** formatting rules. Run `cargo fmt` before committing.
- Use `clippy` to lint the code and fix any warnings or errors.
- Use descriptive variable and function names that clearly indicate their purpose.
- Prefer functional programming paradigms where applicable, such as using iterators, combinators (e.g., `map`, `filter`, `fold`), and avoiding mutable state when possible.
- Use `async`/`await` syntax for asynchronous programming to improve readability and maintainability.
- Ensure proper use of `tokio` runtime for asynchronous tasks. Avoid blocking calls within asynchronous contexts.
- Use structured concurrency patterns to manage tasks, such as `tokio::spawn` or `tokio::select`, and ensure proper cancellation and cleanup of tasks.

## 6. Observability
- Use **Jaeger** for tracing and ensure all API endpoints are instrumented for observability.
- Follow the configuration in the `.env` file for tracing settings.
- Ensure that the `ENABLED_OTLP_EXPORTER` flag is respected in the code.

## 7. Error Handling
- Use `Result` and `Option` types for error handling.
- Avoid panics in production code. Use proper error propagation with `?` or `map_err`.

## 8. Dependencies
- Use only well-maintained and widely-used crates.
- Regularly update dependencies to their latest stable versions.
- Avoid adding unnecessary dependencies to keep the project lightweight.

## 9. Documentation
- Document all public functions, structs, and modules using Rustdoc.
- Update the Postman Collection in the `postman_collection` folder when API changes are made.
- Maintain an up-to-date README for the project.

## 10. Security
- Avoid exposing sensitive information in logs or error messages.
- Use environment variables for credentials and secrets.
- Regularly audit the codebase for potential vulnerabilities.

By following these guidelines, we can ensure that the project remains robust, maintainable, and secure.
