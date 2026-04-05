defmodule ElixirService.Application do
  use Application

  @impl true
  def start(_type, _args) do
    children = [
      # 1. Start the Phoenix Endpoint (which handles the WebSocket transport)
      ElixirServiceWeb.Endpoint,

      # 2. Correctly start the PubSub server as a child process.
      #    We use the Phoenix.PubSub module directly, specifying its name and adapter.
      {Phoenix.PubSub, name: ElixirService.PubSub, adapter: Phoenix.PubSub.PG2}
    ]

    opts = [strategy: :one_for_one, name: ElixirService.Supervisor]
    Supervisor.start_link(children, opts)
  end



  # Tell Phoenix to update the endpoint configuration
  # whenever the application is updated.
  @impl true
  def config_change(changed, _new, removed) do
    ElixirServiceWeb.Endpoint.config_change(changed, removed)
    :ok
  end
end
