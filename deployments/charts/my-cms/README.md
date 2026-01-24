# my-cms Helm Chart

An umbrella Helm chart for deploying the complete my-cms system, including:

- **my-cms-api**: Backend Rust API server (Axum + SeaORM)
- **my-cms-admin**: Admin UI React SPA served by nginx

## Prerequisites

- Kubernetes 1.19+
- Helm 3.2.0+
- PV provisioner support in the underlying infrastructure (if persistence is required)

## Installation

### Add dependencies

Before installing, update the chart dependencies:

```bash
cd deployments/charts/my-cms
helm dependency update
```

### Install the chart

```bash
# Install with default values
helm install my-cms ./deployments/charts/my-cms -n <namespace>

# Install with custom values
helm install my-cms ./deployments/charts/my-cms -n <namespace> -f custom-values.yaml
```

### Upgrade the chart

```bash
helm upgrade my-cms ./deployments/charts/my-cms -n <namespace> -f custom-values.yaml
```

## Configuration

The following table lists the configurable parameters. See `values.yaml` for full details.

### Global Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `global.imagePullSecrets` | Global image pull secrets | `[]` |

### my-cms-api Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `my-cms-api.enabled` | Enable the API deployment | `true` |
| `my-cms-api.replicaCount` | Number of API replicas | `1` |
| `my-cms-api.image.repository` | API image repository | `registry.hub.docker.com/doitsu2014/my-cms` |
| `my-cms-api.image.tag` | API image tag | `latest` |
| `my-cms-api.service.port` | API service port | `5000` |
| `my-cms-api.ingress.enabled` | Enable API ingress | `false` |

### my-cms-admin Configuration

| Parameter | Description | Default |
|-----------|-------------|---------|
| `my-cms-admin.enabled` | Enable the Admin deployment | `true` |
| `my-cms-admin.adminSide.enabled` | Enable Admin Side | `true` |
| `my-cms-admin.adminSide.replicaCount` | Number of Admin replicas | `1` |
| `my-cms-admin.adminSide.image.repository` | Admin image repository | `doitsu2014/my-cms-admin` |
| `my-cms-admin.adminSide.image.tag` | Admin image tag | `latest` |
| `my-cms-admin.adminSide.appConfig.*` | Runtime config for Keycloak and API URLs | See values.yaml |
| `my-cms-admin.adminSide.ingress.enabled` | Enable Admin ingress | `false` |

## Example: Production Configuration

```yaml
my-cms-api:
  enabled: true
  replicaCount: 2
  image:
    tag: "v1.0.0"
  ingress:
    enabled: true
    className: nginx
    hosts:
      - host: api.example.com
        paths:
          - path: /
            pathType: Prefix
            backendServicePort: 5000
    tls:
      - secretName: api-tls
        hosts:
          - api.example.com
  resources:
    limits:
      cpu: 500m
      memory: 512Mi
    requests:
      cpu: 100m
      memory: 256Mi

my-cms-admin:
  enabled: true
  adminSide:
    replicaCount: 2
    image:
      tag: "v1.0.0"
    appConfig:
      keycloakUrl: "https://auth.example.com"
      keycloakRealm: "my-realm"
      keycloakClientId: "my-cms-admin"
      graphqlApiUrl: "https://api.example.com/graphql"
      restApiUrl: "https://api.example.com"
    ingress:
      enabled: true
      className: nginx
      hosts:
        - host: admin.example.com
          paths:
            - path: /
              pathType: Prefix
      tls:
        - secretName: admin-tls
          hosts:
            - admin.example.com
```

## Uninstalling the Chart

```bash
helm uninstall my-cms -n <namespace>
```
