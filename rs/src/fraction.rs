use std::fmt;
use std::ops;


impl_op!(+ |a: &Fraction, b: &Fraction| -> Fraction { Fraction::new(a.numerator + b.numerator, a.denominator + b.denominator) });

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Fraction {
    pub numerator: u64,
    pub denominator: u64,
    pub value: f64,
}

impl Fraction {
    pub fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator: numerator,
            denominator: denominator,
            value: numerator as f64 / denominator as f64,
        }
    }
}

/// Format the fraction as follows:
///               27450985
/// 0.33333339 ≈ ----------
///               82352941
impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let numerator_str = self.numerator.to_string();
        let denominator_str = self.denominator.to_string();

        let precision = 15;

        let max_fraction_length = std::cmp::max(numerator_str.len(), denominator_str.len());
        let separator = "-".repeat(max_fraction_length+2);
        let mut value_str = format!("{:0.precision$}", self.value).trim().to_string();

        // get rid of trailing zeroes
        while value_str.ends_with('0') {
            value_str.pop();
        }
        if value_str.ends_with('.') {
            value_str.pop();
        }

        let buf_len = value_str.len() + 4;
        let buf = " ".repeat(buf_len);

        write!(f, "\n{}{}\n{} ≈ {}\n{}{}\n\n$ {} ≈ frac({},{}) $",
            buf, numerator_str,
            value_str, separator,
            buf, denominator_str,
            value_str,
            numerator_str, denominator_str
        )
    }
}
