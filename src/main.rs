/// Date derived from work by Singal, Tanmay & Singal, Ashok. (2009). in
/// `Determining planetary positions in the sky for ~50 years to an accuracy of
/// 1 degree with a calculator.`
///
/// https://www.researchgate.net/publication/45878423_Determining_planetary_positions_in_the_sky_for_50_years_to_an_accuracy_of_1_degree_with_a_calculator
use std::result::Result;
use std::vec::Vec;

use structopt::StructOpt;

extern crate chrono;

use chrono::prelude::{DateTime, NaiveDate, Utc};

mod planet;
pub use planet::Planet;

#[derive(StructOpt)]
/// Determine the position of a planet in our solar system by date for 50 years
/// to an accuracy of 1 degree. If no date is provided, the current time (utc)
/// will used.
struct Cli {
    /// The epoch at which the planet's position will be determined. If now is
    /// selected the current date will be used
    #[structopt(short, long, default_value = "now")]
    date: String,

    /// Which planet to locate. If all is selected, every planet will be printed.
    #[structopt(short, long, default_value = "all")]
    planet: String,
}

fn main() {
    let args = Cli::from_args();

    let planet_name = args.planet;

    let date = parse_date_str(args.date).expect("Failed to parse provided date");

    if planet_name == "all" {
        let planets = get_planets();

        for planet in planets {
            print_planet_data(planet, date);
            println!("================");
        }
    } else {
        let planet = get_planet_by_name(planet_name).unwrap();

        print_planet_data(planet, date);
    }
}

/// Retrieve all planets within our solar system
fn get_planets() -> Vec<Planet> {
    let mut planets = Vec::new();

    planets.push(Planet::new("Mercury".to_string(), 250.2, 87.969));
    planets.push(Planet::new("Venus".to_string(), 181.2, 224.701));
    planets.push(Planet::new("Earth".to_string(), 100.0, 365.256));
    planets.push(Planet::new("Mars".to_string(), 355.2, 686.98));
    planets.push(Planet::new("Jupiter".to_string(), 34.3, 4332.59));
    planets.push(Planet::new("Saturn".to_string(), 50.1, 10759.2));
    planets.push(Planet::new("Uranus".to_string(), 313.23218, 30687.15));
    planets.push(Planet::new("Neptune".to_string(), 304.88003, 60190.03));

    planets
}

/// Retrieve a planet by it's name
fn get_planet_by_name(name: String) -> Result<Planet, &'static str> {
    for planet in get_planets() {
        if planet.name == name {
            return Ok(planet);
        }
    }

    Err("No planet found by that name.")
}

/// Attempts to parse the provided string into a `DateTime<Utc>` instance
fn parse_date_str(date_str: String) -> Result<DateTime<Utc>, &'static str> {
    if date_str == "now" {
        return Ok(Utc::now());
    }

    let naive_date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d");

    if naive_date.is_err() {
        return Err("Failed to parse provided date.");
    }

    let naive_datetime = naive_date.unwrap().and_hms(0, 0, 0);

    Ok(DateTime::from_utc(naive_datetime, Utc))
}

/// Prints a planet's data
///
/// Prints a planet's name, astronomical units, orbital period in years, and
/// position in degrees.
fn print_planet_data(planet: Planet, date: DateTime<Utc>) {
    let position = planet.get_position_at_date(date);

    println!("Planet:\t\t\t\t{0}", planet.name);
    println!("Astronomical Units:\t\t{0}", planet.get_au());
    println!("Years to orbit the sun:\t\t{0}", planet.get_years());
    println!("Planet {0} is located at:\t{1}", planet.name, position);
}
