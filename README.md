# mcp-weather

[![Crates.io](https://img.shields.io/crates/v/mcp-weather.svg)](https://crates.io/crates/mcp-weather)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Global weather intelligence MCP server — current conditions, forecasts up to 16 days, historical data, air quality, and marine conditions for any location on Earth. **7 tools** powered by Open-Meteo (free, no API key, no rate limits).

## Installation

```bash
cargo install mcp-weather
```

## Quick Start

```bash
mcp-weather  # Zero configuration, works immediately
```

## MCP Client Configuration

### Claude Desktop / Cursor

```json
{
  "mcpServers": {
    "weather": {
      "command": "mcp-weather"
    }
  }
}
```

No API keys. No environment variables. No rate limits.

## Tools (7)

| Tool | Description |
|------|-------------|
| `get_current_weather` | Temperature, wind, humidity, precipitation right now |
| `get_forecast` | Daily forecast up to 16 days (max/min temp, rain, wind) |
| `get_hourly_forecast` | Hour-by-hour for next 48 hours |
| `get_historical_weather` | Past weather for any date range |
| `get_air_quality` | AQI, PM2.5, PM10, ozone, NO2, CO levels |
| `get_marine_conditions` | Wave height, period, direction, ocean currents |
| `geocode_location` | City name → latitude/longitude coordinates |

## Usage Examples

### "What's the weather in Nairobi?"
```
→ geocode_location(name="Nairobi")
  → lat: -1.29, lon: 36.82

→ get_current_weather(latitude=-1.29, longitude=36.82)
  → 16.8°C, wind 0.9 km/h, humidity 81%
```

### "Will it rain in London this week?"
```
→ get_forecast(latitude=51.51, longitude=-0.13, days=7)
  → Daily: max temps, min temps, precipitation_sum per day
```

### "Air quality in Delhi"
```
→ get_air_quality(latitude=28.61, longitude=77.21)
  → AQI: 156, PM2.5: 68.2, PM10: 112.4
```

### "Wave conditions for surfing in Cape Town"
```
→ get_marine_conditions(latitude=-33.92, longitude=18.42)
  → Wave height: 2.1m, period: 12s, direction: SW
```

### "What was the weather on my birthday last year?"
```
→ get_historical_weather(latitude=40.71, longitude=-74.01, start_date="2025-03-15", end_date="2025-03-15")
  → Max: 12°C, Min: 4°C, Precipitation: 0mm
```

## Coordinates for Common Cities

| City | Latitude | Longitude |
|------|----------|-----------|
| Nairobi | -1.29 | 36.82 |
| London | 51.51 | -0.13 |
| New York | 40.71 | -74.01 |
| Dubai | 25.20 | 55.27 |
| Tokyo | 35.68 | 139.69 |
| Sydney | -33.87 | 151.21 |
| Lagos | 6.45 | 3.40 |
| Mumbai | 19.08 | 72.88 |

Use `geocode_location` to find coordinates for any city.

## Data Source

All data from [Open-Meteo](https://open-meteo.com/) — a free, open-source weather API:
- No API key required
- No rate limits for reasonable use
- Global coverage (any lat/lon on Earth)
- Updates every 15 minutes (current), hourly (forecast)
- Historical data back to 1940

## License

Apache-2.0
