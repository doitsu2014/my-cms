docker run -d --name my-cms \
  -e DATABASE_SCHEMA=public \
  -e DATABASE_URL=postgresql://postgres:1234567890@localhost:5432/my-cms-2 \
  -e HOST=127.0.0.1 \
  -e PORT=8989 \
  -e OTEL_SERVICE_NAME=my-cms-headless-api \
  -e SERVICE_NAME=my-cms-headless-api \
  -e OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=http://localhost:4317 \
  -e OTEL_TRACES_SAMPLER=always_on \
  -p 8989:8989 \
  my-cms
