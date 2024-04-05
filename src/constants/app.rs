pub struct AppConst {}

impl AppConst {
    pub const API_KEY: &'static str  = "6b15677169a1ea1d3cc026022fec71ef";
    pub const OPEN_WEATHER_API: &'static str = "https://api.openweathermap.org/data/2.5/weather?q={city}&appid={api_key}&units=metric";
}
