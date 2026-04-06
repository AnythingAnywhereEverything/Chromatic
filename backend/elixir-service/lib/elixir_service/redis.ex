defmodule ElixirService.Redis do
  def command(args) when is_list(args) do
    Redix.command(:redix, args)
  end

  def command!(args) when is_list(args) do
    Redix.command!(:redix, args)
  end

  def get(key) when is_binary(key) do
    case command(["GET", key]) do
      {:ok, nil} -> {:ok, nil}
      {:ok, value} -> {:ok, value}
      {:error, reason} -> {:error, reason}
    end
  end

  def set(key, value) when is_binary(key) and is_binary(value) do
    command(["SET", key, value])
  end

  def del(key) when is_binary(key) do
    command(["DEL", key])
  end

  def incr(key) when is_binary(key) do
    command(["INCR", key])
  end
end
