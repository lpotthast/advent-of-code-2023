use itertools::Itertools;
use smallvec::SmallVec;
use std::{ops::Range, str::Lines};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn part1(input: &str) -> Option<i64> {
    let (seeds, config) = read_input(input, SeedInterpretation::Single);
    seeds
        .expect_single()
        .into_iter()
        .map(|seed| {
            let soil = config.seed_to_soil.project(seed);
            let fertilizer = config.soil_to_fertilizer.project(soil);
            let water = config.fertilizer_to_water.project(fertilizer);
            let light: i64 = config.water_to_light.project(water);
            let temperature = config.light_to_temperature.project(light);
            let humidity = config.temperature_to_humidity.project(temperature);
            config.humidity_to_location.project(humidity)
        })
        .min()
}

pub fn part2(input: &str) -> Option<i64> {
    let (seeds, config) = read_input(input, SeedInterpretation::Range);
    seeds
        .expect_ranges()
        .into_iter()
        .fold(None, |prev_lowest_location, seed_range| {
            let mut to_soil = SmallVec::<[Projection; 32]>::new();
            config.seed_to_soil.project_range(seed_range, &mut to_soil);
            let to_fertilizer = config.soil_to_fertilizer.project_projections(to_soil);
            let to_water = config.fertilizer_to_water.project_projections(to_fertilizer);
            let to_light = config.water_to_light.project_projections(to_water);
            let to_temperature = config.light_to_temperature.project_projections(to_light);
            let to_humidity = config.temperature_to_humidity.project_projections(to_temperature);
            let to_location = config.humidity_to_location.project_projections(to_humidity);

            let min_location = to_location
                .into_iter()
                .map(|p| p.target_range().start)
                .min()
                .expect("at least one location projection");

            Some(i64::min(min_location, prev_lowest_location.unwrap_or(i64::MAX)))
        })
}

#[derive(Debug, Clone, Copy)]
struct MyRange {
    start: i64,
    end: i64, // exclusive
}

impl MyRange {
    const fn range(&self) -> Range<i64> {
        self.start..self.end
    }

    fn overlaps(&self, other: &Self) -> Option<(i64, i64)> {
        let max_start = i64::max(self.start, other.start);
        let min_end = i64::min(self.end, other.end);
        match max_start < min_end {
            true => Some((max_start, min_end)),
            false => None,
        }
    }
}

#[derive(Debug)]
struct Projection {
    source_range: MyRange,
    offset: i64, // No range here, as the target range has the same size as the source range!
}

impl Projection {
    const fn target_range(&self) -> MyRange {
        MyRange {
            start: self.source_range.start + self.offset,
            end: self.source_range.end + self.offset,
        }
    }
}

#[derive(Debug)]
struct Projections {
    ty: ProjectionType,
    projections: SmallVec<[Projection; 32]>,
}

impl Projections {
    fn project(&self, value: i64) -> i64 {
        for mapping in &self.projections {
            if mapping.source_range.range().contains(&value) {
                return value + mapping.offset;
            }
        }
        value
    }

    fn project_range(&self, source_range: MyRange, out: &mut SmallVec<[Projection; 32]>) {
        for mapping in &self.projections {
            if let Some((max_start, min_end)) = source_range.overlaps(&mapping.source_range) {
                out.push(Projection {
                    source_range: MyRange {
                        start: max_start,
                        end: min_end,
                    },
                    offset: mapping.offset,
                });
                out.sort_by_key(|p| p.source_range.start);
            }
        }

        if out.is_empty() {
            out.push(Projection {
                source_range,
                offset: 0,
            });
        }

        // Add identity projections. Every range not covered with offset 0.
        let mut next = source_range.start;
        let mut filler = SmallVec::<[Projection; 32]>::new();
        for p in out.as_ref() {
            if p.source_range.start > next {
                let identity = Projection {
                    source_range: MyRange {
                        start: next,
                        end: p.source_range.start,
                    },
                    offset: 0,
                };
                filler.push(identity);
            }
            next = p.source_range.end;
        }

        out.append(&mut filler);
        out.sort_by_key(|p| p.source_range.start);
    }

