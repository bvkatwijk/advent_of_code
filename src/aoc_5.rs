use std::{collections::HashMap, iter::FromIterator};

use helper;
use aoc_5::Resource::*;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_5/05_01_example.txt";
// #[allow(dead_code)]
// const ACTUAL: &str = "./src/aoc_5/05_01.txt";

#[allow(dead_code)]
fn aoc_4_1(path: &str) -> u32 {
    let mut index = 0;
    let mut line_groups: Vec<Vec<String>> = vec![];
    helper::file_lines(path)
        .map(|l| l.unwrap())
        .for_each(|s| {
            if s.contains("map") {
                index += 1;
            }
            if !s.is_empty() {
                // push string into correct vector
                let e = line_groups.get_mut(index);
                if e.is_none() {
                    line_groups.push(vec![s]);
                } else {
                    e.unwrap().push(s);
                }
                    // .map_or(default, f)
                    // .or_insert(vec![])
                    // .push(s);
            }
        });

    // Correct length?
    // assert_eq!(8, line_groups.len());

    // debug print map
    // for (key, value) in &line_groups {
    //     println!("{}: {:#?}", key, value);
    // }

    line_groups.get(0)
        .unwrap()[0]
        .split(" ")
        .skip(1)
        .take(1) // TMP: ONLY CALCULATE FIRST SEED REMOVE THIS
        .map(|s| s.parse::<u32>().unwrap())
        .map(|i| map_resources(i, &line_groups))
        .min()
        .unwrap()
}

// Returns mapped seed value through resource group mappings
fn map_resources(i: u32, line_groups: &Vec<Vec<String>>) -> u32 {
    let mut current = i;
    for line_group in line_groups {
        current = map_resource(current, &line_group);
    }
    current
}

fn map_resource(current: u32, line_groups: &Vec<String>) -> u32 {
    // if line group contains line with range overlapping current

    // add diff to current

    // mapped
    current
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Resource {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

enum ResourcePair {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation
}

struct Mapping {
    dest: u32,
    source: u32,
    range: u32
}

fn from_and_to(pair: &ResourcePair) -> (Resource, Resource) {
    match pair {
        ResourcePair::SeedToSoil => (Seed, Soil),
        ResourcePair::SoilToFertilizer => (Soil, Fertilizer),
        ResourcePair::FertilizerToWater => (Fertilizer, Water),
        ResourcePair::WaterToLight => (Water, Light),
        ResourcePair::LightToTemperature => (Light, Temperature),
        ResourcePair::TemperatureToHumidity => (Temperature, Humidity),
        ResourcePair::HumidityToLocation => (Humidity, Location),
    }
}

fn mapping_header<'a>(one: &'a Resource, other: &'a Resource) -> String {
    format!("{:#?}-to-{:#?} map:", one, other)
        .to_lowercase()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn aoc_4_1_test() {
        assert_eq!(1, aoc_4_1(EXAMPLE_01));
    }

    #[test]
    fn mapping_header_test() {
        assert_eq!("fertilizer-to-water map:", mapping_header(&Resource::Fertilizer, &Resource::Water));
    }
}
