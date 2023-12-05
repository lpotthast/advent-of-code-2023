use itertools::Itertools;
use std::{ops::Range, str::Lines};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub fn part1(input: &str) -> Option<i64> {
    let (seeds, config) = read_input(input, SeedInterpretation::Single);
    let seeds = seeds.expect_single();

    seeds
        .into_iter()
        .map(|seed| {
            let soil = config.seed_to_soil.project(seed);
            let fertilizer = config.soil_to_fertilizer.project(soil);
            let water = config.fertilizer_to_water.project(fertilizer);
            let light: i64 = config.water_to_light.project(water);
            let temperature = config.light_to_temperature.project(light);
            let humidity = config.temperature_to_humidity.project(temperature);
            let location = config.humidity_to_location.project(humidity);
            // tracing::info!(seed, soil, fertilizer, water, light, temperature, humidity, location);
            location
        })
        .min()
}

pub fn part2(input: &str) -> i64 {
    let (seeds, config) = read_input(input, SeedInterpretation::Range);

    let mut lowest_location = i64::MAX;

    for seed_range in seeds.expect_ranges() {
        let soil_projections = config.seed_to_soil.project_range(seed_range);
        let fertilizer_projections = config.soil_to_fertilizer.project_projections(soil_projections);
        let water_projections = config.fertilizer_to_water.project_projections(fertilizer_projections);
        let light_projections = config.water_to_light.project_projections(water_projections);
        let temperature_projections = config.light_to_temperature.project_projections(light_projections);
        let humidity_projections = config
            .temperature_to_humidity
            .project_projections(temperature_projections);
        let location_projections = config.humidity_to_location.project_projections(humidity_projections);
        let mut locations = location_projections
            .into_iter()
            .map(|p| p.target_range())
            .collect::<Vec<_>>();
        locations.sort_by_key(|r| r.start);

        if let Some(min) = locations.iter().map(|p| p.start).min() {
            lowest_location = i64::min(lowest_location, min);
        }
    }

    lowest_location
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

    for part in ConfigParser::new(input, seed_interpretation) {
        match part {
            ConfigPart::Seeds(single) => seeds = Some(Seeds::Single(single)),
            ConfigPart::SeedRanges(ranges) => seeds = Some(Seeds::Ranges(ranges)),
            ConfigPart::Mappings(mappings) => match mappings.ty {
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
        seeds.unwrap(),
        Config {
            seed_to_soil: seed_to_soil.unwrap(),
            soil_to_fertilizer: soil_to_fertilizer.unwrap(),
            fertilizer_to_water: fertilizer_to_water.unwrap(),
            water_to_light: water_to_light.unwrap(),
            light_to_temperature: light_to_temperature.unwrap(),
            temperature_to_humidity: temperature_to_humidity.unwrap(),
            humidity_to_location: humidity_to_location.unwrap(),
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

#[derive(Debug, Clone, Copy)]
struct MyRange {
    start: i64,
    end: i64, // exclusive
}

impl MyRange {
    fn range(&self) -> Range<i64> {
        self.start..self.end
    }
}

#[derive(Debug)]
struct Projection {
    source_range: MyRange,
    offset: i64, // No range here, as the target range has the same size as the source range!
}

impl Projection {
    fn target_range(&self) -> MyRange {
        MyRange {
            start: self.source_range.start + self.offset,
            end: self.source_range.end + self.offset,
        }
    }
}

#[derive(Debug)]
struct Projections {
    ty: ProjectionType,
    projections: Vec<Projection>,
}

/// seed-to-soil map:
/// 50 98 2  // source - 48 if >= 98 and < 100
/// 52 50 48 // source + 2  if >= 50 and < 98
impl Projections {
    /// Project a range of size 1.
    fn project(&self, value: i64) -> i64 {
        for mapping in &self.projections {
            if mapping.source_range.range().contains(&value) {
                return value.checked_add(mapping.offset).unwrap();
            }
        }
        value
    }

    /// Project a range.
    fn project_range(&self, source_range: MyRange) -> Vec<Projection> {
        let mut projections = Vec::new();
        for mapping in &self.projections {
            // !!!
            let target_range = mapping.source_range;
            if overlaps(&source_range, &target_range) {
                // tracing::info!(?source_range, ?target_range, "overlaps");
                projections.push(Projection {
                    source_range: MyRange {
                        start: i64::max(mapping.source_range.start, source_range.start),
                        end: i64::min(mapping.source_range.end, source_range.end),
                    },
                    offset: mapping.offset,
                });
                projections.sort_by_key(|p| p.source_range.start);
            }
        }

        // tracing::info!(?projections, "BEFORE");

        if projections.is_empty() {
            projections.push(Projection {
                source_range,
                offset: 0,
            })
        }
        projections.sort_by_key(|p| p.source_range.start);

        // tracing::info!(?projections, "DONE");

        // Add identity projections. Every range not covered with offset 0.
        let mut proj = projections.iter();
        let mut next = source_range.start;
        let mut filler = Vec::new();
        while let Some(p) = proj.next() {
            if p.source_range.start > next {
                let identity = Projection {
                    source_range: MyRange {
                        start: next,
                        end: p.source_range.start,
                    },
                    offset: 0,
                };
                // tracing::info!(?p, ?identity, "push test");
                filler.push(identity);
            }
            next = p.source_range.end;
        }

        projections.append(&mut filler);
        projections.sort_by_key(|p| p.source_range.start);

        projections
    }

    fn project_projections(&self, source_projections: Vec<Projection>) -> Vec<Projection> {
        let mut projections = Vec::new();
        for source_projection in source_projections {
            for projection in self.project_range(source_projection.target_range()) {
                projections.push(projection);
            }
        }
        projections.sort_by_key(|p| p.source_range.start);
        projections
    }
}

fn overlaps(range_a: &MyRange, range_b: &MyRange) -> bool {
    let max_start = i64::max(range_a.start, range_b.start);
    let min_end = i64::min(range_a.end, range_b.end);
    max_start < min_end // TODO <= ??
}

#[derive(Debug)]
enum Seeds {
    Single(Vec<i64>),
    Ranges(Vec<MyRange>),
}

impl Seeds {
    fn expect_single(self) -> Vec<i64> {
        match self {
            Seeds::Single(single) => single,
            Seeds::Ranges(_) => panic!("unexpected"),
        }
    }
    fn expect_ranges(self) -> Vec<MyRange> {
        match self {
            Seeds::Single(_) => panic!("unexpected"),
            Seeds::Ranges(ranges) => ranges,
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

#[derive(Debug)]
enum ConfigPart {
    Seeds(Vec<i64>),
    SeedRanges(Vec<MyRange>),
    Mappings(Projections),
}

#[derive(Debug)]
enum SeedInterpretation {
    Single,
    Range,
}

struct ConfigParser<'a> {
    seed_interpretation: SeedInterpretation,
    lines: Lines<'a>,
    line: Option<&'a str>,
    in_block: Option<Projections>,
}

impl<'a> ConfigParser<'a> {
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

impl<'a> ConfigParser<'a> {
    fn next_line(&mut self) {
        self.line = self.lines.next();
    }

    fn finish_block(&mut self) -> Option<Projections> {
        self.in_block.take()
    }
}

impl<'a> Iterator for ConfigParser<'a> {
    type Item = ConfigPart;

    fn next(&mut self) -> Option<Self::Item> {
        'outer: loop {
            match self.line {
                Some(line) => {
                    if line.starts_with("seeds:") {
                        let numbers = line["seeds:".len()..]
                            .trim_start()
                            .split_ascii_whitespace()
                            .map(|it| it.parse::<i64>().unwrap());

                        match self.seed_interpretation {
                            SeedInterpretation::Single => {
                                let seeds = numbers.collect::<Vec<_>>();
                                self.next_line();
                                return Some(ConfigPart::Seeds(seeds));
                            }
                            SeedInterpretation::Range => {
                                let mut tuples = numbers.tuples();
                                let ranges = tuples
                                    .by_ref()
                                    .map(|(start, len)| MyRange {
                                        start,
                                        end: start.checked_add(len).unwrap(),
                                    })
                                    .collect_vec();
                                for leftover_seed in tuples.into_buffer() {
                                    tracing::warn!(leftover_seed, "found leftover when reading seeds as range pairs");
                                }
                                self.next_line();
                                return Some(ConfigPart::SeedRanges(ranges));
                            }
                        }
                    }

                    if line.is_empty() {
                        self.next_line();
                        if let Some(block) = self.finish_block() {
                            return Some(ConfigPart::Mappings(block));
                        }
                        continue 'outer;
                    }

                    if let Some(projections) = &mut self.in_block {
                        // assume line is not empty.
                        let mut nums = line.split_ascii_whitespace().map(|it| it.parse::<i64>().unwrap());
                        let target_start = nums.next().unwrap();
                        let source_start = nums.next().unwrap();
                        let len = nums.next().unwrap();
                        assert_eq!(nums.next(), None);
                        projections.projections.push(Projection {
                            source_range: MyRange {
                                start: source_start,
                                end: source_start.checked_add(len).unwrap(),
                            },
                            offset: target_start.checked_sub(source_start).unwrap(),
                        });
                        self.next_line();
                        continue 'outer;
                    }

                    for ty in ProjectionType::iter() {
                        if line.starts_with(ty.block_name()) {
                            self.in_block = Some(Projections {
                                ty,
                                projections: Vec::new(),
                            });
                            self.next_line();
                            continue 'outer;
                        }
                    }

                    tracing::warn!(line, "We do not understand this input...");
                    return None;
                }
                None => {
                    if let Some(block) = self.finish_block() {
                        return Some(ConfigPart::Mappings(block));
                    }
                    return None;
                }
            }
        }
    }
}
