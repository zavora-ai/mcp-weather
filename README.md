# mcp-weather

[![Crates.io](https://img.shields.io/crates/v/mcp-weather.svg)](https://crates.io/crates/mcp-weather)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Global weather intelligence MCP server — current conditions, forecasts, historical data, air quality, and marine conditions. **7 tools** powered by Open-Meteo (free, no API key required).

## Quick Start

```bash
cargo install mcp-weather
mcp-weather  # Zero configuration needed
```

## Tools (7)

| Tool | Description |
|------|-------------|
| `get_current_weather` | Temperature, wind, humidity, precipitation now |
| `get_forecast` | Daily forecast up to 16 days |
| `get_hourly_forecast` | Hourly forecast for next 48 hours |
| `get_historical_weather` | Historical data for any date range |
| `get_air_quality` | AQI, PM2.5, PM10, ozone, NO2, CO |
| `get_marine_conditions` | Wave height, period, direction, ocean currents |
| `geocode_location` | City name → coordinates |

## Configuration

```json
{ "mcpServers": { "weather": { "command": "mcp-weather" } } }
```

No API keys. No rate limits. Works globally.

## License

Apache-2.0
