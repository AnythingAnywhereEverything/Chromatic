import Config
import Dotenvy

env_file_location = System.get_env("ENV_FILE")

source!([
  Path.absname(env_file_location),
  Path.absname(".#{config_env()}.env", env_file_location),
  Path.absname(".#{config_env()}.overrides.env", env_file_location),
  System.get_env()
])

# ---------- DATABASE ----------

config :elixir_service, ElixirService.Repo,
  username: env!("POSTGRES_USER", :string),
  password: env!("POSTGRES_PASSWORD", :string),
  hostname: env!("POSTGRES_HOST", :string),
  port: env!("POSTGRES_PORT", :integer),
  database: env!("POSTGRES_DB", :string),
  pool_size: env!("POSTGRES_CONNECTION_POOL", :integer)

IO.puts("Database config successfully loaded from environment variables.")
IO.puts("Database host: #{env!("POSTGRES_HOST", :string)}")

# ---------- REDIS ----------

config :elixir_service, :redis,
  host: env!("REDIS_HOST", :string) || "localhost",
  port: env!("REDIS_PORT", :integer)

IO.puts("Redis config successfully loaded from environment variables.")

# ---------- WORKER ----------

config :elixir_service, :worker, id: env!("WORKER_ID", :integer)

IO.puts("Worker config successfully loaded from environment variables.")

# ---------- PHOENIX ----------

config :elixir_service, ElixirServiceWeb.Endpoint,
  http: [
    ip: {0, 0, 0, 0},
    port: 4000
  ]
