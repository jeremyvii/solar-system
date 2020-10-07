/// Date derived from work by Singal, Tanmay & Singal, Ashok. (2009). in
/// `Determining planetary positions in the sky for ~50 years to an accuracy of
/// 1 degree with a calculator.`
///
/// https://www.researchgate.net/publication/45878423_Determining_planetary_positions_in_the_sky_for_50_years_to_an_accuracy_of_1_degree_with_a_calculator

use std::collections::HashMap;

extern crate chrono;

use chrono::prelude::{DateTime, Utc, TimeZone};

const YRS: f64 = 365.256;

struct Planet {
  mean_longitude: f64,
  period: f64,
}

impl Planet {
  pub fn new(mean_longitude: f64, period: f64) -> Planet {
    Planet {
      mean_longitude: mean_longitude,
      period: period,
    }
  }

  pub fn get_angular_speed(&self) -> f64 {
    return 360.0 / self.period;
  }

  pub fn get_au(&self) -> f64 {
    return self.get_years().powf(2.0).cbrt();
  }

  pub fn get_years(&self) -> f64 {
    return self.period / YRS;
  }

  /// Determines the planets position at a given date
  pub fn get_position_at_date(&self, date: DateTime<Utc>) -> f64 {
    let timestamp = date.timestamp();

    let ref_timestamp = Utc.ymd(2000, 1, 1).and_hms(0, 0, 0).timestamp();

    let date_diff = (ref_timestamp - timestamp) / (24 * 60 * 60 * 1000);
    let date_diff_float = date_diff as f64;

    let angle_traversed = self.get_angular_speed() * date_diff_float;

    return (self.mean_longitude * angle_traversed) % 360.0;
  }
}

fn main() {
  let mut planets = HashMap::new();

  planets.insert("Mercury", Planet::new(250.2, 87.969));
  planets.insert("Venus",   Planet::new(181.2, 224.701));
  planets.insert("Earth",   Planet::new(100.0, 365.256));
  planets.insert("Mars",    Planet::new(355.2, 686.98));
  planets.insert("Jupiter", Planet::new(34.3,  4332.59));
  planets.insert("Saturn",  Planet::new(50.1,  10759.2));

  for planet_map in planets.into_iter() {
    let name = planet_map.0;
    let planet = planet_map.1;

    let date: DateTime<Utc> = Utc::now();

    let position = planet.get_position_at_date(date);

    println!("Planet: {0}", name);
    println!("Astronomical Units: {0}", planet.get_au());
    println!("Years to orbit the sun: {0}", planet.get_years());
    println!("Planet {0} is at {1}", name, position);
    println!("================");
  }
}
