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
            let mass = u32::from_str(&line.unwrap()).unwrap();
            let fuel_module = FuelModule::from_mass(mass);

            fuel_acc += fuel_module.fuel;
            mass_acc += fuel_module.mass;

            println!("Module of mass {:?}:", mass);
            println!("  - fuel needed: {}", fuel_module.fuel);
            println!("  - total mass including fuel:{}", fuel_module.mass);

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
        // Overflow is true if calculation results in a negative fuel value.
        let (fuel, is_neg) = (mass / 3).overflowing_sub(2);

        // Calculate fuel for fuel if overflow is false (recursive).
        let (mass, fuel) = match is_neg {
            false => {
                let meta = FuelModule::from_mass(fuel);
                (mass + meta.mass, fuel + meta.fuel)
            }
            true => (mass, 0),
        };
        // Mass is module mass + fuel mass fuel is fuel for module and fuel for fuel.
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

        assert_eq!(fuel_module.mass, 14);
        assert_eq!(fuel_module.fuel, 2);
    }

    #[test]
    fn fuel_module_handles_negatives() {
        let mass = 5;
        let fuel_module = FuelModule::from_mass(mass);

        assert_eq!(fuel_module.mass, 5);
        assert_eq!(fuel_module.fuel, 0);
    }

    #[test]
    fn fuel_for_fuel() {
        let mass = 1969;
        let fuel_module = FuelModule::from_mass(mass);

        assert_eq!(fuel_module.mass, 2935);
        assert_eq!(fuel_module.fuel, 966);
    }
}
