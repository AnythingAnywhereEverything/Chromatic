defmodule ElixirServiceWeb.DMChannel do
  use ElixirServiceWeb, :channel

  alias Phoenix.PubSub

  @impl true
  def join("dm:" <> user_id, _params, socket) do
    current_user_id = socket.assigns.user.id
    user_id = String.to_integer(user_id)

    if user_id == current_user_id do
      PubSub.subscribe(
        ElixirService.PubSub,
        "dm:#{current_user_id}"
      )

      {:ok, socket}
    else
      {:error, :unauthorized}
    end
  end

  @impl true
  def handle_in(
        "send_message",
        %{
          "recipient_id" => recipient_id,
          "content" => content
        },
        socket
      ) do
    sender_id = socket.assigns.user.id
    recipient_id = String.to_integer(recipient_id)

    {:ok, message_id} = ElixirService.Snowflake.generate_id(:api)

    case ElixirService.Messages.send_message(
           message_id,
           sender_id,
           recipient_id,
           content
         ) do
      {:ok, _message} ->
        message = %{
          id: Integer.to_string(message_id),
          sender_id: Integer.to_string(sender_id),
          recipient_id: Integer.to_string(recipient_id),
          content: content
        }

        # * Sender always receives the successfully saved message.
        push(socket, "new_message", message)

        # * Don't broadcast to ourselves again.
        # * If the recipient is offline, PubSub simply has no subscribers.
        if recipient_id != sender_id do
          PubSub.broadcast(
            ElixirService.PubSub,
            "dm:#{recipient_id}",
            {:new_message, message}
          )
        end

        {:reply, :ok, socket}

      {:error, _changeset} ->
        {:reply, {:error, %{reason: "failed_to_send"}}, socket}
    end
  end

  @impl true
  def handle_in(
        "delete_message",
        %{"message_id" => message_id},
        socket
      ) do
    sender_id = socket.assigns.user.id
    message_id = String.to_integer(message_id)

    case ElixirService.Messages.delete_message(message_id, sender_id) do
      {:ok, message} ->
        deleted = %{
          id: Integer.to_string(message_id),
          sender_id: Integer.to_string(message.user_id),
          recipient_id: Integer.to_string(message.target_id)
        }

        # * Sender always receives the successful deletion.
        push(socket, "message_deleted", deleted)

        # * Don't broadcast to ourselves again.
        # * If the recipient is offline, PubSub simply has no subscribers.
        if message.target_id != sender_id do
          PubSub.broadcast(
            ElixirService.PubSub,
            "dm:#{message.target_id}",
            {:message_deleted, deleted}
          )
        end

        {:reply, :ok, socket}

      {:error, :not_found} ->
        {:reply, {:error, %{reason: "not_found"}}, socket}

      {:error, :unauthorized} ->
        {:reply, {:error, %{reason: "unauthorized"}}, socket}

      {:error, _changeset} ->
        {:reply, {:error, %{reason: "failed_to_delete"}}, socket}
    end
  end

  @impl true
  @spec handle_info(
          {:new_message, any()},
          Phoenix.Socket.t()
        ) :: {:noreply, Phoenix.Socket.t()}
  def handle_info({:new_message, message}, socket) do
    IO.inspect(message, label: "RECEIVED MESSAGE")

    push(socket, "new_message", message)

    {:noreply, socket}
  end

  @impl true
  def handle_info({:message_deleted, deleted}, socket) do
    push(socket, "message_deleted", deleted)

    {:noreply, socket}
  end
end
