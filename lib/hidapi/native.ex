defmodule Hidapi.Native do
  use Rustler, otp_app: :hidapi_rs, crate: :hidapi

  def api(), do: error()
  def devices(_api), do: error()
  # def refresh_devices(_api), do: error()
  def open(_appi, _vid, _pid), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)

end
