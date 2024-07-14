## [![SonarCloud](https://sonarcloud.io/images/project_badges/sonarcloud-white.svg)](https://sonarcloud.io/summary/new_code?id=doitsu2014_my-cms)

# Overview

## Architecture

### 1. ORM

The project using SeaORM to interact with the database. SeaORM is a modern and easy-to-use ORM for Rust.

### 2. Unit Tests and Integration Tests

For unit tests, we use built-in feature mock of SeaORM to test the database interaction. For integration tests, we use the test database to test the whole system.
For integration tests, we use testcontainers to setup whole infrastructure to make sure the system is working as expected.

## Configuration

Use .env file to configure the system.

```text
# App Host and Port
HOST=127.0.0.1
PORT=8989

# App Infrastructure
DATABASE_URL=postgresql://postgres:1234567890@localhost:5432/my-cms

OTEL_SERVICE_NAME=my-cms-headless-api
OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://localhost:4317
OTEL_TRACES_SAMPLER=always_on
```

## Cross-cutting concerns

- Jaeger

```bash
docker image pull jaegertracing/all-in-one:1.49
docker run --rm -d --name jaeger \
  -e COLLECTOR_OTLP_ENABLED:true \
  -e LOG_LEVEL:debug \
  -p 6831:6831/udp \
  -p 6832:6832/udp \
  -p 5778:5778 \
  -p 16686:16686 \
  -p 4317:4317 \
  -p 4318:4318 \
  -p 14250:14250 \
  -p 14268:14268 \
  -p 14269:14269 \
  -p 9411:9411 \
  jaegertracing/all-in-one:1.49
```
