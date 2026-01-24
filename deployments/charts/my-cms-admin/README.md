# my-cms-admin Helm Chart

Helm chart for deploying my-cms-admin applications (Admin Side and Client Side React SPAs served by nginx with runtime configuration).

## Quick Start

```bash
cd deployments/charts

# Install with default values (admin side only)
helm install my-cms-admin ./my-cms-admin --namespace my-cms-admin --create-namespace

# Install with custom values
helm install my-cms-admin ./my-cms-admin -f values-prod.yaml --namespace my-cms-admin

# Upgrade
helm upgrade my-cms-admin ./my-cms-admin -f values-prod.yaml --namespace my-cms-admin

# Uninstall
helm uninstall my-cms-admin --namespace my-cms-admin
```

## Configuration

### Components

The chart supports two components that can be enabled/disabled independently:

| Component           | Default | Description                           |
| ------------------- | ------- | ------------------------------------- |
| `adminSide.enabled` | `true`  | Admin panel for managing blog content |

### Runtime App Config

Both components use ConfigMaps to inject runtime configuration via `/usr/share/nginx/html/config.js`.

#### Admin Side Config

```yaml
adminSide:
  appConfig:
    keycloakUrl: "https://auth.example.com"
    keycloakRealm: "myrealm"
    keycloakClientId: "admin-client"
    keycloakScope: "my-headless-cms-api-all email openid profile"
    graphqlApiUrl: "https://api.example.com/graphql"
    graphqlCacheApiUrl: ""
    restApiUrl: "https://api.example.com"
    mediaUploadApiUrl: ""
```

### Example `values-prod.yaml`

```yaml
# Shared credentials
imageCredentials:
  registry: hub.docker.com
  username: doitsu2014
  password: your-password
  email: your-email@example.com

# Admin Side Configuration
adminSide:
  enabled: true
  replicaCount: 2

  image:
    repository: doitsu2014/my-cms-admin
    pullPolicy: Always
    tag: v1.0.0

  appConfig:
    keycloakUrl: "https://auth.example.com"
    keycloakRealm: "production"
    keycloakClientId: "admin-client"
    keycloakScope: "my-headless-cms-api-all email openid profile"
    graphqlApiUrl: "https://api.example.com/graphql"
    restApiUrl: "https://api.example.com"

  ingress:
    enabled: true
    className: nginx
    annotations:
      nginx.ingress.kubernetes.io/proxy-body-size: "8m"
    hosts:
      - host: admin.example.com
        paths:
          - path: /
            pathType: Prefix
    tls:
      - secretName: admin-tls
        hosts:
          - admin.example.com

  resources:
    limits:
      cpu: 200m
      memory: 128Mi
    requests:
      cpu: 100m
      memory: 64Mi
```

## Health Checks

Both nginx containers expose `/health` endpoint for liveness and readiness probes.

## Chart Structure

```
my-cms-admin/
├── Chart.yaml
├── values.yaml
├── .helmignore
└── templates/
    ├── _helpers.tpl
    ├── NOTES.txt
    ├── secret.yaml                    # Shared image pull secret
    ├── configmap-admin-side.yaml
    ├── configmap-client-side.yaml
    ├── deployment-admin-side.yaml
    ├── deployment-client-side.yaml
    ├── service-admin-side.yaml
    ├── service-client-side.yaml
    ├── ingress-admin-side.yaml
    ├── ingress-client-side.yaml
    ├── hpa-admin-side.yaml
    ├── hpa-client-side.yaml
    └── tests/
        └── test-connection.yaml
```
