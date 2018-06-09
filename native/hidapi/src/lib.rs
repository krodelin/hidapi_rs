#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rustler;
#[macro_use]
extern crate rustler_codegen;

extern crate hidapi;

use rustler::{Env, Term, Error, Encoder};
use rustler::resource::ResourceArc;

mod atoms {
    rustler_atoms! {
        atom ok;
        atom error;
        //atom __true__ = "true";
        //atom __false__ = "false";
    }
}

rustler_export_nifs! {
    "Elixir.Hidapi.Native",
    [
        ("api", 0, api),
        ("devices", 1, devices),
        // ("refresh_devices", 1, refresh_devices),
        ("open", 3, open)
    ],
    Some(on_load)
}

fn on_load(env: Env, _info: Term) -> bool {
    resource_struct_init!(Api, env);
    resource_struct_init!(Device<'a>, env);
}

struct Api(hidapi::HidApi);

impl Drop for Api {
    fn drop(&mut self) {
        println!("Drop");
    }
}

struct Device<'a>(hidapi::HidDevice<'a>);



#[derive(NifStruct)]
#[module = "Hidapi.DeviceInfo"]
struct DeviceInfo {
    pub path: String,
    pub vendor_id: u16,
    pub product_id: u16,
    pub serial_number: Option<String>,
    pub release_number: u16,
    pub manufacturer_string: Option<String>,
    pub product_string: Option<String>,
    pub usage_page: u16,
    pub usage: u16,
    pub interface_number: i32,
}


impl DeviceInfo {
    fn new(device_info: &hidapi::HidDeviceInfo) -> DeviceInfo {
        DeviceInfo {
            path: device_info.path.clone(),
            vendor_id: device_info.vendor_id,
            product_id: device_info.product_id,
            serial_number: device_info.serial_number.clone(),
            release_number: device_info.release_number,
            manufacturer_string: device_info.manufacturer_string.clone(),
            product_string: device_info.product_string.clone(),
            usage_page: device_info.usage_page,
            usage: device_info.usage,
            interface_number: device_info.interface_number,
        }
    }
}

fn format_error(error: &str) -> String {
    format!("{}", error)
}


fn api<'a>(env: Env<'a>, _args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    match hidapi::HidApi::new() {
        Ok(inner) => {
            let api = Api(inner);
            let resource = ResourceArc::new(api);
            Ok((atoms::ok(), resource).encode(env))
        }
        Err(err) => Ok((atoms::error(), format_error(err)).encode(env)),
    }
}


fn devices<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Api> = args[0].decode()?;
    let api: &hidapi::HidApi = &resource.0;

    let devices: Vec<hidapi::HidDeviceInfo> = api.devices();

    let devices: Vec<DeviceInfo> = devices.into_iter().map(|device_info: hidapi::HidDeviceInfo| DeviceInfo::new(&device_info)).collect();


    Ok((atoms::ok(), devices).encode(env))
}


fn open<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let resource: ResourceArc<Api> = args[0].decode()?;
    let api: &hidapi::HidApi = &resource.0;

    let vid: u16 = args[1].decode()? as u16;
    let pid: u16 = args[2].decode()? as u16;

    match api.open(vid, pid) {
        Ok(inner) => {
            let device = Device(inner);
            let resource = ResourceArc::new(api);
            Ok((atoms::ok(), resource).encode(env))
        }
        Err(err) => Ok((atoms::error(), format_error(err)).encode(env)),
    }
}