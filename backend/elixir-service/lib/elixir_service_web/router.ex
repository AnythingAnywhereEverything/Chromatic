defmodule ElixirServiceWeb.Router do
  use ElixirServiceWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/", ElixirServiceWeb do
    pipe_through :api

    get "/health", HealthController, :index
  end
end
