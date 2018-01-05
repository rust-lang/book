extern crate aggregator;

fn main() {
let weather_forecast = WeatherForecast {
    high_temp: 30.,
    low_temp: -10.,
    chance_of_precipitation: 25.5,
    
};

println!("the weather in the forecast new is  {}", weather_forecast.summary());
}
