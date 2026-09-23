defmodule ElixirServiceWeb.UserSocket do
  use Phoenix.Socket

  channel "counter:*", ElixirServiceWeb.CounterChannel
  channel "dm:*", ElixirServiceWeb.DMChannel

  @impl true
  def connect(%{"token" => token}, socket, _connect_info) do
    case ElixirService.Auth.validate_token(token) do
      {:ok, user} ->
        socket = assign(socket, :user, user)
        {:ok, socket}

      {:error, reason} ->
        {:error, reason}
    end
  end

  @impl true
  def connect(_params, _socket, _connect_info) do
    {:error, "Missing token"}
  end

  @impl true
  def id(_socket), do: nil
end
