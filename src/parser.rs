extern crate bluez;

use bytes::Buf;
use bytes::Bytes;

const PREAMBLE: &[u8; 2] = b"\x1a\x18";

pub fn is_sensor(data: &Bytes) -> bool {
    data.slice(2..4) == Bytes::from_static(PREAMBLE)
}

pub fn read_name(data: &Bytes) -> Option<String> {
    let slice = data.slice((data.len() - 10)..);
    std::str::from_utf8(slice.as_ref()).ok().map(String::from)
}

pub fn read_temperature(data: &Bytes) -> f32 {
    data.slice(10..12).get_i16() as f32 / 10.0
}

pub fn read_humidity(data: &Bytes) -> u8 {
    data.slice(12..13).get_u8()
}

pub fn read_battery(data: &Bytes) -> u8 {
    data.slice(13..14).get_u8()
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;

    const MSG_1: &[u8; 29] =
        b"\x10\x16\x1a\x18\xa4\xc18\xe1o\xb2\0\xc5/Y\x0b\xc4\\\x0b\tATC_E16FB2";
    const MSG_2: &[u8; 29] =
        b"\x10\x16\x1a\x18\xa4\xc18\x8a\xf0\xda\0\xbb3T\x0b\x97\x8a\x0b\tATC_8AF0DA";

    #[test]
    fn reading_humidity() {
        let msg1 = Bytes::from_static(MSG_1);
        assert_eq!(read_humidity(&msg1), 47);
        let msg2 = Bytes::from_static(MSG_2);
        assert_eq!(read_humidity(&msg2), 51);
    }

    #[test]
    fn reading_temperature() {
        let msg1 = Bytes::from_static(MSG_1);
        assert_eq!(read_temperature(&msg1), 19.7);
        let msg2 = Bytes::from_static(MSG_2);
        assert_eq!(read_temperature(&msg2), 18.7);
    }

    #[test]
    fn reading_battery() {
        let msg1 = Bytes::from_static(MSG_1);
        assert_eq!(read_battery(&msg1), 89);
        let msg2 = Bytes::from_static(MSG_2);
        assert_eq!(read_battery(&msg2), 84);
    }

    #[test]
    fn reading_name() {
        let msg1 = Bytes::from_static(MSG_1);
        assert_eq!(read_name(&msg1), Some("ATC_E16FB2".into()));
        let msg2 = Bytes::from_static(MSG_2);
        assert_eq!(read_name(&msg2), Some("ATC_8AF0DA".into()));
    }
}
