defmodule ElixirServiceWeb do
  @moduledoc """
  The entrypoint for defining your web interface, such
  as controllers, components, channels, and so on.

  This can be used in your application as:

      use ElixirServiceWeb, :controller
      use ElixirServiceWeb, :html

  The definitions below will be executed for every controller,
  component, etc, so keep them short and clean, focused
  on imports, uses and aliases.

  Do NOT define functions inside the quoted expressions
  below. Instead, define additional modules and import
  those modules here.
  """

  # lib/elixir_service_web.ex (Modified)

  def router do
    quote do
      # Phoenix.Router is correctly used here.
      use Phoenix.Router, namespace: ElixirServiceWeb

      # You can safely remove import Plug.Conn if you are not using HTTP routing,
      # but keeping it is fine as it's defined inside the quote block.
    end
  end

  def channel do
    quote do
      use Phoenix.Channel
    end
  end

  def controller do
    quote do
      use Phoenix.Controller, formats: [:html, :json]

      use Gettext, backend: ElixirServiceWeb.Gettext

      import Plug.Conn

      unquote(verified_routes())
    end
  end

  def verified_routes do
    quote do
      use Phoenix.VerifiedRoutes,
        endpoint: ElixirServiceWeb.Endpoint,
        router: ElixirServiceWeb.Router,
        statics: ElixirServiceWeb.static_paths()
    end
  end

  @doc """
  When used, dispatch to the appropriate controller/live_view/etc.
  """
  defmacro __using__(which) when is_atom(which) do
    apply(__MODULE__, which, [])
  end
end
