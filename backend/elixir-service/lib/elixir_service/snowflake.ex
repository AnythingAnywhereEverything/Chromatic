defmodule ElixirService.Snowflake do
  use GenServer
  import Bitwise

  @custom_epoch 1_767_225_600_000

  @worker_id_bits 10
  @kind_bits 2
  @sequence_bits 10

  @max_worker_id (1 <<< @worker_id_bits) - 1
  @max_sequence (1 <<< @sequence_bits) - 1

  @kind_shift @sequence_bits
  @worker_id_shift @sequence_bits + @kind_bits
  @timestamp_shift @sequence_bits + @kind_bits + @worker_id_bits

  # * These are the only kinds currently supported.
  @kinds %{
    api: 0,
    media: 1
  }

  def start_link(kind) do
    GenServer.start_link(
      __MODULE__,
      kind,
      name: via(kind)
    )
  end

  defp via(kind) do
    {:via, Registry, {ElixirService.Snowflake.Registry, kind}}
  end

  def generate_id(kind) do
    GenServer.call(via(kind), :generate_id)
  end

  @impl true
  def init(kind) do
    worker_id = Application.fetch_env!(:elixir_service, :worker)[:id]

    with :ok <- validate_worker_id(worker_id),
         {:ok, kind_value} <- normalize_kind(kind) do
      {:ok,
       %{
         worker_id: worker_id,
         kind: kind_value,
         last_timestamp: 0,
         sequence: 0
       }}
    end
  end

  @impl true
  def handle_call(:generate_id, _from, state) do
    case generate(state) do
      {:ok, id, new_state} ->
        {:reply, {:ok, id}, new_state}

      {:error, reason} ->
        {:reply, {:error, reason}, state}
    end
  end

  defp generate(state) do
    current_timestamp = current_timestamp()

    # ! Do not generate IDs when the system clock moves backwards.
    if current_timestamp < state.last_timestamp do
      {:error, :clock_move_backward}
    else
      generate_with_timestamp(state, current_timestamp)
    end
  end

  defp generate_with_timestamp(state, current_timestamp) do
    {timestamp, sequence} =
      cond do
        current_timestamp > state.last_timestamp ->
          # * New millisecond, reset the sequence.
          {current_timestamp, 0}

        true ->
          next_sequence(state, current_timestamp)
      end

    new_state = %{
      state
      | last_timestamp: timestamp,
        sequence: sequence
    }

    id =
      timestamp <<< @timestamp_shift |||
        state.worker_id <<< @worker_id_shift |||
        state.kind <<< @kind_shift |||
        sequence

    {:ok, id, new_state}
  end

  defp next_sequence(state, current_timestamp) do
    sequence = state.sequence + 1 &&& @max_sequence

    if sequence == 0 do
      # * 1024 IDs have already been generated in this millisecond.
      #   Wait until the clock advances before generating another one.
      timestamp = wait_for_next_millisecond(state.last_timestamp)

      {timestamp, 0}
    else
      {current_timestamp, sequence}
    end
  end

  defp current_timestamp do
    System.system_time(:millisecond) - @custom_epoch
  end

  defp wait_for_next_millisecond(last_timestamp) do
    timestamp = current_timestamp()

    if timestamp <= last_timestamp do
      # * Sleep very briefly instead of busy-spinning like the Rust version.
      Process.sleep(1)

      wait_for_next_millisecond(last_timestamp)
    else
      timestamp
    end
  end

  defp validate_worker_id(worker_id)
       when is_integer(worker_id) and
              worker_id >= 0 and
              worker_id <= @max_worker_id do
    :ok
  end

  defp validate_worker_id(worker_id) do
    {:error, {:invalid_worker_id, worker_id, @max_worker_id}}
  end

  defp normalize_kind(kind) when is_atom(kind) do
    case Map.fetch(@kinds, kind) do
      {:ok, value} ->
        {:ok, value}

      :error ->
        {:error, {:invalid_kind, kind}}
    end
  end

  defp normalize_kind(kind) do
    {:error, {:invalid_kind, kind}}
  end
end
