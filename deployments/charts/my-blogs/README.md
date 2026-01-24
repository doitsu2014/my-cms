# my-blogs Helm Chart

Helm chart for deploying my-blogs applications (Admin Side and Client Side React SPAs served by nginx with runtime configuration).

## Quick Start

```bash
cd deployments/charts

# Install with default values (admin side only)
helm install my-blogs ./my-blogs --namespace my-blogs --create-namespace

# Install with both admin and client sides
helm install my-blogs ./my-blogs --set clientSide.enabled=true --namespace my-blogs --create-namespace

# Install with custom values
helm install my-blogs ./my-blogs -f values-prod.yaml --namespace my-blogs

# Upgrade
helm upgrade my-blogs ./my-blogs -f values-prod.yaml --namespace my-blogs

# Uninstall
helm uninstall my-blogs --namespace my-blogs
```

## Configuration

### Components

The chart supports two components that can be enabled/disabled independently:

| Component | Default | Description |
|-----------|---------|-------------|
| `adminSide.enabled` | `true` | Admin panel for managing blog content |
| `clientSide.enabled` | `false` | Public-facing blog client |

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

#### Client Side Config

```yaml
clientSide:
  appConfig:
    graphqlApiUrl: "https://api.example.com/graphql"
    graphqlCacheApiUrl: ""
    restApiUrl: "https://api.example.com"
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
    repository: doitsu2014/my-blogs-admin-side
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

# Client Side Configuration
clientSide:
  enabled: true
  replicaCount: 3

  image:
    repository: doitsu2014/my-blogs-client-side
    pullPolicy: Always
    tag: v1.0.0

  appConfig:
    graphqlApiUrl: "https://api.example.com/graphql"
    restApiUrl: "https://api.example.com"

  ingress:
    enabled: true
    className: nginx
    hosts:
      - host: blog.example.com
        paths:
          - path: /
            pathType: Prefix
    tls:
      - secretName: blog-tls
        hosts:
          - blog.example.com

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
my-blogs/
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
