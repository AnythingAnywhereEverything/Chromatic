defmodule ElixirService.Auth do
  import Ecto.Query

  alias ElixirService.Repo
  alias ElixirService.Redis
  alias ElixirService.Auth.Session
  alias ElixirService.Auth.SessionToken

  @session_ttl 60 * 60
  @session_extend_ttl 60 * 60

  def validate_token(token) when is_binary(token) do
    # this will run in order:
    # 1. parse the token
    # 2. convert the timestamp to datetime
    # 3. validate the session in redis or database
    with {:ok, parsed} <- SessionToken.parse(token),
         {:ok, created_at} <- millis_to_datetime(parsed.timestamp),
         {:ok, user} <- validate_session(parsed.user_id, parsed.timestamp, created_at, token) do
      {:ok, user}
    else
      _ ->
        {:error, :unauthorized}
    end
  end

  def validate_token(_), do: {:error, :unauthorized}

  defp validate_session(user_id, created_time, created_at, token) do
    key = session_cache_key(user_id, created_time, token)

    case Redis.command(["EXISTS", key]) do
      {:ok, 1} ->

        Redis.command(["EXPIRE", key, Integer.to_string(@session_extend_ttl)])

        {:ok, %{id: user_id}}

      {:ok, 0} ->

        case find_session(user_id, created_at, token) do
          {:ok, session} ->

            Redis.command([
              "SETEX",
              key,
              Integer.to_string(@session_ttl),
              "1"
            ])

            {:ok, %{id: session.user_id}}

          :error ->
            {:error, :unauthorized}
        end

      {:error, _reason} ->
        {:error, :unauthorized}
    end
  end

  defp find_session(user_id, created_at, token) do
    from(s in Session,
      where: s.user_id == ^user_id and s.created_at == ^created_at,
      limit: 5
    )
    |> Repo.all()
    |> Enum.find(fn session ->
      Argon2.verify_pass(token, session.session_hashed)
    end)
    |> case do
      nil -> :error
      session -> {:ok, session}
    end
  end

  defp session_cache_key(user_id, created_time, token) do
    token_hash =
      :crypto.hash(:sha256, token)
      |> Base.encode16(case: :lower)

    "session_active:#{user_id}:#{created_time}:#{token_hash}"
  end

  defp millis_to_datetime(timestamp_millis) when is_integer(timestamp_millis) do
    DateTime.from_unix(timestamp_millis, :millisecond)
  end
end
