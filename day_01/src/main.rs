use std::io;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    // Read mass input from stdin.
    let stdin = io::stdin();
    let lines = stdin.lock().lines();

    let result = lines
        .into_iter()
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .fold((0u32, 0u32), |(mut fuel_acc, mut mass_acc), line| {
            let mass = u32::from_str(&line.unwrap());
            let fuel_module = FuelModule::from_mass(mass.unwrap());

            fuel_acc += fuel_module.fuel;
            mass_acc += fuel_module.mass;

            println!("Fuel needed: {} for total mass: {}", fuel_acc, mass_acc);

            (fuel_acc, mass_acc)
        });

    println!("Fuel needed: {} for total mass: {}", result.0, result.1);
}

type Fuel = u32;
type Mass = u32;

struct FuelModule {
    mass: Mass,
    fuel: Fuel,
}

impl FuelModule {
    fn from_mass(mass: Mass) -> Self {
        let fuel = mass / 3 - 2;

        Self { mass, fuel }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_fuel_module_from_mass() {
        let mass = 12;
        let fuel_module = FuelModule::from_mass(mass);

        assert_eq!(fuel_module.mass, 12);
        assert_eq!(fuel_module.fuel, 2);
    }
}