    fn project_projections(&self, source_projections: SmallVec<[Projection; 32]>) -> SmallVec<[Projection; 32]> {
        let mut projections = SmallVec::<[Projection; 32]>::new();
        let mut out = SmallVec::<[Projection; 32]>::new();
        for source_projection in source_projections {
            self.project_range(source_projection.target_range(), &mut out);
            projections.append(&mut out);
        }
        //projections.sort_by_key(|p| p.source_range.start);
        projections
    }
}

#[derive(Debug)]
enum Seeds {
    Single(Vec<i64>),
    Ranges(Vec<MyRange>),
}

impl Seeds {
    fn expect_single(self) -> Vec<i64> {
        match self {
            Self::Single(single) => single,
            Self::Ranges(_) => panic!("unexpected"),
        }
    }
    fn expect_ranges(self) -> Vec<MyRange> {
        match self {
            Self::Single(_) => panic!("unexpected"),
            Self::Ranges(ranges) => ranges,
        }
    }
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum InputPart {
    Seeds(Vec<i64>),
    SeedRanges(Vec<MyRange>),
    Projections(Projections),
}

#[derive(Debug, Clone, Copy)]
enum SeedInterpretation {
    Single,
    Range,
}

struct InputParser<'a> {
    seed_interpretation: SeedInterpretation,
    lines: Lines<'a>,
    line: Option<&'a str>,
    in_block: Option<Projections>,
}

impl<'a> InputParser<'a> {
    fn new(input: &'a str, seed_interpretation: SeedInterpretation) -> Self {
        let mut lines = input.lines();
        Self {
            seed_interpretation,
            line: lines.next(),
            lines,
            in_block: None,
        }
    }
}

impl<'a> InputParser<'a> {
    fn read_seeds<I: Iterator<Item = i64>>(it: I, seed_interpretation: SeedInterpretation) -> InputPart {
        match seed_interpretation {
            SeedInterpretation::Single => {
                let seeds = it.collect::<Vec<_>>();
                InputPart::Seeds(seeds)
            }
            SeedInterpretation::Range => {
                let mut tuples = it.tuples();
                let ranges = tuples
                    .by_ref()
                    .map(|(start, len)| MyRange {
                        start,
                        end: start + len,
                    })
                    .collect_vec();
                for leftover_seed in tuples.into_buffer() {
                    tracing::warn!(leftover_seed, "found leftover when reading seeds as range pairs");
                }
                InputPart::SeedRanges(ranges)
            }
        }
    }

    fn read_projection(line: &str) -> Projection {
        let mut it = line
            .split_ascii_whitespace()
            .map(|it| it.parse::<i64>().expect("number"));
        let target_start = it.next().expect("target_start");
        let source_start = it.next().expect("source_start");
        let len = it.next().expect("len");
        assert_eq!(it.next(), None);
        Projection {
            source_range: MyRange {
                start: source_start,
                end: source_start + len,
            },
            offset: target_start - source_start,
        }
    }

    fn start_block(&mut self, ty: ProjectionType) {
        self.in_block = Some(Projections {
            ty,
            projections: SmallVec::<[Projection; 32]>::new(),
        });
    }

    fn finish_block(&mut self) -> Option<Projections> {
        self.in_block.take()
    }

    fn next_line(&mut self) {
        self.line = self.lines.next();
    }
}

