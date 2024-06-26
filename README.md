# Overview

## Architecture

### 1. ORM

The project using SeaORM to interact with the database. SeaORM is a modern and easy-to-use ORM for Rust.

### 2. Unit Tests and Integration Tests

For unit tests, we use built-in feature mock of SeaORM to test the database interaction. For integration tests, we use the test database to test the whole system.
For integration tests, we use testcontainers to setup whole infrastructure to make sure the system is working as expected.
