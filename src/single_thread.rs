use crate::storage::Storage;
use std::error::Error;
use std::fmt::Write;
use std::io::BufRead;
use std::time::Instant;

pub fn run<R: BufRead>(reader: &mut R) -> Option<Box<dyn Error>> {
    let mut storage = Storage::new();
    let separator = b';';
    let eol = b'\n';
    let mut marker = separator;
    let mut city = String::with_capacity(1024);
    let mut buf: Vec<u8> = Vec::with_capacity(512);

    loop {
        let result = reader.read_until(marker, &mut buf);
        match result {
            Ok(size) => {
                if size == 0 {
                    break;
                }
                // read_until also includes the delimeter. Parse one char less
                let value = std::str::from_utf8(&buf[..size - 1]).unwrap();
                if marker == separator {
                    city.write_str(value);
                    marker = eol;
                } else {
                    let temperature: f64 = value.parse().unwrap();
                    marker = separator;

                    storage.add(city.clone(), temperature);
                    city.clear();
                }
                buf.clear();
            }
            Err(err) => break,
        }
    }

    println!("{}", storage);

    None
}
