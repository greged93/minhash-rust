mod utils;
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use utils::{Grid, Input, MechState};

const MAX_INPUT: u32 = 2u32.pow(16) - 1;
const HASH_COUNT: u32 = 100;
const c: u32 = 65537;

const OFFSET_INSTRUCTIONS: u16 = 0;
const OFFSET_MECHS: u16 = 2048;
const OFFSET_OPERATORS: u16 = 4096;

fn min_hash(d1: &[u16], d2: &[u16], a: &[u32], b: &[u32]) -> u16 {
    let h1: Vec<u16> = a
        .into_iter()
        .zip(b.into_iter())
        .into_iter()
        .map(|(&x, &y)| hash(d1, x, y).unwrap_or_default())
        .collect();
    let h2: Vec<u16> = a
        .into_iter()
        .zip(b.into_iter())
        .into_iter()
        .map(|(&x, &y)| hash(d2, x, y).unwrap_or_default())
        .collect();
    let mut equal: u16 = 0;
    h1.into_iter().zip(h2.into_iter()).for_each(|(x, y)| {
        if x == y {
            equal += 1
        }
    });
    equal
}

fn hash(data: &[u16], a: u32, b: u32) -> Option<u16> {
    data.into_iter()
        .map(|&x| ((a * x as u32 + b) % c) as u16)
        .min()
}

pub fn map_unique_values(data: &[u16], global_offset: u16, local_offset: u16) -> Vec<u16> {
    let mut map: HashMap<u16, u16> = HashMap::new();
    data.iter()
        .map(|&x| {
            let entry = *map.entry(x).or_insert(0);
            *map.entry(x).or_insert(0) += 1;
            global_offset + entry + x * local_offset
        })
        .collect()
}

pub fn map_unique_grid(grids: &[Grid], global_offset: u16, offset_increase: u16) -> Vec<u16> {
    let mut map: HashMap<u16, u16> = HashMap::new();
    grids
        .iter()
        .map(|x| {
            let entry = *map.entry(x.x * 256 + x.y * 32).or_insert(0);
            *map.entry(x.x * 256 + x.y * 32).or_insert(0) += offset_increase;
            global_offset + entry + x.x * 256 + x.y * 32
        })
        .collect()
}

