defmodule Hidapi.DeviceInfo do
  defstruct path: "",
            product_id: 0,
            vendor_id: 0,
            serial_number: "",
            release_number: 0,
            manufacturer_string: "",
            product_string: "",
            usage_page: 0,
            usage: 0,
            interface_number: 0

end
