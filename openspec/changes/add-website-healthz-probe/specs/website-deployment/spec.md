## MODIFIED Requirements

### Requirement: Healthcheck endpoint

The production Ducth website server SHALL expose an unauthenticated `GET /healthz` endpoint that returns HTTP 200 when its Express process is accepting requests. The handler SHALL complete without invoking React SSR, reading the SSR template, creating an Apollo client, executing a GraphQL operation, fetching media, or calling any other external service. The runner stage SHALL declare `HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD wget --no-verbose --tries=1 --spider http://localhost:3001/healthz || exit 1`. The `/en` reader route SHALL retain its existing HTTP 200 SSR behavior for reader traffic and SHALL NOT be used as the container health-check target.

#### Scenario: Healthy container reports healthy through the shallow endpoint
- **WHEN** the container is started with valid `WEBSITE_*` env vars
- **THEN** `GET /healthz` returns HTTP 200 without making a GraphQL request
- **AND** `docker inspect --format '{{.State.Health.Status}}' <container>` returns `healthy` within the healthcheck interval

#### Scenario: Reader traffic remains SSR-backed independently of health checking
- **WHEN** a client sends `GET /en` to a healthy website process
- **THEN** the SSR handler renders the reader response and returns HTTP 200
- **AND** the Docker health check requests `/healthz` rather than `/en`

#### Scenario: Misconfigured container reports unhealthy
- **WHEN** the container is started without `WEBSITE_PUBLIC_GRAPHQL_API_URL`
- **THEN** the container exits before the healthcheck runs
- **AND** `docker ps` does not list the container in `Up` state
