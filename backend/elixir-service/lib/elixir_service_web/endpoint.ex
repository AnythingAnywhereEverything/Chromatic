# lib/elixir_service_web/endpoint.ex
defmodule ElixirServiceWeb.Endpoint do
  use Phoenix.Endpoint, otp_app: :elixir_service

  # The socket is defined here
  socket("/socket", ElixirServiceWeb.UserSocket,
    websocket: true,
    longpoll: false
  )

  plug(ElixirServiceWeb.Router)
end
