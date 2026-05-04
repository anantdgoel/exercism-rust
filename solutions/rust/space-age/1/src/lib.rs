// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    s: usize,
}

const EARTH_YEAR_IN_SECONDS: usize = 31_557_600;

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s: s as usize }
    }
}

pub trait Planet {
    fn orbital_year_in_seconds() -> f64 {
        EARTH_YEAR_IN_SECONDS as f64
    }

    fn years_during(d: &Duration) -> f64 {
        d.s as f64 / Self::orbital_year_in_seconds()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn orbital_year_in_seconds() -> f64 {
        0.2408467 * EARTH_YEAR_IN_SECONDS as f64
    }
}
impl Planet for Venus {
    fn orbital_year_in_seconds() -> f64 {
        0.61519726 * EARTH_YEAR_IN_SECONDS as f64
    }
}
impl Planet for Earth {}
impl Planet for Mars {
    fn orbital_year_in_seconds() -> f64 {
        1.8808158 * EARTH_YEAR_IN_SECONDS as f64
    }
}
impl Planet for Jupiter {
    fn orbital_year_in_seconds() -> f64 {
        11.862615 * EARTH_YEAR_IN_SECONDS as f64
    }
}
impl Planet for Saturn {
    fn orbital_year_in_seconds() -> f64 {
        29.447498 * EARTH_YEAR_IN_SECONDS as f64
    }
}
impl Planet for Uranus {
    fn orbital_year_in_seconds() -> f64 {
        84.016846 * EARTH_YEAR_IN_SECONDS as f64
    }
}
impl Planet for Neptune {
    fn orbital_year_in_seconds() -> f64 {
        164.79132 * EARTH_YEAR_IN_SECONDS as f64
    }
}
