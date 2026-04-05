# lib/elixir_service_web/channels/user_socket.ex
defmodule ElixirServiceWeb.UserSocket do
  use Phoenix.Socket

  channel("counter:*", ElixirServiceWeb.CounterChannel)

  @impl true
  def connect(_params, socket) do
    {:ok, socket}
  end

  @impl true
  def id(_socket) do
    nil
  end
end
