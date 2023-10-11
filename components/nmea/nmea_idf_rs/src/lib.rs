#![cfg_attr(not(feature = "std"), no_std)]

use core::ffi::{c_void, c_char, CStr};
use core::panic::PanicInfo;

use nmea::{Nmea, ParseResult};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut BUFFER: [u8; 128] = [0; 128];
use core::mem::size_of;
// use esp_println::println;

static HELLO_ESP32: &'static [u8] = b"Hello ESP-RS. https://github.com/esp-rs\0";

#[no_mangle]
pub extern "C" fn hello() -> *const c_void {
    HELLO_ESP32.as_ptr() as *const c_void
}

#[no_mangle]
pub extern "C" fn nmea_size() -> u32 {
    let size = core::mem::size_of::<Nmea>();
    size as u32
}

#[no_mangle]
pub extern "C" fn nmea_gga_altitude(gga_cstr: *const c_char) -> f32 {
    if gga_cstr.is_null() {
        return 0.0;
    }

    let c_str = unsafe { CStr::from_ptr(gga_cstr) };
    let gga_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return 0.0,
    };
    match nmea::parse_str(gga_str) {
        Ok(result) => {
            match result {
                ParseResult::GGA(gga) => gga.altitude.unwrap_or(0.0),
                _ => 0.0
            }
        },
        Err(_) => 0.0
    }
}

#[repr(C)]
pub struct CGgaData {
    pub fix_hour: i32, // -1 for None
    pub fix_minute: i32, // -1 for None
    pub fix_second: i32, // -1 for None
    pub fix_type: i32, // -1 for None
    pub latitude: f64, // NaN for None
    pub longitude: f64, // NaN for None
    pub fix_satellites: i32, // -1 for None
    pub hdop: f32, // NaN for None
    pub altitude: f32, // NaN for None
    pub geoid_separation: f32, // NaN for None
}

#[no_mangle]
pub extern "C" fn parse_nmea_gga(gga_cstr: *const c_char) -> CGgaData {
    let mut c_gga_data = CGgaData {
        fix_hour: -1,
        fix_minute: -1,
        fix_second: -1,
        fix_type: -1,
        latitude: f64::NAN,
        longitude: f64::NAN,
        fix_satellites: -1,
        hdop: f32::NAN,
        altitude: f32::NAN,
        geoid_separation: f32::NAN,
    };

    if gga_cstr.is_null() {
        return c_gga_data;
    }

    let c_str = unsafe { CStr::from_ptr(gga_cstr) };
    let result = nmea::parse_str(c_str.to_str().unwrap()).unwrap();

    if let ParseResult::GGA(gga) = result {
        // Disabled, because it requires chrono which is std
        // if let Some(fix_time) = gga.fix_time {
        //     c_gga_data.fix_hour = fix_time.hour() as i32;
        //     c_gga_data.fix_minute = fix_time.minute() as i32;
        //     c_gga_data.fix_second = fix_time.second() as i32;
        // }
        c_gga_data.fix_type = gga.fix_type.map_or(-1, |ft| ft as i32);
        c_gga_data.latitude = gga.latitude.unwrap_or(f64::NAN);
        c_gga_data.longitude = gga.longitude.unwrap_or(f64::NAN);
        c_gga_data.fix_satellites = gga.fix_satellites.map_or(-1, |fs| fs as i32);
        c_gga_data.hdop = gga.hdop.unwrap_or(f32::NAN);
        c_gga_data.altitude = gga.altitude.unwrap_or(f32::NAN);
        c_gga_data.geoid_separation = gga.geoid_separation.unwrap_or(f32::NAN);
    }

    c_gga_data
}
