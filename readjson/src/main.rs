extern crate serde_json;

use std::env;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind};

use serde_json::Value;

fn main() {
    let args: Vec<String> = env::args().collect();
    let result = match args.as_slice() {
        [_, json_file] => process(json_file, Vec::new()),
        [_, json_file, path] => process(json_file, parse_path(path)),
        _ => error(),
    };

    println!("{:?}", result);
}

fn process(json_file: &str, path_vector: Vec<&str>) -> io::Result<()> {
    let file = File::open(json_file)?;
    let mut json: &Value = &serde_json::from_reader(io::BufReader::new(file))?;

    for path_fragment in path_vector {
        match json.as_object() {
            Some(json_obj) => {
                json = json_obj.get(path_fragment).unwrap();
                continue;
            },
            _ => {},
        }
        match json.as_array() {
            Some(json_array) => {
                let index: usize = path_fragment.parse().unwrap();
                if index < json_array.len() {
                    json = &json_array[index];
                    continue;
                } else {
                    return Err(Error::from(ErrorKind::NotFound));
                }
            },
            _ => {},
        }

        return Err(Error::from(ErrorKind::NotFound));
    }

    println!("{}", json);

    Ok(())
}

fn parse_path(path: &str) -> Vec<&str> {
    return path.split(".").collect();
}

fn error() -> io::Result<()> {
    Err(Error::from(ErrorKind::InvalidInput))
}
