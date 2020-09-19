use std::io;
use std::io::prelude::*;
use std::io::Error;
use std::iter::FromIterator;
use std::str::FromStr;

fn main() {
    // Read mass input from stdin.
    let stdin = io::stdin();
    let locked = stdin.lock().lines();
    let masses = Masses::from_iter(locked);

    println!("{:?}", masses.0);
    
    // Calculate fuel based on masses.
    let mut total_fuel: u32 = 0;

    for mass in masses {
        let fuel_module = FuelModule::from_mass(mass);
        total_fuel += fuel_module.fuel;
    }

    // Return result.

    println!("Fuel needed: {}", total_fuel);
}

type Fuel = u32;
type Mass = u32;

struct Masses(Vec<Mass>);

impl Masses {
    fn new() -> Self {
        Masses(Vec::new())
    }

    fn add(&mut self, mass: Mass) {
        self.0.push(mass)
    }
}



impl FromIterator<Result<String, Error>> for Masses {
    fn from_iter<I: IntoIterator<Item = Result<String, Error>>>(iter: I) -> Self {
        let mut masses = Masses::new();

        for i in iter {
            match i {
                Ok(i) => {
                    // Empty line signals end of input.
                    if i.is_empty() {break;}

                    let mass = u32::from_str(&i).unwrap();
                    masses.add(mass);
                },
                Err(_) => println!("Fuck")
            }
        }
        masses
    }
}

impl IntoIterator for Masses {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

struct FuelModule {
    mass: Mass,
    fuel: Fuel,
}

impl FuelModule {
    fn from_mass(mass: Mass) -> Self {
        let fuel = mass / 3 - 2;

        Self {
            mass,
            fuel,
        }
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
        assert_eq!(fuel_module.fuel, 4);
    }
}
