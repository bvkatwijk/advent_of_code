use helper;

#[allow(dead_code)]
const EXAMPLE_01: &str = "./src/aoc_5/05_01_example.txt";
#[allow(dead_code)]
const ACTUAL: &str = "./src/aoc_5/05_01_input.txt";

#[allow(dead_code)]
fn aoc_4_1(path: &str) -> u64 {
    let (seeds, line_groups) = seeds_and_line_groups(path);
    seeds.split(" ")
        .skip(1)
        // .take(1) // TMP: ONLY CALCULATE FIRST SEED REMOVE THIS
        .map(|s| s.parse::<u64>().unwrap())
        .map(|i| map_resources(i, &line_groups))
        .min()
        .unwrap()
}

#[allow(dead_code)]
fn aoc_4_2(path: &str) -> u64 {
    let (seeds, line_groups) = seeds_and_line_groups(path);
    let seed_values: Vec<u64> = seeds.split(" ")
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    let seed_ranges: Vec<SeedRange> = (0..seed_values.len()/2).into_iter()
        .map(|i| SeedRange {
           start: *seed_values.get(i*2).unwrap(),
           range: *seed_values.get(i*2+1).unwrap() 
        })
        .collect();
    seed_ranges.iter()
        .flat_map(|r| (r.start..r.start+r.range))
        .map(|i| map_resources(i, &line_groups))
        .min()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct SeedRange {
    start: u64,
    range: u64
}

fn seeds_and_line_groups(path: &str) -> (String, Vec<Vec<Mapping>>) {
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
                let mapping = create_mapping(&s);
                let e = line_groups.get_mut(index);
                if e.is_none() {
                    line_groups.push(vec![mapping]);
                } else {
                    e.unwrap().push(mapping);
                }
            }
        });
    (seeds, line_groups)
}

// Returns mapped seed value through resource group mappings
fn map_resources(i: u64, line_groups: &Vec<Vec<Mapping>>) -> u64 {
    let mut current = i;
    for line_group in line_groups {
        current = map_resource(current, &line_group);
    }
    current
}

// Returns current value mapped through single resource group mapping
fn map_resource(current: u64, line_groups: &Vec<Mapping>) -> u64 {
    for mapping in line_groups {
        if mapping.in_range(current) {
            return mapping.map(current);
        }
    }
    current
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Mapping {
    dest: u64,
    source: u64,
    range: u64
}

impl Mapping {
    fn in_range(&self, value: u64) -> bool {
        value >= self.source && value < (self.source + self.range)
    }

    fn map(&self, value: u64) -> u64 {
        value + self.dest - self.source
    }
}

fn create_mapping(s: &str) -> Mapping {
    let split: Vec<u64> = s.split(" ")
        .map(|s| s.parse::<u64>().unwrap())
        .collect();
    Mapping {
        dest: split[0],
        source: split[1],
        range: split[2],
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn aoc_4_1_test() {
        assert_eq!(35, aoc_4_1(EXAMPLE_01));
        assert_eq!(403695602, aoc_4_1(ACTUAL));
    }

    #[test]
    fn aoc_4_2_test() {
        assert_eq!(46, aoc_4_2(EXAMPLE_01));
        // assert_eq!(403695602, aoc_4_1(ACTUAL));
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
        let mapping = Mapping {
            dest: 50,
            source: 98,
            range: 2
        };
        assert!(mapping.in_range(98));
        assert!(mapping.in_range(99));
        assert!(!mapping.in_range(97));
        assert!(!mapping.in_range(100));
    }

    #[test]
    fn mapping_map_test() {
        let mapping = Mapping {
            dest: 50,
            source: 98,
            range: 2
        };
        assert_eq!(51, mapping.map(99));
        let mapping = Mapping {
            dest: 52,
            source: 50,
            range: 48
        };
        assert_eq!(81, mapping.map(79));
    }

    #[test]
    fn create_mapping_test() {
        assert_eq!(Mapping {
            dest: 50,
            source: 98,
            range: 2
        }, create_mapping("50 98 2"));
    }
}
