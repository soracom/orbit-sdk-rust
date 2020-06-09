#[macro_use]
extern crate serde_derive;
use soracom_orbit_sdk as orbit;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[repr(i32)]
#[derive(Debug)]
pub enum ErrorCode {
    Ok = 0,
    ExecError = -1,
}

/*
uplink() processes input data like the following:
{
    "clickType":1,
    "clickTypeName":"SINGLE",
    "batteryLevel":1,
    "binaryParserEnabled":true
}
*/

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    click_type: i32,
    click_type_name: String,
    battery_level: f64,
    binary_parser_enabled: bool,
}

/*
output with some data from tags and source:
{
    "clickType":1,
    "clickTypeName":"SINGLE",
    "batteryLevel":1,
    "binaryParserEnabled":true
    "imsi":"xxxxxxxxxxxxxxx",
    "name":"name of button"
    "location":{
        "lat":35.12345,
        "lon":138.12345,
    }
}
*/
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Output {
    click_type: i32,
    click_type_name: String,
    battery_level: f64,
    binary_parser_enabled: bool,
    imsi: String,
    name: String,
    location: Option<orbit::Location>,
    timestamp: i64,
    userdata: String,
}

#[no_mangle]
pub fn uplink() -> ErrorCode {
    let buf = orbit::get_input_buffer();
    let output = process_uplink(buf);
    if let Err(e) = output {
        orbit::log(format!("{}", e).as_str());
        return ErrorCode::ExecError;
    }
    let output = output.unwrap();
    orbit::set_output_json(output.as_str());

    ErrorCode::Ok
}

fn process_uplink(buf: Vec<u8>) -> Result<String, Error> {
    let input: Input = serde_json::from_slice(buf.as_ref())?;

    let output = Output {
        click_type: input.click_type,
        click_type_name: input.click_type_name,
        battery_level: input.battery_level,
        binary_parser_enabled: input.binary_parser_enabled,
        imsi: orbit::get_source_value("resourceId"),
        name: orbit::get_tag_value("name"),
        location: orbit::get_location(),
        timestamp: orbit::get_timestamp(),
        userdata: orbit::get_userdata(),
    };

    let output_json = serde_json::to_string(&output)?;
    Ok(output_json)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
