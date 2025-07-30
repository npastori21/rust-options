// lib.rs
pub mod pricing;
pub mod greeks;


pub mod pricing {
    use statrs::distribution::Normal;
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
    use statrs::distribution::Normal;
    use statrs::distribution::ContinuousCDF;
    use statrs::distribution::Continuous;

    fn d1(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        ((s / k).ln() + (r + 0.5 * v * v) * t) / (v * t.sqrt())
    }

    fn d2(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        d1(s, k, t, r, v) - v * t.sqrt()
    }

    pub fn delta_call(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let d1 = ((s / k).ln() + (r + 0.5 * v * v) * t) / (v * t.sqrt());
        Normal::new(0.0, 1.0).unwrap().cdf(d1)
    }

    pub fn delta_put(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        delta_call(s, k, t, r, v) - 1.0
    }
    pub fn gamma(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
    let norm = Normal::new(0.0, 1.0).unwrap();
    norm.pdf(d1(s, k, t, r, v)) / (s * v * t.sqrt())
}

    pub fn vega(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let norm = Normal::new(0.0, 1.0).unwrap();
        s * norm.pdf(d1(s, k, t, r, v)) * t.sqrt() / 100.0 // per 1% change
    }

    pub fn theta_call(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let norm = Normal::new(0.0, 1.0).unwrap();
        let d1 = d1(s, k, t, r, v);
        let d2 = d2(s, k, t, r, v);
        let term1 = -s * norm.pdf(d1) * v / (2.0 * t.sqrt());
        let term2 = r * k * (-r * t).exp() * norm.cdf(d2);
        (term1 - term2) / 365.0
    }

    pub fn theta_put(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let norm = Normal::new(0.0, 1.0).unwrap();
        let d1 = d1(s, k, t, r, v);
        let d2 = d2(s, k, t, r, v);
        let term1 = -s * norm.pdf(d1) * v / (2.0 * t.sqrt());
        let term2 = r * k * (-r * t).exp() * norm.cdf(-d2);
        (term1 + term2) / 365.0
    }

    pub fn rho_call(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let norm = Normal::new(0.0, 1.0).unwrap();
        k * t * (-r * t).exp() * norm.cdf(d2(s, k, t, r, v)) / 100.0
    }

    pub fn rho_put(s: f64, k: f64, t: f64, r: f64, v: f64) -> f64 {
        let norm = Normal::new(0.0, 1.0).unwrap();
        -k * t * (-r * t).exp() * norm.cdf(-d2(s, k, t, r, v)) / 100.0
    }

}