# Dockerfile - Elixir Phoenix production build

# === Stage 1: Build the release ===
FROM elixir:1.15-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    build-base \
    git \
    nodejs \
    npm

# Set build environment
ENV MIX_ENV=prod

WORKDIR /app

# Install Hex and Rebar (Elixir build tools)
RUN mix local.hex --force && \
    mix local.rebar --force

# Install Elixir dependencies first for layer caching
COPY mix.exs mix.lock ./
RUN mix deps.get --only prod

# Copy compile-time config before compiling dependencies
RUN mkdir config
COPY config/config.exs config/prod.exs ./config/
RUN mix deps.compile

# Install and build frontend assets
COPY assets/package.json assets/package-lock.json ./assets/
RUN npm --prefix assets ci

# Copy the rest of the application source
COPY priv ./priv
COPY lib ./lib

# Compile the application
RUN mix compile

# Build frontend assets (CSS, JavaScript)
COPY assets ./assets
RUN mix assets.deploy

# Copy runtime config and release templates
COPY config/runtime.exs ./config/
COPY rel ./rel

# Build the release
RUN mix release

# === Stage 2: Runtime image ===
FROM alpine:3.19

# Install runtime dependencies
# libstdc++ is needed for the BEAM VM
# openssl is needed for crypto operations
# ncurses is needed for the remote console
RUN apk add --no-cache \
    libstdc++ \
    openssl \
    ncurses-libs \
    curl

# Create a non-root user
RUN addgroup -S elixir && adduser -S elixir -G elixir

WORKDIR /app

# Copy the release from the build stage
COPY --from=builder --chown=elixir:elixir /app/_build/prod/rel/my_app ./

USER elixir

# Expose the Phoenix port
EXPOSE 4000

# Health check against the Phoenix endpoint
HEALTHCHECK --interval=15s --timeout=5s --retries=3 --start-period=15s \
  CMD curl -f http://localhost:4000/health || exit 1

# Set environment variables for the release
ENV PHX_HOST=localhost
ENV PORT=4000

# Start the Phoenix server
CMD ["bin/chromatic", "start"]