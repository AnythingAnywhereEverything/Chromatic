FROM elixir:1.20 AS builder

WORKDIR /app

COPY mix.exs mix.lock ./
RUN mix deps.get

COPY . .
RUN mix compile


FROM elixir:1.20

WORKDIR /app

RUN mix local.hex --force && \
    mix local.rebar --force

COPY --from=builder /app .

EXPOSE 4000

CMD ["mix", "phx.server"]