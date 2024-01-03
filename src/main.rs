use bpaf::Bpaf;
use std::fmt;

#[derive(Clone, Debug, Bpaf)]
#[bpaf(options, version)]

struct Args {
    /// The real number
    number: f64,
}

#[derive(Debug)]
struct Fraction {
    numerator: u64,
    denominator: u64,
    value: f64,
}

impl fmt::Display for Fraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let numerator_str = self.numerator.to_string();
        let denominator_str = self.denominator.to_string();

        let precision = 15;

        let max_fraction_length = std::cmp::max(numerator_str.len(), denominator_str.len());
        let separator = "-".repeat(max_fraction_length+2);
        let mut value_str = format!("{:0.precision$}", self.value).trim().to_string();

        // Trim trailing zeroes
        while value_str.ends_with('0') {
            value_str.pop();
        }
        // If the last character is a decimal point, remove it as well
        if value_str.ends_with('.') {
            value_str.pop();
        }

        let buf_len = value_str.len() + 4;
        let buf = " ".repeat(buf_len);

        write!(f, "\n{}{}\n{} ≈ {}\n{}{}\n",
            buf, numerator_str,
            value_str, separator,
            buf, denominator_str
        )
    }
}

fn farey(real_number: f64) -> Fraction {
    let mut left_numerator: f64 = real_number.floor();
    let mut left_denominator = 1.0;
    let mut right_numerator: f64 = real_number.ceil();
    let mut right_denominator = 1.0;

    let mut mediant_numerator: f64;
    let mut mediant_denominator: f64;
    let mut mediant: f64;
    let e = f64::EPSILON;
    loop {
        mediant_numerator = left_numerator + right_numerator;
        mediant_denominator = left_denominator + right_denominator;
        mediant = mediant_numerator / mediant_denominator;

        if mediant == real_number || (real_number - mediant).abs() < e {
            break;
        }

        if mediant > real_number {
            right_numerator = mediant_numerator;
            right_denominator = mediant_denominator;
        } else {
            left_numerator = mediant_numerator;
            left_denominator = mediant_denominator;
        }
    }

    return Fraction {
        numerator: mediant_numerator as u64,
        denominator: mediant_denominator as u64,
        value: mediant_numerator / mediant_denominator,
    }
}

fn main() {
    let opts = args().run();
    let approx = farey(opts.number);
    println!("{}", approx);
}
