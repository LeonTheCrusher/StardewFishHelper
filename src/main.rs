use serde::Deserialize;
use std::fs;

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
enum Time {
    Single(u32, u32), // Represents a single time interval with start and end times
    Multiple(Vec<(u32, u32)>), // Represents multiple time intervals in a day
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Fish {
    name: String,
    desc: String,
    base_price: u32,
    location: Vec<String>,
    season: Vec<String>,
    weather: String,
    size: Size,
    behavior: String,
    difficulty: u32,
    base_xp: u32,
    uses: Vec<String>,
    #[serde(deserialize_with = "deserialize_time")]
    time: Time,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Clone)]
struct Size {
    min: i32,
    max: i32,
}

// Chat Gippity Start
fn deserialize_time<'de, D>(deserializer: D) -> Result<Time, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Debug, Deserialize)]
    #[serde(untagged)]
    enum TimeEnum {
        Single(Vec<u32>), // Represents a single time interval as an array of two elements
        Multiple(Vec<Vec<u32>>), // Represents multiple time intervals as an array of arrays
    }

    let time_enum = TimeEnum::deserialize(deserializer)?;

    match time_enum {
        TimeEnum::Single(time_vec) => {
            if time_vec.len() != 2 {
                return Err(serde::de::Error::custom(
                    "Invalid single time interval format",
                ));
            }
            Ok(Time::Single(time_vec[0], time_vec[1]))
        }
        TimeEnum::Multiple(times_vec) => {
            let mut times = Vec::new();
            for time_vec in times_vec {
                if time_vec.len() != 2 {
                    return Err(serde::de::Error::custom(
                        "Invalid multiple time interval format",
                    ));
                }
                times.push((time_vec[0], time_vec[1]));
            }
            Ok(Time::Multiple(times))
        }
    }
}
// Chat Gippity End

fn main() {
    let file_content = fs::read_to_string("fish.json").expect("error reading file");

    let fish_vector: Vec<Fish> = serde_json::from_str(&file_content).expect("error parsing JSON");

    // Change these values
    //Mountain Lake
    // Secret Woods Pond
    // The Sewers
    // Ocean
    // River (Town)
    // Forest Pond
    // River (Town+Forest)
    // Forest River
    // Forest Farm
    // Mines (20)
    // Mines (60)
    // Mines (100)
    // Ginger Island Ocean
    // Ginger Island Rivers
    // Pirate Cove
    // Mutant Bug Lair
    // Witch's Swamp
    // Desert
    // Volcano

    let location = "Ocean";
    let season = "Summer"; // Winter, Spring, Summer, Fall, or Any
    let weather = "Any"; // Sun, Rain, Any

    let output = get_by_weather(
        &get_by_season(&get_by_loc(&fish_vector, &location), &season),
        &weather,
    );

    for fish in output.iter() {
        println!("{:?}", fish.name);
    }
}

fn get_by_loc(fishes: &[Fish], loc: &str) -> Vec<Fish> {
    let mut found_fish: Vec<Fish> = Vec::new();
    for fish in fishes {
        for location in &fish.location {
            if loc == "Any" || loc == location {
                found_fish.push(fish.clone());
                break;
            }
        }
    }
    found_fish
}

fn get_by_season(fishes: &[Fish], sea: &str) -> Vec<Fish> {
    let mut found_fish: Vec<Fish> = Vec::new();
    for fish in fishes {
        for season in &fish.season {
            if sea == "Any" || sea == season {
                found_fish.push(fish.clone());
                break;
            }
        }
    }
    found_fish
}

fn get_by_weather(fishes: &[Fish], wea: &str) -> Vec<Fish> {
    let mut found_fish: Vec<Fish> = Vec::new();
    for fish in fishes {
        if wea == "Any" || wea == &fish.weather {
            found_fish.push(fish.clone());
        }
    }
    found_fish
}
