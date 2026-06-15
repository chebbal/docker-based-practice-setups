// use std::env;
use clap::Parser;

const FAREN_2_CELSIUS_FACTOR: f64 = 5.0 / 9.0;
const CELSIUS_2_FAREN_FACTOR: f64 = 9.0 / 5.0;

fn faren_2_celsius(faren: f64) -> f64 {
    (faren - 32.0) * FAREN_2_CELSIUS_FACTOR
}

fn celsius_2_faren(celsius: f64) -> f64 {
    celsius * CELSIUS_2_FAREN_FACTOR + 32.0 
}

#[derive(Parser)]
#[command(name = "temperature_converter")]
#[command(about = "Convert temperatures between Fahrenheit and Celsius")]
struct Args {
    /// Temperature to convert
    temperature: f64,

    /// Input is Fahrenheit; convert to Celsius (default: Celsius → Fahrenheit)
    #[arg(long)]
    to_celsius: bool,
}

fn main() {

    let args = Args::parse();
    let temp: f64 = args.temperature;

    if args.to_celsius {
        let result = faren_2_celsius(temp);
        println!("{}°F = {}°C", temp, result);
    } else {
        let result = celsius_2_faren(temp);
        println!("{}°C = {}°F", temp, result);
    }
}
