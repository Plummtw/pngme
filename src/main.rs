use std::fmt::Debug;
use std::fmt::Display;

mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

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

fn main() -> Result<()> {
    todo!()
}