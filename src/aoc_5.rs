use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_5/05_01_example.txt";
// #[allow(dead_code)]
// const ACTUAL: &str = "./src/aoc_5/05_01.txt";

#[allow(dead_code)]
fn aoc_4_1(path: &str) -> i32 {
    use aoc_5::Resource::*;

    let resources = vec![Seed, Soil, Fertilizer, Water, Light, Temperature, Humidity, Location];
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

#[cfg(test)]
mod tests{
    use super::*;


}
