# Overview

## 1. Testing helm charts

Change Directory: `cd deployments/charts`

```bash
# Run helm uninstall charts, if you did install it before
helm uninstall my-cms-api

# Run helm install charts
helm install my-cms-api ./my-cms-api --namespace test
helm install my-cms-api my-cms-api -f ./my-cms-api/secret.values.yaml --namespace test

# Note: please prepare namespace `test` before running helm install
```

There is sample secret values `secret.values.yaml`

```yaml
replicaCount: 1

image:
  repository: registry.hub.docker.com/doitsu2014/my-cms
  pullPolicy: Always
  tag: 1.0.10

nameOverride: ""
fullnameOverride: ""
env: []
podAnnotations: {}
serviceAnnotations: {}

podSecurityContext:
  {}
  # fsGroup: 2000

securityContext:
  {}
  # capabilities:
  #   drop:
  #   - ALL
  # readOnlyRootFilesystem: true
  # runAsNonRoot: true
  # runAsUser: 1000

service:
  type: ClusterIP
  port: 8989
  ports:
    - port: 8989
      targetPort: http
      protocol: TCP
      name: http

container_ports:
  - name: http
    containerPort: 8989
    protocol: TCP

ingress:
  enabled: true
  className: "nginx"
  annotations:
    nginx.ingress.kubernetes.io/use-regex: "true"
    # nginx.ingress.kubernetes.io/backend-protocol: "HTTPS"
    nginx.ingress.kubernetes.io/proxy-buffering: "on"
    nginx.ingress.kubernetes.io/proxy-body-size: "8m"
    nginx.ingress.kubernetes.io/proxy-buffer-size: "1m"
    nginx.ingress.kubernetes.io/proxy-buffers-number: "12"
    nginx.ingress.kubernetes.io/client-body-buffer-size: 1m
    nginx.ingress.kubernetes.io/affinity: "cookie"
  hosts:
    - host: my-cms-api.doitsu.tech
      paths:
        - path: /
          pathType: Prefix
          backendServicePort: 5000
  tls:
    []
    # - secretName: tls-secret
    #   hosts:
    #     - chart-example.local

  # For api gateway
  # enabled: false
  # className: ""
  # annotations:
  #   nginx.ingress.kubernetes.io/use-regex: "true"
  #   nginx.ingress.kubernetes.io/client-body-buffer-size: 4m
  #   nginx.ingress.kubernetes.io/proxy-body-size: 8m
  #   nginx.ingress.kubernetes.io/rewrite-target: "/prefix/$2"
  #   nginx.ingress.kubernetes.io/app-root: "/prefix"
  # hosts:
  #   - host: chart-example.local
  #     paths:
  #       - path: /prefix(/|$)(.*)
  #         pathType: Prefix
  #         backendServicePort: 5000
  # tls: []
  # - secretName: tls-secret
  #   hosts:
  #     - chart-example.local

container_readinessProbe: {}
#   httpGet:
#     path: /healthz
#     port: http

container_livenessProbe:
  {}
  # httpGet:
  #   path: /healthz
  #   port: http
  # initialDelaySeconds: 15
  # periodSeconds: 20

secretData:
  "DATABASE_SCHEMA": "public"
  "DATABASE_URL": "postgresql://changeme:changeme@changeme:5432/changeme"
  "HOST": "0.0.0.0"
  "PORT": "8989"
  "ENABLED_OTLP_EXPORTER": false
  "OTEL_SERVICE_NAME": "my-headless-cms-api"
  "SERVICE_NAME": "my-headless-cms-api"
  "OTEL_EXPORTER_OTLP_TRACES_ENDPOINT": "http://jaeger-cluster-collector.infrastructure:4317"
  "OTEL_TRACES_SAMPLER": "always_on"
  "MAX_BODY_LENGTH": "10485760"
  "S3_REGION": "ChangeMe"
  "S3_BUCKET_NAME": "ChangeMe"
  "AWS_ACCESS_KEY_ID": "ChangeMe"
  "AWS_SECRET_ACCESS_KEY": "ChangeMe"
  "MEDIA_IMG_PROXY_SERVER": "changeme"

resources:
  {}
  # We usually recommend not to specify default resources and to leave this as a conscious
  # choice for the user. This also increases chances charts run on environments with little
  # resources, such as Minikube. If you do want to specify resources, uncomment the following
  # lines, adjust them as necessary, and remove the curly braces after 'resources:'.
  # limits:
  #   cpu: 100m
  #   memory: 128Mi
  # requests:
  #   cpu: 100m
  #   memory: 128Mi

autoscaling:
  enabled: false
  minReplicas: 1
  maxReplicas: 100
  targetCPUUtilizationPercentage: 80
  # targetMemoryUtilizationPercentage: 80

nodeSelector: {}
tolerations: []
affinity: {}
```
