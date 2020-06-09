use num_enum::TryFromPrimitive;
use serde::Serialize;
use serde_repr::Serialize_repr;
use soracom_orbit_sdk as orbit;
use std::cmp::PartialEq;
use std::convert::TryFrom;

#[repr(i32)]
#[derive(Debug)]
pub enum ErrorCode {
    Ok = 0,
    InsufficientData = -1,
    InvalidBase64 = -2,
    UnknownMode = -10,
    UnknownTimeframe = -11,
    UnknownType = -12,
    UnableToSerializeToJSON = -100,
    UnableToGetBuffer = -101,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize_repr, TryFromPrimitive)]
pub enum Mode {
    Button = 0,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize_repr, TryFromPrimitive)]
pub enum Timeframe {
    TenMinutes = 0,
    OneHour = 1,
    SixHours = 2,
    TwentyFourHours = 3,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize_repr, TryFromPrimitive)]
pub enum Type {
    Regular = 0,
    ButtonCall = 1,
    Alert = 2,
    NewMode = 3,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ButtonMode {
    mode: Mode,
    mode_text: String,
    timeframe: Timeframe,
    timeframe_text: String,
    #[serde(rename = "type")]
    type_: Type,
    type_text: String,
    battery: f32,
    temp_c_low_precision: f32,
    temp_c: f32,
    temp_f_low_precision: f32,
    temp_f: f32,
    reed_switch_state: u8,
    major_version: u8,
    minor_version: u8,
}

#[no_mangle]
pub fn uplink() -> ErrorCode {
    let buf = orbit::get_input_buffer();
    let output = process_uplink(buf);
    if let Err(e) = output {
        return e;
    }
    let output = output.unwrap();

    orbit::set_output_json(output.as_str());

    ErrorCode::Ok
}

fn process_uplink(buf: Vec<u8>) -> Result<String, ErrorCode> {
    let mode = get_mode(&buf)?;

    match mode {
        Mode::Button => parse_button_mode(mode, &buf),
    }
}

fn get_mode(data: &Vec<u8>) -> Result<Mode, ErrorCode> {
    if data.len() == 0 {
        return Err(ErrorCode::InsufficientData);
    }
    match Mode::try_from(data[0] & 0x07) {
        Ok(mode) => Ok(mode),
        Err(_) => Err(ErrorCode::UnknownMode),
    }
}

fn get_timeframe(data: &Vec<u8>) -> Result<Timeframe, ErrorCode> {
    if data.len() == 0 {
        return Err(ErrorCode::InsufficientData);
    }
    match Timeframe::try_from((data[0] & 0x18) >> 3) {
        Ok(tf) => Ok(tf),
        Err(_) => Err(ErrorCode::UnknownTimeframe),
    }
}

fn get_type(data: &Vec<u8>) -> Result<Type, ErrorCode> {
    if data.len() == 0 {
        return Err(ErrorCode::InsufficientData);
    }
    match Type::try_from((data[0] & 0x60) >> 5) {
        Ok(typ) => Ok(typ),
        Err(_) => Err(ErrorCode::UnknownType),
    }
}

fn get_battery(data: &Vec<u8>) -> Result<f32, ErrorCode> {
    if data.len() < 2 {
        return Err(ErrorCode::InsufficientData);
    }
    let b_msb = (data[0] & 0x80) >> 7;
    let b_lsb = data[1] & 0x0f;
    Ok((((b_msb * 16) + b_lsb) + 54) as f32 / 20.0)
}

fn get_temp_msb(data: &Vec<u8>) -> Result<u8, ErrorCode> {
    if data.len() < 2 {
        return Err(ErrorCode::InsufficientData);
    }
    Ok((data[1] & 0xf0) >> 4)
}

fn get_temp_lsb(data: &Vec<u8>) -> Result<u8, ErrorCode> {
    if data.len() < 3 {
        return Err(ErrorCode::InsufficientData);
    }
    Ok(data[2] & 0x3f)
}

fn get_temp_c(msb: u8, lsb: u8) -> f32 {
    (((msb * 64) + lsb) - 200) as f32 / 8.0
}

fn get_temp_f(msb: u8, lsb: u8) -> f32 {
    (get_temp_c(msb, lsb) * 1.8) + 32.0
}

fn get_reed_switch_state(data: &Vec<u8>) -> Result<u8, ErrorCode> {
    if data.len() < 3 {
        return Err(ErrorCode::InsufficientData);
    }
    Ok((data[2] & 0x40) >> 6)
}

fn get_major_version(data: &Vec<u8>) -> Result<u8, ErrorCode> {
    if data.len() < 4 {
        return Err(ErrorCode::InsufficientData);
    }
    Ok((data[3] & 0xf0) >> 4)
}

fn get_minor_version(data: &Vec<u8>) -> Result<u8, ErrorCode> {
    if data.len() < 4 {
        return Err(ErrorCode::InsufficientData);
    }
    Ok(data[3] & 0x0f)
}

fn parse_button_mode(mode: Mode, data: &Vec<u8>) -> Result<String, ErrorCode> {
    let timeframe = get_timeframe(data)?;
    let typ = get_type(data)?;
    let battery = get_battery(data)?;
    let temp_msb = get_temp_msb(data)?;
    let temp_lsb = get_temp_lsb(data)?;
    let temp_c_low_precision = get_temp_c(temp_msb, 0);
    let temp_f_low_precision = get_temp_f(temp_msb, 0);
    let temp_c = get_temp_c(temp_msb, temp_lsb);
    let temp_f = get_temp_f(temp_msb, temp_lsb);
    let reed_switch_state = get_reed_switch_state(data)?;
    let major_version = get_major_version(data)?;
    let minor_version = get_minor_version(data)?;

    let button_mode = ButtonMode {
        mode,
        mode_text: format!("{:?}", mode),
        timeframe,
        timeframe_text: format!("{:?}", timeframe),
        type_: typ,
        type_text: format!("{:?}", typ),
        battery,
        temp_c_low_precision,
        temp_c,
        temp_f_low_precision,
        temp_f,
        reed_switch_state,
        major_version,
        minor_version,
    };

    let j = serde_json::to_string(&button_mode);
    if let Err(e) = j {
        orbit::log(format!("error while serializing button mode to json: {}\n", e).as_str());
        return Err(ErrorCode::UnableToSerializeToJSON);
    }
    let j = j.unwrap();
    return Ok(j);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
