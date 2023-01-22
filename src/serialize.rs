use std::io::{Seek, BufRead};
use std::{fs::File, io::Write};

use crate::Commands;

use crate::Result;

pub fn append_to_file(file: &mut File, command: Commands) -> Result<usize> {
    let command_json = serde_json::to_string(&command)? + "\n";

    // set the position to the end of the file
    let pos = file.seek(std::io::SeekFrom::End(0))?;
    
    // write the command to the file to the given position
    file.write_all(command_json.as_bytes())?;


    Ok(pos as usize)
}

pub fn read_from_file(file: &mut File, pos: usize) -> Result<Commands> {
    // read the command from the file
    let mut buf = String::new();
    file.seek(std::io::SeekFrom::Start(pos as u64))?;
    let mut reader = std::io::BufReader::new(file);
    // set the position to the given position
    reader.read_line(&mut buf)?;
    // deserialize the command
    let command: Commands = serde_json::from_str(&buf)?;
    Ok(command)
}