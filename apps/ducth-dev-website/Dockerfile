# syntax=docker.io/docker/dockerfile:1
FROM node:20-alpine AS deps
RUN apk add --no-cache libc6-compat
WORKDIR /app
COPY package.json pnpm-lock.yaml pnpm-workspace.yaml ./
RUN corepack enable pnpm && pnpm install --frozen-lockfile

FROM node:20-alpine AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN corepack enable pnpm && pnpm run build

FROM node:20-alpine AS runner
WORKDIR /app
RUN addgroup --system --gid 1001 nodejs && adduser --system --uid 1001 expressjs
COPY --from=builder /app/dist ./dist
COPY --from=builder /app/server.prod.mjs ./server.prod.mjs
COPY --from=builder /app/package.json ./package.json
COPY --from=deps /app/node_modules ./node_modules
RUN chown -R expressjs:nodejs /app
USER expressjs
ENV NODE_ENV=production
ENV PORT=3001
EXPOSE 3001
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 CMD wget --no-verbose --tries=1 --spider http://localhost:3001/en || exit 1
CMD ["node", "server.prod.mjs"]
