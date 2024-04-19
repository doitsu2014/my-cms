[![build and test cms](https://github.com/doitsu2014/my-blogs-with-rust/actions/workflows/rust.yml/badge.svg)](https://github.com/doitsu2014/my-blogs-with-rust/actions/workflows/rust.yml)

# Overview

This project is using rust programming language to building up a website, which shows my posts about technology, and my hobbits., etc.

## Services

### 1. cms: Content Management System

I am using `axum` to build up the backend server, and `SeaORM` to interact with the database (postgresql). I am using `testcontainers-rs` to write integration tests for this api.
