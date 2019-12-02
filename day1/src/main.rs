use std::io::prelude::*;
fn main() {
    let stdin = std::io::stdin();
    let mut handle = stdin.lock();

    let mut fuel = 0;

    loop {
        let mut buf = String::new();
        let bytes_read = handle.read_line(&mut buf).expect("error reading stdin");
        if bytes_read == 0 {
            break;
        }

        let mass:i64 = buf.trim().parse().expect("expected valid number");
        fuel += calculate_fuel_required_including_fuel(mass);
    }

    println!("fuel required:{}", fuel);
}

fn calculate_fuel_required(mass:i64) -> i64 {
    mass / 3 - 2
}

fn calculate_fuel_required_including_fuel(mass: i64) -> i64 {
    let basic_fuel = calculate_fuel_required(mass);

    let mut total_fuel = basic_fuel;

    let mut additional_fuel = basic_fuel;
    loop {
        additional_fuel = calculate_fuel_required(additional_fuel);

        if additional_fuel > 0 {
            total_fuel += additional_fuel;
        } else {
            break;
        }
    }

    total_fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_calculate_fuel_required() {
        assert_eq!(2, calculate_fuel_required(12));
        assert_eq!(2, calculate_fuel_required(14));
        assert_eq!(654, calculate_fuel_required(1969));
        assert_eq!(33583, calculate_fuel_required(100756));
    }

    #[test]
    pub fn test_calculate_fuel_required_including_fuel() {
        assert_eq!(2, calculate_fuel_required_including_fuel(12));
        assert_eq!(966, calculate_fuel_required_including_fuel(1969));
        assert_eq!(50346, calculate_fuel_required_including_fuel(100756));
    }
}
