FROM node:22-alpine AS base

ENV PNPM_HOME="/pnpm"
ENV PATH="$PNPM_HOME:$PATH"
RUN corepack enable

FROM base AS builder
RUN apk update
RUN apk add --no-cache libc6-compat
# Set working directory
WORKDIR /app
# Replace <your-major-version> with the major version installed in your repository. For example:
# RUN pnpm global add turbo@^2
RUN pnpm global add turbo@^2.3.3
COPY . .

# Generate a partial monorepo with a pruned lockfile for a target workspace.
# Assuming "api" is the name entered in the project's package.json: { name: "api" }
RUN turbo prune api --docker

# Add lockfile and package.json's of isolated subworkspace
FROM base AS installer
RUN apk update
RUN apk add --no-cache libc6-compat
WORKDIR /app
 
# First install the dependencies (as they change less often)
COPY --from=builder /app/out/json/ .
RUN pnpm install

# Build the project
COPY --from=builder /app/out/full/ .
RUN yarn turbo run build
 
FROM base AS runner
WORKDIR /app

CMD [ "pnpm", "start:prod" ]