impl<'a> Iterator for InputParser<'a> {
    type Item = InputPart;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            match self.line {
                Some(line) => {
                    if let Some(line) = line.strip_prefix("seeds:") {
                        let it = line
                            .trim_start()
                            .split_ascii_whitespace()
                            .map(|it| it.parse::<i64>().expect("number"));
                        self.next_line();
                        return Some(InputParser::read_seeds(it, self.seed_interpretation));
                    }

                    if line.is_empty() {
                        self.next_line();
                        if let Some(block) = self.finish_block() {
                            return Some(InputPart::Projections(block));
                        }
                        continue 'outer;
                    }

                    if let Some(projections) = &mut self.in_block {
                        projections.projections.push(InputParser::read_projection(line));
                        self.next_line();
                        continue 'outer;
                    }

                    for ty in ProjectionType::iter() {
                        if line.starts_with(ty.block_name()) {
                            self.start_block(ty);
                            self.next_line();
                            continue 'outer;
                        }
                    }

                    tracing::warn!(line, "We do not understand this input...");
                    return None;
                }
                None => {
                    if let Some(block) = self.finish_block() {
                        return Some(InputPart::Projections(block));
                    }
                    return None;
                }
            }
        }
    }
}

#[derive(Debug)]
struct Config {
    seed_to_soil: Projections,
    soil_to_fertilizer: Projections,
    fertilizer_to_water: Projections,
    water_to_light: Projections,
    light_to_temperature: Projections,
    temperature_to_humidity: Projections,
    humidity_to_location: Projections,
}

fn read_input(input: &str, seed_interpretation: SeedInterpretation) -> (Seeds, Config) {
    let mut seeds: Option<Seeds> = None;
    let mut seed_to_soil: Option<Projections> = None;
    let mut soil_to_fertilizer: Option<Projections> = None;
    let mut fertilizer_to_water: Option<Projections> = None;
    let mut water_to_light: Option<Projections> = None;
    let mut light_to_temperature: Option<Projections> = None;
    let mut temperature_to_humidity: Option<Projections> = None;
    let mut humidity_to_location: Option<Projections> = None;

    for part in InputParser::new(input, seed_interpretation) {
        match part {
            InputPart::Seeds(single) => seeds = Some(Seeds::Single(single)),
            InputPart::SeedRanges(ranges) => seeds = Some(Seeds::Ranges(ranges)),
            InputPart::Projections(mappings) => match mappings.ty {
                ProjectionType::SeedToSoil => seed_to_soil = Some(mappings),
                ProjectionType::SoilToFertilizer => soil_to_fertilizer = Some(mappings),
                ProjectionType::FertilizerToWater => fertilizer_to_water = Some(mappings),
                ProjectionType::WaterToLight => water_to_light = Some(mappings),
                ProjectionType::LightToTemperature => light_to_temperature = Some(mappings),
                ProjectionType::TemperatureToHumidity => temperature_to_humidity = Some(mappings),
                ProjectionType::HumidityToLocation => humidity_to_location = Some(mappings),
            },
        }
    }
    (
        seeds.expect("seeds"),
        Config {
            seed_to_soil: seed_to_soil.expect("block"),
            soil_to_fertilizer: soil_to_fertilizer.expect("block"),
            fertilizer_to_water: fertilizer_to_water.expect("block"),
            water_to_light: water_to_light.expect("block"),
            light_to_temperature: light_to_temperature.expect("block"),
            temperature_to_humidity: temperature_to_humidity.expect("block"),
            humidity_to_location: humidity_to_location.expect("block"),
        },
    )
}

#[derive(Debug, Clone, Copy, EnumIter)]
enum ProjectionType {
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl ProjectionType {
    const fn block_name(self) -> &'static str {
        match self {
            Self::SeedToSoil => "seed-to-soil",
            Self::SoilToFertilizer => "soil-to-fertilizer",
            Self::FertilizerToWater => "fertilizer-to-water",
            Self::WaterToLight => "water-to-light",
            Self::LightToTemperature => "light-to-temperature",
            Self::TemperatureToHumidity => "temperature-to-humidity",
            Self::HumidityToLocation => "humidity-to-location",
        }
    }
}
