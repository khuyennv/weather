use constants::app::AppConst;
use serde::{Deserialize};
use std::{error::Error};
#[macro_use] extern crate prettytable;
use prettytable::{color, Attr, Cell, Row, Table};

mod constants;


#[derive(Debug, Deserialize)]
struct WeatherMain {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: i32,
    humidity: i32,
}

#[derive(Debug, Deserialize)]
struct Weather {
    main: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct WeatherResponse {
    main: WeatherMain,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
struct RequestError {
    cod: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let city_name = std::env::args().nth(1).expect("You need in put city");

    let url = AppConst::OPEN_WEATHER_API.replace("{city}", &city_name).replace("{api_key}", AppConst::API_KEY);

    let response = reqwest::get(&url).await;
 
    if let Ok(res) = response {
        if res.status().is_success() {
            let body = res.text().await.unwrap();
            let weather_response: WeatherResponse = serde_json::from_str(&body)?;

             // Create the table
            let mut table = Table::new();
            // Add a row per time
                table.add_row(Row::new(vec![
                    Cell::new(&format!("Weather in {} Today!", city_name)).style_spec("c").with_style(Attr::Bold)
                    .with_style(Attr::ForegroundColor(color::GREEN))
                    .with_hspan(2)
                ]));

            table.add_row(row!["Temperature", weather_response.main.temp]);
            table.add_row(row!["Feels like", weather_response.main.feels_like]);

            table.add_row(row!["Temperature Min", weather_response.main.temp_min]);
            table.add_row(row!["Temperature Max", weather_response.main.temp_max]);

            table.add_row(row!["Pressure", weather_response.main.pressure]);
            table.add_row(row!["humidity", weather_response.main.humidity]);
            table.add_row(row!["Description", weather_response.weather[0].description]);
         

            // Print the table to stdout
            table.printstd();
        } else {
            let body = res.text().await.unwrap();
            let response_body: RequestError = serde_json::from_str(&body)?;
            println!("Request result: {}", response_body.message);
        }
    } else {
        println!("Check your internet line and try again!");
    }

    Ok(())
}
