defmodule ElixirServiceWeb.CounterChannel do
  use Phoenix.Channel

  @impl true
  def join("counter:lobby", _params, socket) do
    send(self(), :after_join)
    {:ok, socket}
  end

  @impl true
  def join(_topic, _params, _socket) do
    {:error, %{reason: "unauthorized"}}
  end

  @impl true
  def handle_info(:after_join, socket) do
    count =
      case :persistent_term.get(:current_count, nil) do
        nil ->
          :persistent_term.put(:current_count, 0)
          0

        value ->
          value
      end

    push(socket, "count_update", %{count: count})
    {:noreply, socket}
  end

  @impl true
  def handle_in("increment", _payload, socket) do
    current = :persistent_term.get(:current_count, 0)
    new = current + 1

    :persistent_term.put(:current_count, new)

    ElixirServiceWeb.Endpoint.broadcast!(
      "counter:lobby",
      "count_update",
      %{count: new}
    )

    {:noreply, socket}
  end
end
