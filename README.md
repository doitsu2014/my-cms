[![CI Workflow](https://github.com/doitsu2014/my-cms/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/doitsu2014/my-cms/actions/workflows/ci.yml)
<br/>
[![codecov](https://codecov.io/gh/doitsu2014/my-cms/graph/badge.svg?token=7V6BYO0TJO)](https://codecov.io/gh/doitsu2014/my-cms)

# Overview

There is my-cms project, it is an api system to handle biz services of a headless CMS for my website. I choose Rust as the main programming language to build the system because of its performance and safety.
Let's see how far I can go with this project.

## Configuration

### 1. Cross-cutting concerns

- Jaeger

```bash
docker image pull jaegertracing/all-in-one:1.53
docker run --rm -d --name jaeger \
  -e COLLECTOR_OTLP_ENABLED:true \
  -e LOG_LEVEL:debug \
  -p 6831:6831/udp \
  -p 6832:6832/udp \
  -p 5778:5778 \
  -p 16686:16686 \
  -p 4317:4317\
  -p 4318:4318 \
  -p 14250:14250 \
  -p 14268:14268 \
  -p 14269:14269 \
  -p 9411:9411 \
  jaegertracing/all-in-one:1.53
```

### 2. Environment Setup

Use .env file to configure the system.

```text
DATABASE_SCHEMA=public
DATABASE_URL=postgresql://postgres:1234567890@localhost:5432/my-cms
HOST=127.0.0.1
PORT=8989

# Api Configuration
# Trace
ENABLED_OTLP_EXPORTER=false
OTEL_SERVICE_NAME=my-cms-headless-api
SERVICE_NAME=my-cms-headless-api
OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://localhost:4317
OTEL_TRACES_SAMPLER=always_on

# Request Limit
# Default: 10MB
MAX_BODY_LENGTH=10485760

# Media Config
# S3 Configuration
S3_REGION=ap-southeast-1
S3_BUCKET_NAME=
AWS_ACCESS_KEY_ID=
AWS_SECRET_ACCESS_KEY=

MEDIA_IMG_PROXY_SERVER=https://imgproxy.doitsu.tech
```

## Keys to take-away

### 1. ORM

The project using SeaORM to interact with the database. SeaORM is a modern and easy-to-use ORM for Rust.
We use Schema First approach to design the database schema and generate the code from the schema. It helps us to keep the schema and the code in sync.

The `entities` will be generated from the schema, and we can use them to interact with the database.

### 2. Unit Tests and Integration Tests

For unit tests, we use built-in feature mock of SeaORM to test the database interaction. For integration tests, we use the test database to test the whole system.
For integration tests, we use testcontainers to setup whole infrastructure to make sure the system is working as expected.

### 3. CI/CD

I use Docker to build the image and Github Actions to run the CI/CD pipeline.

## Play around

You can play around the project using Postman Collection in folder `postman_collection`.

Construct your Environment Variable before you start playing.
