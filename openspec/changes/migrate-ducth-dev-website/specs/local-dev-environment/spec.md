# local-dev-environment Specification (delta)

## Purpose
The capability describes how the my-cms local development environment is composed, configured, and reset. This delta adds the public reader service to the apps compose, so the same Docker Swarm stack brings up `supabase`, `my-cms-api`, `my-cms-frontend`, `jaeger`, and `ducth-dev-website` on the shared `supabase_network`.

## MODIFIED Requirements

### Requirement: Two-compose bring-up on a shared external network

The repository SHALL provide the local development stack as two standalone Docker Compose files under `deployments/docker-swarm/`: `deployments/docker-swarm/supabase/docker-compose.yaml` (the Supabase stack — `db`, `supavisor`, `auth`, `rest`, `realtime`, `storage`, `imgproxy`, `meta`, `studio`, `kong`, `mailpit`) and `deployments/docker-swarm/apps/docker-compose.yaml` (the project apps — `init-wait`, `migrate`, `my-cms-api`, `my-cms-frontend`, `jaeger`, **`ducth-dev-website`**). Both files SHALL declare a Docker network `supabase_network` as `external: true` with `name: supabase_network`, and SHALL join the my-cms apps to the Supabase stack by DNS resolution of the Supabase compose's container hostnames (`db`, `auth`, `storage`, `kong`, `realtime`, `meta`, `studio`, `imgproxy`, `mailpit`, `supavisor`). The `ducth-dev-website` service SHALL declare `depends_on: my-cms-api { condition: service_started }` so it starts only after the API has bound port 8989.

#### Scenario: First-time setup
- **WHEN** a developer clones the repository, copies `deployments/docker-swarm/supabase/.env.example` to `deployments/docker-swarm/supabase/.env` and `deployments/docker-swarm/apps/.env.example` to `deployments/docker-swarm/apps/.env`, and edits the shared secrets in both files
- **AND** the developer runs `deployments/docker-swarm/bootstrap.sh` (one-time only) which creates the external `supabase_network`
- **THEN** `docker compose -f deployments/docker-swarm/supabase/docker-compose.yaml --env-file deployments/docker-swarm/supabase/.env up -d` starts the Supabase stack on the shared network
- **AND** `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d` starts the my-cms apps (including `ducth-dev-website`) on the same shared network
- **AND** every Supabase service and every apps service reports `running` or `healthy` within its healthcheck interval

#### Scenario: Restart preserves data
- **WHEN** a developer runs `docker compose -f deployments/docker-swarm/supabase/docker-compose.yaml down` followed by `docker compose -f deployments/docker-swarm/supabase/docker-compose.yaml up -d`
- **THEN** the database, uploaded files, and Mailpit data persist across restarts
- **AND** the same applies to the apps compose independently

#### Scenario: External network absent
- **WHEN** a developer runs `docker compose -f deployments/docker-swarm/supabase/docker-compose.yaml up -d` without first creating `supabase_network`
- **THEN** Compose fails with a clear error identifying the missing external network
- **AND** the error message includes the network name so the developer can create it with the documented command

## ADDED Requirements

### Requirement: Project services include the public reader

The apps compose SHALL run, alongside the existing `init-wait`, `migrate`, `my-cms-api`, `my-cms-frontend`, and `jaeger` services, a `ducth-dev-website` service. The service SHALL join `supabase_network`, SHALL declare `depends_on: my-cms-api { condition: service_started }`, and SHALL consume the `WEBSITE_*` env contract documented in `deployments/docker-swarm/apps/.env.example`. Removing the `ducth-dev-website` block from the apps compose, the `website` router from `deployments/docker-swarm/traefik/dynamic/my-cms.yml`, and the `website-compress` middleware reference SHALL be sufficient to roll the reader back without affecting the rest of the stack.

#### Scenario: Full local stack brings up the reader
- **WHEN** a developer follows the `deployments/docker-swarm/README.md` daily workflow with `deployments/docker-swarm/apps/.env` and `deployments/docker-swarm/traefik/.env` populated
- **AND** runs `docker compose -f deployments/docker-swarm/apps/docker-compose.yaml --env-file deployments/docker-swarm/apps/.env up -d`
- **THEN** the `ducth-dev-website` container reports `running` (or `healthy` after the healthcheck interval)
- **AND** `curl -s -o /dev/null -w "%{http_code}" http://ducth-dev-website:3001/en` returns `200` from a peer container on `supabase_network`
- **AND** the Traefik router for `Host(\`ducth.dev\`)` resolves to `ducth-dev-website:3001` per the dynamic file

#### Scenario: Reader rolled back independently
- **WHEN** a developer edits `deployments/docker-swarm/apps/docker-compose.yaml` to delete the `ducth-dev-website` service block
- **AND** edits `deployments/docker-swarm/traefik/dynamic/my-cms.yml` to delete the `website` router, the `website-service`, the `website-compress` middleware, and the `https://ducth.dev` entry in `api-cors.accessControlAllowOriginList`
- **AND** edits `deployments/docker-swarm/traefik/.env.example` to remove `CMS_WEBSITE_HOST`
- **THEN** the rest of the apps stack (`supabase`, `init-wait`, `migrate`, `my-cms-api`, `my-cms-frontend`, `jaeger`) brings up unchanged
- **AND** `my-cms-frontend` and `my-cms-api` are not restarted

#### Scenario: Reader waits for the API
- **WHEN** `my-cms-api` is still starting (e.g. during a cold boot)
- **THEN** `ducth-dev-website` does not start
- **AND** `docker compose ps` shows `ducth-dev-website` in a `waiting` state with reason `dependency_failed` or similar
