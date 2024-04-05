# Rust CLI Weather Tool

This is a simple command-line tool written in Rust to fetch and display weather information for a given city using the
WeatherAPI.com API.

## Prerequisites

- Rust Programming Language: Install Rust from [rustup.rs](https://rustup.rs/)
- WeatherAPI.com API Key: Obtain an API key from [WeatherAPI.com](https://www.weatherapi.com/)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/quannadev/weather_checker.git
```

1. Change into the project directory:

```bash
cd weather_checker
```

3. Create a `.env` file in the root of the project and add your WeatherAPI.com API key:

```bash
WEATHER_API_KEY=your_api_key_here
```

4. Build the project:

```bash
cargo build --release
```

5. Run the project:

```bash
./target/release/weather_check
```

```bash
type your city_name in the terminal
```