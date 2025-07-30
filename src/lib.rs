// lib.rs
pub mod pricing;
pub mod greeks;
pub mod strategy;
pub mod api;


pub mod pricing {
    use statrs::distribution::{Normal, Univariate};
    use statrs::distribution::ContinuousCDF;

    pub fn black_scholes_call_price(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let d1 = ((s / k).ln() + (r + 0.5 * v * v) * t) / (v * t.sqrt());
        let d2 = d1 - v * t.sqrt();
        let norm = Normal::new(0.0, 1.0).unwrap();
        s * norm.cdf(d1) - k * (-r * t).exp() * norm.cdf(d2)
    }

    pub fn black_scholes_put_price(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let d1 = ((s / k).ln() + (r + 0.5 * v * v) * t) / (v * t.sqrt());
        let d2 = d1 - v * t.sqrt();
        let norm = Normal::new(0.0, 1.0).unwrap();
        k * (-r * t).exp() * norm.cdf(-d2) - s * norm.cdf(-d1)
    }
}


pub mod greeks {
    use statrs::distribution::{Normal, Univariate};
    use statrs::distribution::ContinuousCDF;

    pub fn delta_call(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let d1 = ((s / k).ln() + (r + 0.5 * v * v) * t) / (v * t.sqrt());
        Normal::new(0.0, 1.0).unwrap().cdf(d1)
    }

    pub fn delta_put(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        delta_call(s, k, t, r, v) - 1.0
    }
}