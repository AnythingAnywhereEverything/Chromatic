defmodule ElixirService.Messages.Message do
  use Ecto.Schema

  @primary_key {:id, :integer, autogenerate: false}

  schema "messages" do
    field :user_id, :integer
    field :target_id, :integer
    field :target_type, :string
    field :content, :string
    field :has_attachment, :boolean
    field :has_reactions, :boolean
    field :created_at, :utc_datetime
    field :updated_at, :utc_datetime
    field :deleted_at, :utc_datetime
  end
end
