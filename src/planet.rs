extern crate chrono;
use chrono::prelude::{DateTime, TimeZone, Utc};

/// The number of days lapsed before the Earth completes one orbit
const YRS: f64 = 365.256;

/// Represents a planet
pub struct Planet {
    /// The planet's name
    pub name: String,

    /// Location around the sun in degrees
    pub mean_longitude: f64,

    /// The planet's orbital period
    pub period: f64,

    /// The reference to compare the provided date to
    pub ref_date: DateTime<Utc>,
}

impl Planet {
    /// Constructs a new planet
    pub fn new(name: String, mean_longitude: f64, period: f64) -> Planet {
        Planet {
            name,
            mean_longitude,
            period,
            ref_date: Utc.ymd(2000, 1, 1).and_hms(0, 0, 0),
        }
    }

    /// Returns the planet's angular speed
    pub fn get_angular_speed(&self) -> f64 {
        360.0 / self.period
    }

    /// Returns the planet's distance from the sun in Astronomical Units
    pub fn get_au(&self) -> f64 {
        self.get_years().powf(2.0).cbrt()
    }

    /// Retrieves how many Earth years a planet will take to orbit the sun
    pub fn get_years(&self) -> f64 {
        self.period / YRS
    }

    /// Determines the planets position in degrees at a given date
    pub fn get_position_at_date(&self, date: DateTime<Utc>) -> f64 {
        let timestamp = date.timestamp();

        let ref_timestamp = self.ref_date.timestamp();

        let date_diff = (ref_timestamp - timestamp) / (24 * 60 * 60 * 1000);
        let date_diff_float = date_diff as f64;

        let angle_traversed = self.get_angular_speed() * date_diff_float;

        (self.mean_longitude * angle_traversed) % 360.0
    }
}
