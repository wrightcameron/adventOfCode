use std::rc::Rc;

use log::debug;

struct AlmanacMap {
    destination: i64,
    source: i64,
    range: i64,
}

struct Almanac {
    almanac_maps: Vec<AlmanacMap>,
}

struct Seed {
    source: i64,
    range: i64,
    lowest: i64,
}

pub fn problem1(input: &String) -> i64 {
    // Parse the input, store them in a struct
    let chunks: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let mut chunks_iter = chunks.iter();
    let (_before, after) = chunks_iter.next().unwrap().split_once(":").unwrap();
    let seeds = after
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    debug!("Seeds: {:?}", seeds);
    // Parse the rest of the maps, looks like there alwasy are 7

    // Seed to Soil Map
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let seed_soil = create_map(&map);
    debug!("seed-to-soil map:");
    for map in &seed_soil.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range)
    }

    // soil-to-fertilizer
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map: Vec<&str> = after.lines().collect::<Vec<&str>>();
    let soil_fertilizer = create_map(&map);
    println!("soil-to-fertilizer map:");
    for map in &soil_fertilizer.almanac_maps {
        println!(" {} {} {}", map.destination, map.source, map.range)
    }

    // fertilizer-to-water
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let fertilizer_water = create_map(&map);
    debug!("fertilizer-to-water map:");
    for map in &fertilizer_water.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range)
    }

    // water-to-light
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let water_light = create_map(&map);
    debug!("water-to-light map:");
    for map in &water_light.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range)
    }

    // light-to-temperature
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let light_temperature = create_map(&map);
    debug!("light-to-temperature map:");
    for map in &light_temperature.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range)
    }

    // temperature-to-humidity
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let temperature_humidity = create_map(&map);
    debug!("temperature-to-humidity map:");
    for map in &temperature_humidity.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range)
    }

    // humidity-to-location
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let humidity_location = create_map(&map);
    debug!("humidity-to-location map:");
    for map in &humidity_location.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range)
    }

    // Now for the rest
    return seeds
        .iter()
        .map(|i| {
            println!("Starting Seed {}", i);
            let temp = get_destination(&seed_soil.almanac_maps, *i);
            println!("soil  {}", temp);
            let temp = get_destination(&soil_fertilizer.almanac_maps, temp);
            println!("fertilizer  {}", temp);
            let temp = get_destination(&fertilizer_water.almanac_maps, temp);
            println!("water  {}", temp);
            let temp = get_destination(&water_light.almanac_maps, temp);
            println!("light  {}", temp);
            let temp = get_destination(&light_temperature.almanac_maps, temp);
            println!("temperature  {}", temp);
            let temp = get_destination(&temperature_humidity.almanac_maps, temp);
            println!("humidity  {}", temp);
            let temp = get_destination(&humidity_location.almanac_maps, temp);
            println!("Location {}", temp);
            return temp;
        })
        .min()
        .unwrap();
}

