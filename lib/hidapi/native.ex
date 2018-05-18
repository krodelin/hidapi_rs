defmodule Hidapi.Native do
  use Rustler, otp_app: :hidapi_rs, crate: :hidapi

  def add(_a, _b), do: error()

  defp error, do: :erlang.nif_error(:nif_not_loaded)

end
