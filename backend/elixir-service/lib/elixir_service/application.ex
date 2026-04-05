defmodule ElixirService.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      ElixirService.Repo,
      {Phoenix.PubSub, name: ElixirService.PubSub},
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
