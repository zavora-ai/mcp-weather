use rmcp::{handler::server::wrapper::Parameters, schemars, tool, tool_router};
use reqwest::Client;
use serde_json::Value;

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct LocationInput {
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ForecastInput {
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Number of forecast days (1-16, default 7)
    pub days: Option<u32>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct HistoricalInput {
    /// Latitude
    pub latitude: f64,
    /// Longitude
    pub longitude: f64,
    /// Start date (YYYY-MM-DD)
    pub start_date: String,
    /// End date (YYYY-MM-DD)
    pub end_date: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GeocodingInput {
    /// City or location name
    pub name: String,
    /// Max results (default 5)
    pub limit: Option<u32>,
}

#[derive(Clone)]
pub struct WeatherServer {
    pub client: Client,
}

#[tool_router(server_handler)]
impl WeatherServer {
    #[tool(description = "Get current weather conditions for a location (temperature, wind, humidity, precipitation)")]
    async fn get_current_weather(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&current=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation,wind_speed_10m,wind_direction_10m,weather_code&timezone=auto",
            input.latitude, input.longitude
        );
        fetch_json(&self.client, &url).await
    }

    #[tool(description = "Get daily weather forecast (temperature, precipitation, wind) for up to 16 days")]
    async fn get_forecast(&self, Parameters(input): Parameters<ForecastInput>) -> String {
        let days = input.days.unwrap_or(7).min(16);
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&daily=temperature_2m_max,temperature_2m_min,precipitation_sum,wind_speed_10m_max,weather_code,sunrise,sunset&timezone=auto&forecast_days={}",
            input.latitude, input.longitude, days
        );
        fetch_json(&self.client, &url).await
    }

    #[tool(description = "Get hourly weather forecast for the next 48 hours")]
    async fn get_hourly_forecast(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let url = format!(
            "https://api.open-meteo.com/v1/forecast?latitude={}&longitude={}&hourly=temperature_2m,precipitation_probability,precipitation,wind_speed_10m,weather_code&timezone=auto&forecast_hours=48",
            input.latitude, input.longitude
        );
        fetch_json(&self.client, &url).await
    }

    #[tool(description = "Get historical weather data for a date range")]
    async fn get_historical_weather(&self, Parameters(input): Parameters<HistoricalInput>) -> String {
        let url = format!(
            "https://archive-api.open-meteo.com/v1/archive?latitude={}&longitude={}&start_date={}&end_date={}&daily=temperature_2m_max,temperature_2m_min,precipitation_sum,wind_speed_10m_max&timezone=auto",
            input.latitude, input.longitude, input.start_date, input.end_date
        );
        fetch_json(&self.client, &url).await
    }

    #[tool(description = "Get current air quality index and pollutant levels")]
    async fn get_air_quality(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let url = format!(
            "https://air-quality-api.open-meteo.com/v1/air-quality?latitude={}&longitude={}&current=us_aqi,pm2_5,pm10,carbon_monoxide,nitrogen_dioxide,ozone",
            input.latitude, input.longitude
        );
        fetch_json(&self.client, &url).await
    }

    #[tool(description = "Get marine/ocean conditions (wave height, period, direction, sea temperature)")]
    async fn get_marine_conditions(&self, Parameters(input): Parameters<LocationInput>) -> String {
        let url = format!(
            "https://marine-api.open-meteo.com/v1/marine?latitude={}&longitude={}&current=wave_height,wave_direction,wave_period,ocean_current_velocity&daily=wave_height_max,wave_period_max&timezone=auto&forecast_days=3",
            input.latitude, input.longitude
        );
        fetch_json(&self.client, &url).await
    }

    #[tool(description = "Search for a location by name and get coordinates (geocoding)")]
    async fn geocode_location(&self, Parameters(input): Parameters<GeocodingInput>) -> String {
        let limit = input.limit.unwrap_or(5);
        let url = format!(
            "https://geocoding-api.open-meteo.com/v1/search?name={}&count={}&language=en&format=json",
            input.name.replace(' ', "+"), limit
        );
        fetch_json(&self.client, &url).await
    }
}

async fn fetch_json(client: &Client, url: &str) -> String {
    match client.get(url).send().await {
        Ok(resp) => match resp.json::<Value>().await {
            Ok(data) => serde_json::to_string_pretty(&data).unwrap_or_default(),
            Err(e) => format!("Error parsing response: {e}"),
        },
        Err(e) => format!("Error: {e}"),
    }
}
