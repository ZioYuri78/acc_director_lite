use std::fs;
use std::io::Cursor;
use std::path;

use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};

use crate::accd_config::ACCDConfig;

pub fn read_string(cur: &mut Cursor<&Vec<u8>>) -> String {
    let lenght = cur.read_u16::<NativeEndian>().unwrap();

    let mut bytes = Vec::with_capacity(lenght as usize);
    for _i in 0..lenght {
        bytes.push(cur.read_u8().unwrap());
    }

    String::from_utf8(bytes).unwrap()
}

pub fn write_string(buffer: &mut Vec<u8>, s: &String) {
    let s = s.as_bytes();
    buffer.write_u16::<NativeEndian>(s.len() as u16).unwrap();
    buffer.append(&mut s.to_vec());
}

pub fn parse_config_file(file_path: String) -> ACCDConfig {
    let path = path::Path::new(&file_path);
    match fs::read_to_string(&path) {
        Ok(data) => {
            let mut lines = data.lines();

            ACCDConfig {
                debug_sleep_time: 10,
                protocol_version: parse_line(lines.next().unwrap(), '='),
                display_name: parse_line(lines.next().unwrap(), '='),
                connection_psw: parse_line(lines.next().unwrap(), '='),
                update_interval: parse_line(lines.next().unwrap(), '='),
                command_psw: parse_line(lines.next().unwrap(), '='),
                bind_addr: parse_line(lines.next().unwrap(), '='),
                destination_addr: parse_line(lines.next().unwrap(), '='),
            }
        }
        Err(e) => {
            println!("ERROR({}): {}", file_path, e);
            ACCDConfig::default()
        }
    }
}

fn parse_line<T>(line: &str, separator: char) -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let mut splits: Vec<&str> = line.split(separator).collect();
    splits[0] = splits[0].trim();
    splits[1] = splits[1].trim();

    match splits[1].parse::<T>() {
        Ok(value) => value,
        Err(_e) => {
            println!("{:#?}", _e);
            panic!(
                "ERROR: cannot parse string \"{}\" in config file .cfg",
                line
            )
        }
    }
}
