use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_5/05_01_example.txt";
// #[allow(dead_code)]
// const ACTUAL: &str = "./src/aoc_5/05_01.txt";

#[allow(dead_code)]
fn aoc_4_1(path: &str) -> u32 {
    let mut index = 0;
    let mut line_groups: Vec<Vec<Mapping>> = vec![];
    let mut seeds: String = "EMPTY".to_string();
    helper::file_lines(path)
        .map(|l| l.unwrap())
        .for_each(|s| {
            if s.contains("seeds:") {
                seeds = s;
                return;
            }
            if s.contains("map") {
                index += 1;
                return;
            }
            if !s.is_empty() {
                let mapping = create_mapping(s);
                let e = line_groups.get_mut(index);
                if e.is_none() {
                    line_groups.push(vec![mapping]);
                } else {
                    e.unwrap().push(mapping);
                }
            }
        });

    seeds.split(" ")
        .skip(1)
        .take(1) // TMP: ONLY CALCULATE FIRST SEED REMOVE THIS
        .map(|s| s.parse::<u32>().unwrap())
        .map(|i| map_resources(i, &line_groups))
        .min()
        .unwrap()
}

// Returns mapped seed value through resource group mappings
fn map_resources(i: u32, line_groups: &Vec<Vec<Mapping>>) -> u32 {
    let mut current = i;
    for line_group in line_groups {
        current = map_resource(current, &line_group);
    }
    current
}

// Returns current value mapped through single resource group mapping
fn map_resource(current: u32, line_groups: &Vec<Mapping>) -> u32 {
    for mapping in line_groups {
        if mapping.in_range(current) {
            return mapping.map(current);
        }
    }
    current
}

struct Mapping {
    dest: u32,
    source: u32,
    range: u32
}

impl Mapping {
    fn in_range(&self, value: u32) -> bool {
        value >= self.source && value < (self.source + self.range)
    }

    fn map(&self, value: u32) -> u32 {
        value
    }
}

fn create_mapping(s: String) -> Mapping {
    Mapping {
        dest: 0,
        source: 0,
        range: 0
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn aoc_4_1_test() {
        assert_eq!(1, aoc_4_1(EXAMPLE_01));
    }

    #[test]
    fn mapping_in_range_test() {
        let mapping = Mapping {
            dest: 52,
            source: 50,
            range: 48
        };
        assert!(mapping.in_range(50));
        assert!(mapping.in_range(79));
        assert!(!mapping.in_range(98));
        assert!(!mapping.in_range(49));
    }

    fn mapping_map_test() {
        let mapping = Mapping {
            dest: 50,
            source: 98,
            range: 2
        };
        assert_eq!(81, mapping.map(79));
    }
}
