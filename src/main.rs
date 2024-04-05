use serde::{Deserialize};
use std::{borrow::Borrow, error::Error};

#[derive(Debug, Deserialize)]
struct Main {
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
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Debug, Deserialize)]
struct RequestError {
    cod: String,
    message: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_key = "6b15677169a1ea1d3cc026022fec71ef";
    let city_name = std::env::args().nth(1).expect("You need in put city");
    // let city_name = "Hanoi";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city_name, api_key
    );

    let response = reqwest::get(&url).await;
 
    if let Ok(res) = response {
        if res.status().is_success() {
            let body = res.text().await.unwrap();
            let weather_response: WeatherResponse = serde_json::from_str(&body)?;
            println!("Temperature: {}°C", weather_response.main.temp);
            println!("Feels like: {}°C", weather_response.main.feels_like);
            println!("Description: {}", weather_response.weather[0].description);

        } else {
            let body = res.text().await.unwrap();
            let response_body: RequestError = serde_json::from_str(&body)?;
            println!("Request result:{}", response_body.message);
        }
    } else {
        println!("Check your internet line and try again!");
    }

    Ok(())
}