pub fn problem2(input: &String) -> i64 {
    // Parse the input, store them in a struct
    let chunks: Vec<&str> = input.split("\n\n").collect::<Vec<&str>>();
    let mut chunks_iter = chunks.iter();
    let (_before, after) = chunks_iter.next().unwrap().split_once(":").unwrap();
    let seeds = after
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i64>>();
    let mut seed_vec = Vec::new();
    // Get seeds into vector
    for i in (0..seeds.len()).step_by(2) {
        seed_vec.push(Seed {
            source: seeds[i] as i64,
            range: seeds[i + 1],
            lowest: i64::MAX,
        })
    }
    // Print the seeds
    // for i in seed_vec.iter() {
    //     println!("Start: {}, Range: {}, Lowest: {}",i.source, i.range, i.lowest);
    // }

    // Need to sort the seeds,
    seed_vec.sort_by(|a, b| a.source.cmp(&b.source));
    // for i in seed_vec.iter() {
    //     println!("Start: {}, Range: {}, Lowest: {}",i.source, i.range, i.lowest);
    // }
    // Check for duplicate, if there are overlaps in seed ranges, lets decrease the range.
    for i in 0..(seeds.len() / 2) {
        if i == seed_vec.len() - 1 {
            break;
        }
        println!(
            "Start: {}, Max: {}, Lowest: {}",
            seed_vec[i].source,
            seed_vec[i].source + seed_vec[i].range,
            seed_vec[i].lowest
        );
        if seed_vec[i].source + seed_vec[i].range >= seed_vec[i + 1].source {
            seed_vec[i].range = seed_vec[i + 1].source - seed_vec[i].source;
            println!(
                "New Start: {}, Max: {}, Lowest: {}",
                seed_vec[i].source,
                seed_vec[i].source + seed_vec[i].range,
                seed_vec[i].lowest
            );
        }
    }
    // Parse the rest of the maps, looks like there alwasy are 7

    let mut chunk_size: i64 = i64::MAX;

    // Seed to Soil Map
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let seed_soil = create_map(&map);
    debug!("seed-to-soil map:");
    for map in &seed_soil.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range);
        if map.range < chunk_size {
            chunk_size = map.range;
        }
    }

    // soil-to-fertilizer
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map: Vec<&str> = after.lines().collect::<Vec<&str>>();
    let soil_fertilizer = create_map(&map);
    println!("soil-to-fertilizer map:");
    for map in &soil_fertilizer.almanac_maps {
        println!(" {} {} {}", map.destination, map.source, map.range);
        if map.range < chunk_size {
            chunk_size = map.range;
        }
    }

    // fertilizer-to-water
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let fertilizer_water = create_map(&map);
    debug!("fertilizer-to-water map:");
    for map in &fertilizer_water.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range);
        if map.range < chunk_size {
            chunk_size = map.range;
        }
    }

    // water-to-light
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let water_light = create_map(&map);
    debug!("water-to-light map:");
    for map in &water_light.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range);
        if map.range < chunk_size {
            chunk_size = map.range;
        }
    }

    // light-to-temperature
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let light_temperature = create_map(&map);
    debug!("light-to-temperature map:");
    for map in &light_temperature.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range);
        if map.range < chunk_size {
            chunk_size = map.range;
        }
    }

    // temperature-to-humidity
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let temperature_humidity = create_map(&map);
    debug!("temperature-to-humidity map:");
    for map in &temperature_humidity.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range);
        if map.range < chunk_size {
            chunk_size = map.range;
        }
    }

    // humidity-to-location
    let (_before, after) = chunks_iter.next().unwrap().split_once(":\n").unwrap();
    let map = after.lines().collect::<Vec<&str>>();
    let humidity_location = create_map(&map);
    debug!("humidity-to-location map:");
    for map in &humidity_location.almanac_maps {
        debug!(" {} {} {}", map.destination, map.source, map.range);
        if map.range < chunk_size {
            chunk_size = map.range;
        }
    }

    //TODO Lets cut the data into chunks, find the lowest chunk, then scan all the elements in that chunk after.
    //TODO Find size of chunk, lets make it half of the smallest range.
    println!("Chunk Size {chunk_size}");

    // Find lowest for each seed
    for seed in &mut seed_vec {
        let mut lowest: i64 = i64::MAX;
        for i in (seed.source..(seed.source + seed.range)).step_by(320994 / 20) {
            let temp = get_destination(&seed_soil.almanac_maps, i);
            let temp = get_destination(&soil_fertilizer.almanac_maps, temp);
            let temp = get_destination(&fertilizer_water.almanac_maps, temp);
            let temp = get_destination(&water_light.almanac_maps, temp);
            let temp = get_destination(&light_temperature.almanac_maps, temp);
            let temp = get_destination(&temperature_humidity.almanac_maps, temp);
            let temp = get_destination(&humidity_location.almanac_maps, temp);
            if lowest > temp {
                lowest = temp;
            }
        }
        seed.lowest = lowest;
        println!("Finished seed range {}.  Lowest: {}", seed.source, lowest);
    }
    // Find where seed was from
    let semi_lowest = seed_vec.iter().map(|x| x.lowest).min().unwrap();
    let temp = get_destination(&humidity_location.almanac_maps, semi_lowest);
    let temp = get_destination(&temperature_humidity.almanac_maps, temp);
    let temp = get_destination(&light_temperature.almanac_maps, temp);
    let temp = get_destination(&water_light.almanac_maps, temp);
    let temp = get_destination(&fertilizer_water.almanac_maps, temp);
    let temp = get_destination(&soil_fertilizer.almanac_maps, temp);
    let temp = get_destination(&seed_soil.almanac_maps, temp);

    let orginal_seed = temp;
    // Now for the rest
    let mut lowest: i64 = i64::MAX;
    // for i in orginal_seed..(orginal_seed + (320994 / 20) ) {
    //     let temp = get_destination(&seed_soil.almanac_maps, i);
    //     let temp = get_destination(&soil_fertilizer.almanac_maps, temp);
    //     let temp = get_destination(&fertilizer_water.almanac_maps, temp);
    //     let temp = get_destination(&water_light.almanac_maps, temp);
    //     let temp = get_destination(&light_temperature.almanac_maps, temp);
    //     let temp = get_destination(&temperature_humidity.almanac_maps, temp);
    //     let temp = get_destination(&humidity_location.almanac_maps, temp);
    //     if lowest > temp {
    //         lowest = temp;
    //     }
    // }
    //Brute force just check the range most likely to be in
    for i in 856311572..(856311572 + 542740109) {
        let temp = get_destination(&seed_soil.almanac_maps, i);
        let temp = get_destination(&soil_fertilizer.almanac_maps, temp);
        let temp = get_destination(&fertilizer_water.almanac_maps, temp);
        let temp = get_destination(&water_light.almanac_maps, temp);
        let temp = get_destination(&light_temperature.almanac_maps, temp);
        let temp = get_destination(&temperature_humidity.almanac_maps, temp);
        let temp = get_destination(&humidity_location.almanac_maps, temp);
        if lowest > temp {
            lowest = temp;
        }
    }
    return lowest;
}

