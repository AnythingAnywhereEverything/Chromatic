defmodule ElixirService.Auth.Session do
  use Ecto.Schema

  @primary_key {:id, :integer, autogenerate: false}

  schema "sessions" do
    field(:user_id, :integer)
    field(:session_hashed, :string)
    field(:user_agent, :string)
    field(:ip_address, :string)
    field(:created_at, :utc_datetime_usec)
  end
end
