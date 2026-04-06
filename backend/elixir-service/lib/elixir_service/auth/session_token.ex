defmodule ElixirService.Auth.SessionToken do
  @enforce_keys [:user_id, :timestamp, :random_string, :full_token]
  defstruct [:user_id, :timestamp, :random_string, :full_token]

  @type t :: %__MODULE__{
          user_id: integer(),
          timestamp: integer(),
          random_string: String.t(),
          full_token: String.t()
        }

  @spec parse(String.t()) :: {:ok, t()} | {:error, :invalid_session}
  def parse(token) when is_binary(token) do
    case String.split(token, ".", parts: 3) do
      [encoded_user_id, encoded_timestamp, random_string] ->
        with {:ok, decoded_user_id} <- decode_to_string(encoded_user_id),
             {:ok, decoded_timestamp} <- decode_to_string(encoded_timestamp),
             {user_id, ""} <- Integer.parse(decoded_user_id),
             {timestamp, ""} <- Integer.parse(decoded_timestamp) do
          IO.inspect({user_id, timestamp, random_string}, label: "Parsed session token")
          {:ok,
           %__MODULE__{
             user_id: user_id,
             timestamp: timestamp,
             # * do NOT decode random part
             random_string: random_string,
             full_token: token
           }}
        else
          _ ->
            {:error, :invalid_session}
        end

      _ ->
        {:error, :invalid_session}
    end
  end

  def parse(_), do: {:error, :invalid_session}

  @spec decode_to_string(String.t()) :: {:ok, String.t()} | {:error, :invalid_session}
  defp decode_to_string(value) when is_binary(value) do
    case Base.url_decode64(value, padding: false) do
      {:ok, decoded} ->
        {:ok, decoded}

      :error ->
        case Base.decode64(value, padding: false) do
          {:ok, decoded} -> {:ok, decoded}
          :error -> {:error, :invalid_session}
        end
    end
  end
end
