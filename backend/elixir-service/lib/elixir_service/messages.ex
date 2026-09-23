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
end
