defmodule ElixirServiceWeb.CounterChannel do
  use Phoenix.Channel

  @impl true
  def join("counter:lobby", _params, socket) do
    # 1. Instead of immediately pushing, tell the channel process to send a message to itself.
    send(self(), :after_join)

    # 2. Allow the client to join immediately.
    {:ok, socket}
  end

  @impl true
  def join(_topic, _params, _socket) do
    {:error, %{reason: "unauthorized"}}
  end

  ## New Function to handle the message sent to self on join
  @impl true
  def handle_info(:after_join, socket) do
    # 1. Now that the socket is officially joined, it's safe to push the message.
    {:ok, count} = :persistent_term.get(:current_count, {:ok, 0})

    # 2. Push the initial count back to the joining client
    push(socket, "count_update", %{count: count})

    # 3. Continue without reply
    {:noreply, socket}
  end

  # -----------------------------------------------------
  # Existing handle_in function for 'increment' (No change needed)
  @impl true
  def handle_in("increment", _payload, socket) do
    {:ok, current_count} = :persistent_term.get(:current_count, {:ok, 0})
    new_count = current_count + 1
    :persistent_term.put(:current_count, {:ok, new_count})

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
