use std::io::Read;
use std::fs::read_to_string;

fn main() {
    let module_mass: f32 = 14.0;
    let mut sum: f32 = 0.0;
        let mut fuel_mass = (module_mass / 3.0).floor() - 2.0;
        while fuel_mass > 0.0 {
            sum += fuel_mass;
            fuel_mass = (fuel_mass / 3.0).floor() - 2.0;
        }

    println!("{}", sum);

    let mut sum = 0.0;
    let input = read_to_string("input").unwrap();

    for module_mass in input.lines().map(|line| line.parse::<f32>().unwrap()) {
        let mut fuel_mass = (module_mass / 3.0).floor() - 2.0;
        while fuel_mass > 0.0 {
            sum += fuel_mass;
            fuel_mass = (fuel_mass / 3.0).floor() - 2.0;
        }
    }

    println!("{}", sum);
}
