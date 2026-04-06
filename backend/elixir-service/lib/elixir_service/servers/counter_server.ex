defmodule ElixirService.CounterServer do
    use GenServer

    def start_link(_opts) do
        GenServer.start_link(__MODULE__, 0, name: __MODULE__)
    end

    def get_count do
        GenServer.call(__MODULE__, :get_count)
    end

    def increment do
        GenServer.call(__MODULE__, :increment)
    end

    @impl true
    def init(initial_count) do
        {:ok, initial_count}
    end

    @impl true
    def handle_call(:get_count, _from, count) do
        {:reply, count, count}
    end

    @impl true
    def handle_call(:increment, _from, count) do
        new_count = count + 1
        {:reply, new_count, new_count}
    end
end
