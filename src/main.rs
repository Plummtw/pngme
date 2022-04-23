use std::fmt::Debug;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::str::FromStr;

use clap::{arg, command, Command};

mod chunk;
mod chunk_type;
mod png;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct ChunkNotFound {
    chunk_type: String
}

impl Display for ChunkNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.chunk_type)
    }   
}


impl std::error::Error for ChunkNotFound {
    fn description(&self) -> &str {
        "Chunk not found"
    }
}

fn read_file(file_name: &str) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_name).unwrap();

    let mut data = Vec::new();
    file.read_to_end(&mut data).unwrap();

    return Ok(data);
}

fn write_file(file_name: &str, data: &[u8]) {
    let mut file = File::create(file_name).unwrap();

    file.write_all(data).unwrap();
}

fn main() {
    let matches = command!()
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("encode")
                .about("Adds message to png")
                .arg(arg!([FILE_PATH]).required(true))
                .arg(arg!([CHUNK_TYPE]).required(true))
                .arg(arg!([MESSAGE]).required(true))
                .arg(arg!([OUTPUT_FILE]))
        )
        .subcommand(
            Command::new("decode")
                .about("Retrieve message from png")
                .arg(arg!([FILE_PATH]).required(true))
                .arg(arg!([CHUNK_TYPE]).required(true))
        )
        .subcommand(
            Command::new("remove")
                .about("Remove message from png")
                .arg(arg!([FILE_PATH]).required(true))
                .arg(arg!([CHUNK_TYPE]).required(true))
                .arg(arg!([OUTPUT_FILE]))
        )
        .subcommand(
            Command::new("print")
                .about("print chunktypes from png")
                .arg(arg!([FILE_PATH]).required(true))
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encode", sub_matches)) => {
            let file_path = sub_matches.value_of("FILE_PATH").unwrap();
            let chunk_type = sub_matches.value_of("CHUNK_TYPE").unwrap();
            let message = sub_matches.value_of("MESSAGE").unwrap();
            let output_file = match sub_matches.value_of("OUTPUT_FILE") {
                Some(file_name) => file_name,
                None => file_path
            };

            let file_data = read_file(file_path).unwrap();

            let mut png: Png = TryFrom::try_from(file_data.as_ref()).unwrap();
            let chunk = Chunk::new(ChunkType::from_str(chunk_type).unwrap(), message.to_owned().into_bytes());
            png.append_chunk(chunk);
            write_file(output_file, png.as_bytes().as_ref());
        },
        Some(("decode", sub_matches)) => {
            let file_path = sub_matches.value_of("FILE_PATH").unwrap();
            let chunk_type = sub_matches.value_of("CHUNK_TYPE").unwrap();

            let file_data = read_file(file_path).unwrap();
            let png: Png  = TryFrom::try_from(file_data.as_ref()).unwrap();
            match png.chunk_by_type(chunk_type) {
                Some(chunk) => println!("{}", String::from_utf8(chunk.data().to_owned()).unwrap()),
                None => println!("Chunk not found")
            }
        },
        Some(("remove", sub_matches)) => {
            let file_path = sub_matches.value_of("FILE_PATH").unwrap();
            let chunk_type = sub_matches.value_of("CHUNK_TYPE").unwrap();
            let output_file = match sub_matches.value_of("OUTPUT_FILE") {
                Some(file_name) => file_name,
                None => file_path
            };

            let file_data = read_file(file_path).unwrap();
            let mut png: Png  = TryFrom::try_from(file_data.as_ref()).unwrap();
            match png.remove_chunk(chunk_type) {
                Ok(_) => {
                    write_file(output_file, png.as_bytes().as_ref());
                },
                Err(e) => {
                    println!("{}", e);
                }
            }
        },
        Some(("print", sub_matches)) => {
            let file_path = sub_matches.value_of("FILE_PATH").unwrap();
            let file_data = read_file(file_path).unwrap();
            let png: Png  = TryFrom::try_from(file_data.as_ref()).unwrap();
            for chunk in png.chunks() {
                println!("{}", chunk.chunk_type());
            }
        },
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`"),
    }
}