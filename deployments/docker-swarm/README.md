# deployments/docker-swarm/

Self-hosted Docker Compose stacks for local development. Each component lives
in its own folder with its own compose file, env file, and reset script. All
three stacks share one external Docker network (`supabase_network`).

## Layout

```
docker-swarm/
├── bootstrap.sh         one-time: creates the shared external network
├── README.md            this file
│
├── supabase/            self-hosted Supabase platform
│   ├── docker-compose.yaml
│   ├── docker-compose.expose.yaml   optional override: exposes ports directly on host
│   ├── .env / .env.example
│   ├── reset.sh
│   └── volumes/          bind-mounted config + data (db SQL, Kong, Supavisor, secrets)
│
├── apps/                my-cms API + admin frontend + Jaeger
│   ├── docker-compose.yaml
│   ├── .env / .env.example
│   └── reset.sh
│
└── traefik/             reverse proxy (file-based routing, no Docker socket)
    ├── docker-compose.yaml
    ├── .env / .env.example
    ├── reset.sh
    └── dynamic/my-cms.yml    router/middleware/service definitions
```

## First-time setup

```bash
cd deployments/docker-swarm
./bootstrap.sh                       # creates the external supabase_network

cp supabase/.env.example supabase/.env
cp apps/.env.example     apps/.env
cp traefik/.env.example  traefik/.env   # optional — defaults work for local dev

# edit secrets in supabase/.env and apps/.env (POSTGRES_PASSWORD, JWT_SECRET, ANON_KEY, SERVICE_ROLE_KEY)
# both files must agree on these shared values

rm -rf supabase/volumes/db/data      # only on first boot — let init SQL run fresh
```

## Daily workflow

```bash
cd deployments/docker-swarm

./supabase/reset.sh                  # full reset: wipes volumes, starts Supabase + Traefik, seeds admin
./supabase/reset.sh --restart        # restart only (keep volumes, no re-seed)

./apps/reset.sh                      # full reset: stops, wipes apps volumes, starts API + frontend + Jaeger + ducth-dev-website
./apps/reset.sh --restart            # restart only
./apps/reset.sh --rebuild my-cms-api # rebuild one image and recreate its container

The `ducth-dev-website` container joins the same stack once `my-cms-api` is healthy.

./traefik/reset.sh                   # ensure proxy is up (idempotent)
./traefik/reset.sh --restart         # restart proxy
```

## Access points

| URL                              | Purpose                                       |
|----------------------------------|-----------------------------------------------|
| http://localhost                 | All four routes on port 80 (Host header matches) |
| http://localhost:8989            | my-cms API (direct)                           |
| http://localhost:3002            | Admin frontend (direct)                       |
| https://ducth.dev                 | Public reader (via Traefik Host rule)         |
| http://localhost:16686           | Jaeger UI (direct)                            |
| http://localhost:8080            | Traefik dashboard                             |

When `CMS_HOST` is set to a real hostname (e.g. `ducth.dev`), Cloudflare
terminates SSL and Traefik routes on `:80`.

## Component isolation

- **Code vs deployment**: nothing under `deployments/` is built into app images.
  The apps compose file only references `../../apps/{api,web}` as build context.
- **Supabase vs apps**: two compose files, two env files, two reset scripts.
  Either can start independently of the other as long as the network exists.
- **No direct host ports**: every compose file routes through Traefik on `:80`.
  The optional `supabase/docker-compose.expose.yaml` override is the only
  exception (activated by `EXPOSE_*_PORT` env vars in `supabase/.env`).
- **Per-component .env**: each stack owns its own secrets. Only the secrets that
  are genuinely shared (`POSTGRES_PASSWORD`, `JWT_SECRET`, `ANON_KEY`,
  `SERVICE_ROLE_KEY`) need to agree across files.