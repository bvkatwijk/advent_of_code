use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_5/05_01_example.txt";
// #[allow(dead_code)]
// const ACTUAL: &str = "./src/aoc_5/05_01.txt";

#[allow(dead_code)]
fn aoc_4_1(path: &str) -> i32 {
    use aoc_5::Resource::*;
    let lines: Vec<String> = helper::file_lines(path).map(|l| l.unwrap()).collect();

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

fn mapping_header<'a>(one: &'a Resource, other: &'a Resource) -> String {
    format!("{:#?}-to-{:#?} map:", one, other)
        .to_lowercase()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn mapping_header_test() {
        assert_eq!("fertilizer-to-water map:", mapping_header(&Resource::Fertilizer, &Resource::Water));
    }
}
