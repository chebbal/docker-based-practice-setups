use std::env;

const FAREN_2_CELSIUS_FACTOR: f64 = 5.0 / 9.0;
const CELSIUS_2_FAREN_FACTOR: f64 = 9.0 / 5.0;

fn faren_2_celsius(faren: f64) -> f64 {
    (faren - 32.0) * FAREN_2_CELSIUS_FACTOR
}

fn celsius_2_faren(celsius: f64) -> f64 {
    celsius * CELSIUS_2_FAREN_FACTOR + 32.0 
}

fn main() {
    // println!("Hello, world!");
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: temperature_converter <temperature> [--to-farenheit]");
        std::process::exit(1);
    }

    // assumptions:
    // 1. here to get valid input so using unwrap directly. otherwise parse returns Result type.
    let temp: f64 = args[1].parse().unwrap();
    let to_farenheit: bool = args.contains(&"--to-farenheit".to_string());

    if to_farenheit {
        let result = celsius_2_faren(temp);
        println!("{}°C = {}°F", temp, result);
    } else {
        let result = faren_2_celsius(temp);
        println!("{}°F = {}°C", temp, result);
    }
}
