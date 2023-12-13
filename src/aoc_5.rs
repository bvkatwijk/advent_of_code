use std::{collections::HashMap, iter::FromIterator};

use helper;
use aoc_5::Resource::*;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_5/05_01_example.txt";
// #[allow(dead_code)]
// const ACTUAL: &str = "./src/aoc_5/05_01.txt";

#[allow(dead_code)]
fn aoc_4_1(path: &str) -> i32 {
    let mut index = 0;
    let mut line_groups: HashMap<u8, Vec<String>> = HashMap::new();
    helper::file_lines(path)
        .map(|l| l.unwrap())
        .for_each(|s| {
            println!("Processing {}", s);
            if s.contains("map") {
                index += 1;
            }

            if s.is_empty() {
                return;
            }

            let mut vec = line_groups.entry(index).or_insert(vec![]);
            vec.push(s);
        });

    // let seedToSoilMap: HashMap<u8, u8> = mapping()
    // let resources = vec![Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity, Location];
    // let mappings: HashMap<>
    // resources.windows(2)
    //     .map(|r1, r2| )

    for (key, value) in &line_groups {
        println!("{}: {:#?}", key, value);
    }

    0
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
