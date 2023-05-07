// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        (d.seconds as f64 / 31557600f64) * Self::get_equal_years()
    }
    fn get_equal_years() -> f64;
}

pub struct Mercury {
    eq_earth_years: f64
}

pub struct Venus {
    eq_earth_years: f64
}
pub struct Earth {
    eq_earth_years: f64
}

pub struct Mars {
    eq_earth_years: f64
}

pub struct Jupiter {
    eq_earth_years: f64
}

pub struct Saturn {
    eq_earth_years: f64
}

pub struct Uranus {
    eq_earth_years: f64
}

pub struct Neptune {
    eq_earth_years: f64
}

impl Planet for Mercury {
    fn get_equal_years() -> f64 {
        0.2408467
    }
}
impl Planet for Venus {
    fn get_equal_years() -> f64 {
        0.61519726
    }
}
impl Planet for Earth {
    fn get_equal_years() -> f64 {
        1.0
    }
}
impl Planet for Mars {
    fn get_equal_years() -> f64 {
        1.8808158
    }
}
impl Planet for Jupiter {
    fn get_equal_years() -> f64 {
        11.862615
    }
}
impl Planet for Saturn {
    fn get_equal_years() -> f64 {
        29.447498
    }
}
impl Planet for Uranus {
    fn get_equal_years() -> f64 {
        84.016846
    }
}
impl Planet for Neptune {
    fn get_equal_years() -> f64 {
        164.79132
    }
}
