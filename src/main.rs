use crate::location::LocationResponse;
use inquire::{Select, Text};

mod location;

const KEY: &'static str = "255cf5b7fc5a43a8a8f183109230405";
const BASE_URL: &'static str = "https://api.weatherapi.com/v1/current.json";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = vec!["Get Weather", "Exit"];

    loop {
        let Ok(c) = Select::new("What would you like to do?", options.clone()).prompt() else {
            break
        };
        match c {
            "Get Weather" => get_weather().await?,
            "Exit" => break,
            _ => continue,
        }
    }

    Ok(())
}

async fn get_weather() -> Result<(), reqwest::Error> {
    let req = {
        let loc = Text::new("Enter the location:")
            .prompt()
            .expect("An error occured getting your input");
        format!("{}?key={}&q={}", BASE_URL, KEY, loc)
    };
    let res = reqwest::get(&req).await?;
    let weather = res.json::<LocationResponse>().await?;

    println!("{}, {}:", weather.get_name(), weather.get_region());
    println!(
        "Temp: {}, Condition: {}",
        weather.get_temp(),
        weather.get_condition()
    );

    Ok(())
}
