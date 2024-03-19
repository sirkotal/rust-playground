#[derive(Debug)]
pub struct Duration {
    days: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let days = (s / 60 / 60 / 24) as f64;
        return Duration{days};
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury {
    years: u64
}
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        let years = (d.days / 365.25) / 0.2408467;
        return (years * 100.0).round() / 100.0;
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        let years = (d.days / 365.25) / 0.61519726;
        return (years * 100.0).round() / 100.0;
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        let years = (d.days / 365.25);
        return (years * 100.0).round() / 100.0;
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        let years = (d.days / 365.25) / 1.8808158;
        return (years * 100.0).round() / 100.0;
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        let years = (d.days / 365.25) / 11.862615;
        return (years * 100.0).round() / 100.0;
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        let years = (d.days / 365.25) / 29.447498;
        return (years * 100.0).round() / 100.0;
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        let years = (d.days / 365.25) / 84.016846;
        return (years * 100.0).round() / 100.0;
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        let years = (d.days / 365.25) / 164.79132;
        return (years * 100.0).round() / 100.0;
    }
}
