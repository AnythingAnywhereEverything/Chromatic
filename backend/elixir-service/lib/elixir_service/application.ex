defmodule ElixirService.Application do
  use Application

  @impl true
  def start(_type, _args) do
    redis_config = Application.get_env(:elixir_service, :redis)

    children = [
      ElixirService.Repo,
      {Phoenix.PubSub, name: ElixirService.PubSub},
      {Redix, name: :redix, host: redis_config[:host], port: redis_config[:port]},
      ElixirService.CounterServer,
      ElixirServiceWeb.Endpoint
    ]

    opts = [strategy: :one_for_one, name: ElixirService.Supervisor]
    Supervisor.start_link(children, opts)
  end

  @impl true
  def config_change(changed, _new, removed) do
    ElixirServiceWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
