use iset::IntervalMap;
use itertools::Itertools;


#[derive(Debug)]
struct MapPart {
    source_start: u64,
    target_start: u64,
    length: u64,
}

// #[derive(Debug)]
// struct SourceToTargetMap {
//     parts: IntervalMap<u64, MapPart>,
// }
trait SourceToTargetMap {
    fn get_or_default(&self, source: u64) -> u64;
}
impl SourceToTargetMap for IntervalMap<u64, MapPart> {
    fn get_or_default(&self, source: u64) -> u64 {
        // match self.parts.values_overlap(source).next() {
        let a = match self.values_overlap(source).next() {
            None => {
                // println!("identity: {}", source);
                source
            },
            Some(part) => {
                let i = source - part.source_start;
                // println!("source: {}\nmapping: {:?}\nresult: {}", source,part, part.target_start + i);
                part.target_start + i
            },
        };

        // println!("got {}", a);
        return a;
    }

}

fn is_new_mapping(command: &str) -> bool {
    match command {
        "seed-to-soil map" |
        "soil-to-fertilizer map" |
        "fertilizer-to-water map" |
        "water-to-light map" |
        "light-to-temperature map" |
        "temperature-to-humidity map" |
        "humidity-to-location map" => true,
        _ => false,
    }
}

fn parse_map_part(part: &str) -> MapPart {
    let mut iter = part.split(' ');
    let (a, b, c) = iter.map(|x| x.parse::<u64>().unwrap()).collect_tuple().unwrap();

    return MapPart { source_start: b, target_start: a, length: c };
}

fn parse_input_lines(lines: Vec<&str>) -> (Vec<u64>, Vec<Vec<MapPart>>) {
    let mut maps: Vec<Vec<MapPart>> = vec![];

    let mut lines_iter = lines.iter();
    let seeds_line = lines_iter.next().unwrap();
    let seeds = seeds_line["seeds: ".len()..]
        .split(' ')
        .map(|x| x.parse::<u64>().unwrap().clone())
        .collect::<Vec<u64>>().clone();

    let mut current_mapping: Vec<MapPart> = vec![];
    lines_iter.next();

    for line in lines_iter {
        let mut iter = line.splitn(2, ":");
        let command_or_line = iter.next().unwrap();

        if command_or_line.is_empty() {
            maps.push(current_mapping);
            current_mapping = vec![];
        }
        else if !is_new_mapping(command_or_line) {
            let part = parse_map_part(command_or_line);
            current_mapping.push(part);
        }
    }

    return (seeds, maps);
}

fn build_tree(parts: Vec<MapPart>) -> IntervalMap<u64, MapPart> {
    parts.into_iter()
        .map(|x| (x.source_start .. x.source_start + x.length, x))
        .collect()
}

fn location_for_seed(seed: u64, trees: &Vec<IntervalMap<u64, MapPart>>) -> u64 {
    // println!("\n\n");
    trees.into_iter()
        .fold(seed, |prev, tree| tree.get_or_default(prev))
}


fn main() {

    let data = include_str!("day5.txt").split('\n');
    let (seeds, raw_maps) = parse_input_lines(data.collect());
    println!("seeds {:?}", seeds);

    let trees = raw_maps.into_iter().map(|map| build_tree(map)).collect();


    let soils = seeds.into_iter().map(|seed| location_for_seed(seed, &trees));
    println!("soils: {:?}", soils.clone().collect::<Vec<u64>>());

    println!("ans: {:?}", soils.min());
}
