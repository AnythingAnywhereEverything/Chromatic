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
    count = ElixirService.CounterServer.get_count()
    push(socket, "count_update", %{count: count})
    {:noreply, socket}
  end

  @impl true
  def handle_in("increment", _payload, socket) do
    new_count = ElixirService.CounterServer.increment()

    ElixirServiceWeb.Endpoint.broadcast!(
      "counter:lobby",
      "count_update",
      %{count: new_count}
    )

    {:noreply, socket}
  end

  @impl true
  def handle_in(_event, _payload, socket) do
    {:noreply, socket}
  end
end
