mod options;
mod pricing;

use options::{Option, OptionType};
use pricing::black_scholes_call_price;


fn main() {

    let option = Option {
        option_type: OptionType::Call,
        rfr: 0.05,
        strike: 100.0,
        spot: 105.0,
        iv: 0.30
    };

    let price = black_scholes_call_price(option.spot, option.strike, 1.0/12.0, option.rfr, option.iv);
    println!("Call price {:.2}", price);
}
