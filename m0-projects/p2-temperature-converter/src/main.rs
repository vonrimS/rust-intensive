use std::io::{self, Write};
use p2_temperature_converter::{
    celsius_to_fahrenheit,
    farenheit_to_celsius,
    parse_temperature,
    ConversionMode
};

fn main() {
    println!("Select conversion mode:");
    println!("1. Celsius ---> Fahrenheit");
    println!("2. Fahrenheit ---> Celsius");
    println!("Choice (1 or 2): ");

    io::stdout().flush().unwrap();

    let mut mode_input = String::new();

    if io::stdin().read_line(&mut mode_input).is_err() {
        eprintln!("[ERROR] Failed to read mode from stdin.");
        return;
    }

    let mode = match ConversionMode::from_input(&mode_input) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("[ERROR] {}", err);
            return;
        }
    };

    match mode{
        ConversionMode::CelsiusToFahrenheit => print!("Enter temperature in Celsius: "),
        ConversionMode::FahrenheitToCelsius => print!("Enter temperature in Fahrenheit: "),
    }

    io::stdout().flush().unwrap();

    let mut temp_input = String::new();
    if io::stdin().read_line(&mut temp_input).is_err() {
        eprintln!("[ERROR] Failed to read temperature from stdin.");
        return;
    }

    match parse_temperature(&temp_input) {
        Ok(temp) => match mode {
            ConversionMode::CelsiusToFahrenheit => {
                let res = celsius_to_fahrenheit(temp);
                println!("{:.2} °C is equal to {:.2}°F", temp, res);
            }
            ConversionMode::FahrenheitToCelsius => {
                let res = farenheit_to_celsius(temp);
                println!("{:.2}°F is equal to {:.2}°C", temp, res);
            }
        },
        Err(err) => {
            eprintln!("[ERROR] {}", err);
        }
    }

}
