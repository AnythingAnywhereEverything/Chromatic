defmodule ElixirService.Repo do
  use Ecto.Repo,
    otp_app: :elixir_service,
    adapter: Ecto.Adapters.Postgres
end
