use clap::Parser;
use serde::Deserialize;
use std::env;

/// Weather CLI — простий клієнт для OpenWeatherMap
#[derive(Parser, Debug)]
#[command(author, version, about = "CLI утиліта для перегляду погоди")]
struct Args {
    /// Назва міста
    city: String,
}

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    name: String,
    main: Main,
    weather: Vec<Weather>,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: u64,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

fn main() {
    dotenv::dotenv().ok();
    let args = Args::parse();

    let api_key = env::var("OPENWEATHER_API_KEY").expect("⚠️ Не знайдено API ключ (OPENWEATHER_API_KEY)");

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric&lang=ua",
        args.city, api_key
    );

    match reqwest::blocking::get(&url) {
        Ok(resp) => {
            if resp.status().is_success() {
                let weather: WeatherResponse = resp.json().unwrap();
                println!("🌍 Погода у місті: {}", weather.name);
                println!("🌡️ Температура: {}°C", weather.main.temp);
                println!("💧 Вологість: {}%", weather.main.humidity);
                println!("☁️ Опис: {}", weather.weather[0].description);
            } else {
                eprintln!("❌ Місто не знайдено або помилка API.");
            }
        }
        Err(e) => eprintln!("⚠️ Помилка запиту: {}", e),
    }
}
