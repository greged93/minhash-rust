use anyhow::Error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Grid {
    pub x: u16,
    pub y: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MechState {
    id: String,
    typ: String,
    status: String,
    pub index: Grid,
    pc_next: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct OperatorType {
    pub symbol: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Operator {
    pub input: Vec<Grid>,
    pub output: Vec<Grid>,
    pub typ: OperatorType,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Input {
    pub mechs: Vec<MechState>,
    pub programs: Vec<String>,
    pub operators: Vec<Operator>,
}

pub fn load_input(path: &str) -> Result<Input, Error> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    let input: Input = serde_json::from_reader(buf)?;
    Ok(input)
}

pub fn get_instructions(program: &[String]) -> Vec<u16> {
    let p: String = program.join(",");
    p.as_bytes()
        .into_iter()
        .map(|&x| mapping_program(&[x; 1]))
        .filter(|&x| x < 9)
        .collect()
}

pub fn mapping_program(x: &[u8; 1]) -> u16 {
    match x {
        b"W" => 1,
        b"A" => 2,
        b"S" => 3,
        b"D" => 4,
        b"Z" => 5,
        b"X" => 6,
        b"G" => 7,
        b"H" => 8,
        b"_" => 9,
        _ => 10,
    }
}

pub fn mapping_operator_type(x: &str) -> u16 {
    match x.as_bytes() {
        b"&" => 0,
        b"%" => 1,
        b"^" => 2,
        b"#" => 3,
        _ => panic!("invalid operator"),
    }
}
