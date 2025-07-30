pub enum OptionType {
    Call,
    Put
}

pub struct Option {
    pub option_type: OptionType,
    pub rfr: f64,
    pub strike: f64,
    pub spot: f64,
    pub iv: f64

}