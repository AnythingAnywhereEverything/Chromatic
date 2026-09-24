defmodule ElixirService.Messages do
  alias ElixirService.Messages.Message
  alias ElixirService.Repo

  import Ecto.Changeset

  # * Handle send message only.
  # * History will be managed by REST API.

  def send_message(message_id, sender_id, recipient_id, content) do
    %Message{}
    |> change(%{
      id: message_id,
      user_id: sender_id,
      target_id: recipient_id,
      target_type: "direct_message",
      content: content,
      has_attachment: false,
      has_reactions: false
    })
    |> Repo.insert()
  end

  def delete_message(message_id, sender_id) do
    message = Repo.get(Message, message_id)
    # also check if sender match the user_id of the message before deleting
    case message do
      nil -> {:error, :not_found}
      %Message{user_id: ^sender_id} -> Repo.delete(message)
      _ -> {:error, :unauthorized}
    end
  end

  def edit_message(message_id, new_content) do
    message = Repo.get(Message, message_id)

    case message do
      nil ->
        {:error, :not_found}

      _ ->
        message
        |> change(%{content: new_content})
        |> Repo.update()
    end
  end
end