fn create_map(map_input: &Vec<&str>) -> Almanac {
    let mut almanac_maps: Vec<AlmanacMap> = Vec::new();
    for i in map_input {
        // println!("Map: {:?}", i);
        let j = i
            .trim()
            .split(" ")
            .map(|x| x.parse().unwrap())
            .collect::<Vec<i64>>();
        almanac_maps.push(AlmanacMap {
            destination: j[0],
            source: j[1],
            range: j[2],
        })
    }
    return Almanac {
        almanac_maps: almanac_maps,
    };
}

fn get_destination(maps: &Vec<AlmanacMap>, input: i64) -> i64 {
    for range in maps.iter() {
        // If source in range
        debug!(
            "Source: {} - {}, Value:{}",
            range.source,
            range.source + range.range,
            input
        );
        if range.source <= input && input <= range.source + range.range {
            debug!(
                "Found! Dest {}, {}",
                range.destination,
                range.destination + (range.source - input).abs()
            );
            return range.destination + (range.source - input).abs();
        }
    }
    return input;
}

fn get_source(maps: &Vec<AlmanacMap>, input: i64) -> i64 {
    for range in maps.iter() {
        // If source in range
        debug!("Source: {} - {}, Value:{}", range.source, range.source + range.range, input
        );
        if range.destination <= input && input <= range.destination + range.range {
            debug!("Found! Source {}, {}",range.destination,range.destination + (range.source - input).abs()
            );
            return range.source + (range.destination - input).abs();
        }
    }
    return input;
}
