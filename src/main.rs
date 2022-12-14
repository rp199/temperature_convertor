use std::io;

const CELSIUS_LABEL: &str = "°C";
const FAHRENHEIT_LABEL: &str = "°F";

fn main() {
    println!("Temperature convertor - Celsius <-> Fahrenheit");


    loop {
        let input_temperature_unit = read_temperature_unit();

        let (from_unit, to_unit, convertor) = match input_temperature_unit.trim() {
            "c" | "celsius" => {
                let convertor = convert_celsius_to_fahrenheit;
                (CELSIUS_LABEL, FAHRENHEIT_LABEL, convertor)
            }
            "f" | "fahrenheit" => {
                let convertor = convert_celsius_to_fahrenheit;
                (FAHRENHEIT_LABEL, CELSIUS_LABEL, convertor)
            }
            _ => {
                println!("Invalid temperature.");
                continue;
            }
        };

        let from_value = read_temperature_value();
        let to_value = convertor(from_value);

        println!("Result: {}{} = {}{}", from_value, from_unit, to_value, to_unit);
    }
}

fn read_temperature_unit() -> String {
    println!("Select input temperature unit (c - celsius/f - fahrenheit):");
    let mut input_temperature_unit = String::new();

    io::stdin()
        .read_line(&mut input_temperature_unit)
        .expect("Failed to read temperature unit");

    input_temperature_unit.to_lowercase()
}

fn read_temperature_value() -> f64 {
    loop {
        println!("Enter temperature:");

        let mut temperature_value = String::new();

        io::stdin()
            .read_line(&mut temperature_value)
            .expect("Failed to read line");

        match temperature_value.trim().parse() {
            Ok(num) => return num,
            Err(_) => {
                println!("Invalid temperature.");
                continue;
            }
        };
    }
}

fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}