pub fn map_unique_operator(grids: &[(Vec<Grid>, u16)], global_offset: u16) -> Vec<u16> {
    let mut output: Vec<u16> = vec![];
    grids.into_iter().for_each(|(g, t)| {
        g.into_iter()
            .for_each(|x| output.push(global_offset + t * 256 + x.x * 16 + x.y));
    });
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{get_instructions, load_input, mapping_operator_type};
    const PATH: &str = "./data/solution_1.json";

    fn map_solution(path: &str) -> Vec<u16> {
        let data = load_input(path).unwrap();
        // assemble data
        let instructions = get_instructions(&data.programs);
        let grids_mechs: Vec<Grid> = data.clone().mechs.into_iter().map(|x| x.index).collect();
        let operators_types_it = data
            .clone()
            .operators
            .into_iter()
            .map(|x| mapping_operator_type(&x.typ.symbol));
        let grids_operators: Vec<(Vec<Grid>, u16)> = data
            .clone()
            .operators
            .into_iter()
            .zip(operators_types_it)
            .map(|(x, t)| ([x.input, x.output].concat(), t))
            .collect();
        // apply mappings
        let instructions_mapping = map_unique_values(&instructions, OFFSET_INSTRUCTIONS, 128);
        let mechs_mapping = map_unique_grid(&grids_mechs, OFFSET_MECHS, 1);
        let operators_mapping = map_unique_operator(&grids_operators, OFFSET_OPERATORS);
        [instructions_mapping, mechs_mapping, operators_mapping].concat()
    }

    fn get_a_b() -> (Vec<u32>, Vec<u32>) {
        let mut rng = thread_rng();
        (
            (0..HASH_COUNT)
                .into_iter()
                .map(|_| rng.gen_range(1..MAX_INPUT))
                .collect(),
            (0..HASH_COUNT)
                .into_iter()
                .map(|_| rng.gen_range(1..MAX_INPUT))
                .collect(),
        )
    }

    #[test]
    fn test_map_unique_values() {
        let data = load_input(PATH).unwrap();
        let instructions = get_instructions(&data.programs);
        let mut map_unique: HashMap<u16, u16> = HashMap::new();
        let mapping = map_unique_values(&instructions, 0, 128);
        mapping
            .iter()
            .for_each(|&x| *map_unique.entry(x).or_insert(0) += 1);
        assert_eq!(
            Ok(1u16),
            map_unique.into_iter().map(|(_, v)| v).max().ok_or(0)
        );
    }

    #[test]
    fn test_map_unique_grid() {
        let data = load_input(PATH).unwrap();
        let mut map_unique: HashMap<u16, u16> = HashMap::new();
        let grids: Vec<Grid> = data.mechs.into_iter().map(|x| x.index).collect();
        let mapping = map_unique_grid(&grids, 0, 1);
        mapping
            .iter()
            .for_each(|&x| *map_unique.entry(x).or_insert(0) += 1);
        assert_eq!(
            Ok(1u16),
            map_unique.into_iter().map(|(_, v)| v).max().ok_or(0)
        );
    }

    #[test]
    fn test_map_unique_operator() {
        let data = load_input(PATH).unwrap();
        let operators_types_it = data
            .clone()
            .operators
            .into_iter()
            .map(|x| mapping_operator_type(&x.typ.symbol));
        let grids_operators: Vec<(Vec<Grid>, u16)> = data
            .operators
            .into_iter()
            .zip(operators_types_it)
            .map(|(x, t)| ([x.input, x.output].concat(), t))
            .collect();
        let mut map_unique: HashMap<u16, u16> = HashMap::new();
        let mapping = map_unique_operator(&grids_operators, 0);
        mapping
            .iter()
            .for_each(|&x| *map_unique.entry(x).or_insert(0) += 1);
        assert_eq!(
            Ok(1u16),
            map_unique.into_iter().map(|(_, v)| v).max().ok_or(0)
        );
    }

    #[test]
    fn test_map_solution_to_unique() {
        let global_mapping = map_solution(PATH);
        let mut map_unique: HashMap<u16, u16> = HashMap::new();
        global_mapping
            .iter()
            .for_each(|&x| *map_unique.entry(x).or_insert(0) += 1);
        assert_eq!(
            Ok(1u16),
            map_unique.into_iter().map(|(_, v)| v).max().ok_or(0)
        );
    }

    #[test]
    fn test_hashing() {
        let (a, b) = get_a_b();
        dbg!(&a, &b);
        test_hash_same(a.clone(), b.clone());
        test_hash_similar(a.clone(), b.clone());
        test_hash_different(a, b);
    }

    fn test_hash_same(a: Vec<u32>, b: Vec<u32>) {
        let d1 = map_solution(PATH);
        let d2 = map_solution(PATH);
        let min_hash = min_hash(&d1, &d2, &a, &b);
        assert_eq!(HASH_COUNT as u16, min_hash);
    }

    fn test_hash_similar(a: Vec<u32>, b: Vec<u32>) {
        let d1 = map_solution("./data/solution_2.json");
        let d2 = map_solution("./data/solution_3.json");
        let min_hash = min_hash(&d1, &d2, &a, &b);
        assert!(min_hash >= (HASH_COUNT as f32 * 0.9) as u16);
    }

    fn test_hash_different(a: Vec<u32>, b: Vec<u32>) {
        let d1 = map_solution(PATH);
        let d2 = map_solution("./data/solution_3.json");
        let min_hash = min_hash(&d1, &d2, &a, &b);
        assert!(min_hash < (HASH_COUNT as f32 * 0.7) as u16);
    }
}
