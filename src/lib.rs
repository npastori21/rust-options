// lib.rs


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

pub mod strategy {
    /// Returns the payoff for a long call option
    pub fn long_call_payoff(s: f64, k: f64, premium: f64) -> f64 {
        (s - k).max(0.0) - premium
    }

    /// Returns the payoff for a short call option
    pub fn short_call_payoff(s: f64, k: f64, premium: f64) -> f64 {
        premium - (s - k).max(0.0)
    }

    /// Returns the payoff for a long put option
    pub fn long_put_payoff(s: f64, k: f64, premium: f64) -> f64 {
        (k - s).max(0.0) - premium
    }

    /// Returns the payoff for a short put option
    pub fn short_put_payoff(s: f64, k: f64, premium: f64) -> f64 {
        premium - (k - s).max(0.0)
    }

    /// Payoff for a long call spread (buy call at k1, sell call at k2)
    pub fn long_call_spread(s: f64, k1: f64, c1: f64, k2: f64, c2: f64) -> f64 {
        long_call_payoff(s, k1, c1) + short_call_payoff(s, k2, c2)
    }

    /// Payoff for a short call spread (sell call at k1, buy call at k2)
    pub fn short_call_spread(s: f64, k1: f64, c1: f64, k2: f64, c2: f64) -> f64 {
        short_call_payoff(s, k1, c1) + long_call_payoff(s, k2, c2)
    }

    /// Payoff for a long put spread (buy put at k1, sell put at k2)
    pub fn long_put_spread(s: f64, k1: f64, p1: f64, k2: f64, p2: f64) -> f64 {
        long_put_payoff(s, k1, p1) + short_put_payoff(s, k2, p2)
    }

    /// Payoff for a short put spread (sell put at k1, buy put at k2)
    pub fn short_put_spread(s: f64, k1: f64, p1: f64, k2: f64, p2: f64) -> f64 {
        short_put_payoff(s, k1, p1) + long_put_payoff(s, k2, p2)
    }

    /// Payoff for a long straddle (buy call and put at same strike)
    pub fn long_straddle(s: f64, k: f64, c: f64, p: f64) -> f64 {
        long_call_payoff(s, k, c) + long_put_payoff(s, k, p)
    }

    /// Payoff for a short straddle (sell call and put at same strike)
    pub fn short_straddle(s: f64, k: f64, c: f64, p: f64) -> f64 {
        short_call_payoff(s, k, c) + short_put_payoff(s, k, p)
    }

    /// Payoff for a long strangle (buy OTM call and OTM put)
    pub fn long_strangle(s: f64, kc: f64, c: f64, kp: f64, p: f64) -> f64 {
        long_call_payoff(s, kc, c) + long_put_payoff(s, kp, p)
    }

    /// Payoff for a short strangle (sell OTM call and OTM put)
    pub fn short_strangle(s: f64, kc: f64, c: f64, kp: f64, p: f64) -> f64 {
        short_call_payoff(s, kc, c) + short_put_payoff(s, kp, p)
    }

}