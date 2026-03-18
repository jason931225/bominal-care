# =============================================================================
# Stage 1 – Base: common Node + pnpm foundation
# =============================================================================
FROM node:20-alpine AS base

# Enable corepack and activate the exact pnpm version used by the workspace
RUN corepack enable && corepack prepare pnpm@9.15.4 --activate

WORKDIR /app

# =============================================================================
# Stage 2 – Pruner: extract the minimal source subtree for one app
# =============================================================================
FROM base AS pruner

# turbo prune requires turbo to be available globally
RUN pnpm add -g turbo

COPY . .

ARG APP_NAME
RUN turbo prune @bominal-senior/${APP_NAME} --docker

# =============================================================================
# Stage 3 – Installer: install only the dependencies needed by the pruned tree
# =============================================================================
FROM base AS installer

# Copy the pruned package manifests (no source yet – maximises layer cache)
COPY --from=pruner /app/out/json/ .
COPY --from=pruner /app/out/pnpm-lock.yaml ./pnpm-lock.yaml
COPY --from=pruner /app/out/pnpm-workspace.yaml ./pnpm-workspace.yaml

RUN pnpm install --frozen-lockfile

# =============================================================================
# Stage 4 – Builder: copy full pruned source and run the Turbo build pipeline
# =============================================================================
FROM installer AS builder

COPY --from=pruner /app/out/full/ .

ARG APP_NAME
RUN pnpm turbo build --filter=@bominal-senior/${APP_NAME}

# =============================================================================
# Stage 5 – Runner: lean production image
# =============================================================================
FROM node:20-alpine AS runner

WORKDIR /app

# Create a non-root system user/group for security
RUN addgroup --system --gid 1001 nodejs \
 && adduser  --system --uid 1001 nextjs

ARG APP_NAME

# Copy the Next.js standalone server bundle
COPY --from=builder --chown=nextjs:nodejs /app/apps/${APP_NAME}/.next/standalone ./

# Copy static assets and public directory so they are served correctly
COPY --from=builder --chown=nextjs:nodejs /app/apps/${APP_NAME}/.next/static  ./apps/${APP_NAME}/.next/static
COPY --from=builder --chown=nextjs:nodejs /app/apps/${APP_NAME}/public         ./apps/${APP_NAME}/public

USER nextjs

EXPOSE 3000

# Persist APP_NAME so the CMD can reference it at runtime
ARG APP_NAME
ENV APP_NAME=${APP_NAME}

# Use shell form so ${APP_NAME} is expanded by the shell at runtime
CMD node apps/${APP_NAME}/server.js
