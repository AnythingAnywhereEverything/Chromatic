defmodule ElixirServiceWeb.HealthController do
  use ElixirServiceWeb, :controller

  def index(conn, _params) do
    json(conn, %{ok: true})
  end
end
